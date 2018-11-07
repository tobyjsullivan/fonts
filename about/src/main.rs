extern crate font;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use font::{Font, Name};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ERROR: You must supply a font file.");
        process::exit(1);
    }

    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found.");
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).unwrap();

    match Font::from(&data) {
        Ok(parsed) => {
            print_entry(&parsed, Name::FontFamilyName, "Font Family Name");
            print_entry(&parsed, Name::CopyrightNotice, "Copyright Notice");
        }
        Err(error) => {
            println!("Failed to parse: {:?}", error);
        }
    }
}

fn print_entry(font: &Font, field: Name, label: &str) {
    font.read_unicode_string(field).map(|copyright| {
        println!("{}:\t{}", label, copyright);
    });
}
