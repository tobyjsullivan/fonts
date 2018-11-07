extern crate font;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use font::Font;

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
            // TODO: List copyright info.
            parsed.read_copyright().map(|copyright| {
                println!("Copyright:\t{}", copyright);
            });
        }
        Err(error) => {
            println!("Failed to parse: {:?}", error);
        }
    }
}
