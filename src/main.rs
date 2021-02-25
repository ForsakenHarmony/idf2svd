use std::str::FromStr;

use clap::{app_from_crate, Arg};
use common::ChipType;
use std::fs::File;
use std::io::BufWriter;
use svd_parser::encode::Encode;
use xmltree::EmitterConfig;

mod common;
mod idf;
mod sdk;

fn main() {
    let matches = app_from_crate!("\n")
        .arg(
            Arg::with_name("CHIP")
                .help("which device's SVD to generate")
                .required(true)
                .index(1)
                .possible_values(&["ESP32", "ESP8266", "ESP32C3"])
                .case_insensitive(true),
        )
        .get_matches();

    // Based on which chip has been selected, invoke the appropriate SVD
    // builder (since the ESP32 and ESP8266 have different SDKs).
    let chip = matches.value_of("CHIP").unwrap().to_uppercase();
    let svd = match ChipType::from_str(&chip) {
        Ok(chip) => match chip {
            ChipType::ESP32 => idf::create_svd(chip),
            ChipType::ESP32C3 => idf::create_svd(chip),
            ChipType::ESP8266 => sdk::create_svd(),
        },
        Err(e) => return println!("{}", e),
    };

    let filename = format!("{}.svd", chip.to_lowercase());
    let f = BufWriter::new(File::create(filename).unwrap());
    svd.encode()
        .unwrap()
        .write_with_config(
            f,
            EmitterConfig::new()
                .perform_indent(true)
                .indent_string("    "),
        )
        .unwrap();
}
