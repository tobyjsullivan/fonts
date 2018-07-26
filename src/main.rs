extern crate font;

use std::env;
use std::fs::File;
use std::io::Read;

use font::Font;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply font file.");
    }

    let filename = &args[1];
    println!("Loading file: {}", filename);
    let mut f = File::open(filename).expect("file not found.");
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).unwrap();

    println!("File read in. {} bytes.", data.len());

    let parsed = Font::from(&data);
    println!("Parsed: {:?}", parsed);
}
