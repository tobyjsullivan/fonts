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
            print_entry(&parsed, Name::UniqueFontID, "Unique ID");
            print_entry(&parsed, Name::FullFontName, "Full Name");
            print_entry(&parsed, Name::FontFamilyName, "Family Name");
            print_entry(&parsed, Name::FontSubfamilyName, "Subfamily Name");
            print_entry(&parsed, Name::CompatibleFullName, "Compatible Full Name");
            print_entry(&parsed, Name::PostScriptName, "PostScript Name");
            print_entry(
                &parsed,
                Name::PostScriptCIDFindFontName,
                "PostScript CID Find Font Name",
            );
            print_entry(
                &parsed,
                Name::VariationsPostScriptNamePrefix,
                "Variations PostScript Name Prefix",
            );
            print_entry(
                &parsed,
                Name::TypographicFamilyName,
                "Typographic Family Name",
            );
            print_entry(
                &parsed,
                Name::TypographicSubfamilyName,
                "Typographic Subfamily Name",
            );
            print_entry(&parsed, Name::WWSFamilyName, "WWS Family Name");
            print_entry(&parsed, Name::WWSSubfamilyName, "WWS Subfamily Name");
            print_entry(&parsed, Name::VersionString, "Version");
            print_entry(&parsed, Name::CopyrightNotice, "Copyright Notice");
            print_entry(&parsed, Name::Trademark, "Trademark");
            print_entry(&parsed, Name::Manufacturer, "Manufacturer");
            print_entry(&parsed, Name::Designer, "Designer");
            print_entry(&parsed, Name::DesignerUrl, "Designer Url");
            print_entry(&parsed, Name::Description, "Description");
            print_entry(&parsed, Name::VendorUrl, "Vendor Url");
            print_entry(&parsed, Name::License, "License");
            print_entry(&parsed, Name::LicenseInfoUrl, "License Info Url");
            print_entry(&parsed, Name::SampleText, "Sample Text");
            print_entry(
                &parsed,
                Name::LightBackgroundPalette,
                "Light Background Palette",
            );
            print_entry(
                &parsed,
                Name::DarkBackgroundPalette,
                "Dark Background Palette",
            );
        }
        Err(error) => {
            println!("Failed to parse: {:?}", error);
        }
    }
}

fn print_entry(font: &Font, field: Name, label: &str) {
    font.read_unicode_string(field).map(|copyright| {
        println!("{:<25} {}", label, copyright);
    });
}
