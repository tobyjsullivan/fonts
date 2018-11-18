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

    match Font::from_bytes(&data) {
        Ok(parsed) => {
            let fields = parsed.available_strings();
            for (f, value) in fields {
                if value == "" {
                    continue;
                }
                println!("{:<25} {}", field_label(f).unwrap_or(&format!("{:?}", f)), value);
            }
        }
        Err(error) => {
            println!("Failed to parse: {:?}", error);
        }
    }
}

fn field_label(field: Name) -> Option<&'static str> {
    match field {
        Name::CopyrightNotice => Some("Copyright Notice"),
        Name::FontFamilyName => Some("Family Name"),
        Name::FontSubfamilyName => Some("Subfamily Name"),
        Name::UniqueFontID => Some("Unique ID"),
        Name::FullFontName => Some("Full Name"),
        Name::VersionString => Some("Version"),
        Name::PostScriptName => Some("PostScript Name"),
        Name::Trademark => Some("Trademark"),
        Name::Manufacturer => Some("Manufacturer"),
        Name::Designer => Some("Designer"),
        Name::Description => Some("Description"),
        Name::VendorUrl => Some("Vendor Url"),
        Name::DesignerUrl => Some("Designer Url"),
        Name::License => Some("License"),
        Name::LicenseInfoUrl => Some("License Info Url"),
        Name::TypographicFamilyName => Some("Typographic Family Name"),
        Name::TypographicSubfamilyName => Some("Typographic Subfamily Name"),
        Name::CompatibleFullName => Some("Compatible Full Name"),
        Name::SampleText => Some("Sample Text"),
        Name::PostScriptCIDFindFontName => Some("PostScript CID Find Font Name"),
        Name::WWSFamilyName => Some("WWS Family Name"),
        Name::WWSSubfamilyName => Some("WWS Subfamily Name"),
        Name::LightBackgroundPalette => Some("Light Background Palette"),
        Name::DarkBackgroundPalette => Some("Dark Background Palette"),
        Name::VariationsPostScriptNamePrefix => Some("Variations PostScript Name Prefix"),
    }
}
