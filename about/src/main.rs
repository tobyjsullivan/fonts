#[macro_use]
extern crate clap;
extern crate font;

use std::fs::File;
use std::io::Read;

use font::{Font, Name};

fn main() {
    let matches = clap_app!(about =>
        (about: "Dumps strings stored in font files.")
        (@arg COPYRIGHT: -C --copyright "Print Copyright")
        (@arg FONT_FAMILY: -F --family "Print Font Family Name")
        (@arg FONT_SUBFAMILY: -f --subfamily "Print Font Subfamily Name")
        (@arg UNIQUE_ID: -U --id "Print Unique ID")
        (@arg FULL_NAME: -N --name "Print Full Font Name")
        (@arg VERSION: -V --version "Print Version String")
        (@arg PS_NAME: -P --psname "Print PostScript Name")
        (@arg TRADEMARK: -R --trademark "Print Trademark")
        (@arg MANUFACTURER: -M --manufacturer "Print Manufacturer")
        (@arg DESIGNER: -A --designer "Print Designer")
        (@arg DESCRIPTION: -D --description "Print Description")
        (@arg VENDOR: -v --vendor "Print Vendor URL")
        (@arg DESIGNER_URL: -u --designerurl "Print Designer URL")
        (@arg LICENSE: -L --license "Print License")
        (@arg LICENSE_URL: -i --licenseurl "Print License Info URL")
        (@arg TYPOGRAPHIC_FAMILY: -T --typefam "Print Typographic Family Name")
        (@arg TYPOGRAPHIC_SUBFAMILY: -t --typesubfam "Print Typographic Sumfamily Name")
        (@arg COMPATIBLE_NAME: -n --compatible "Print Compatible Full Name")
        (@arg SAMPLE_TEXT: -s --sampletext "Print Sample Text")
        (@arg PS_CID: -c --pscid "Print PostScript CID Find Font Name")
        (@arg WWS_FAMILY: -W --wwsfam "Print WWS Family Name")
        (@arg WWS_SUBFAMILY: -w --wwssubfam "Print WWS Subfamily Name")
        (@arg LIGHT_PALETTE: -l --light "Print Light Background Palette")
        (@arg DARK_PALETTE: -d --dark "Print Dark Background Palette")
        (@arg PS_VARS: -p --psvar "Print Variations PostScript Name Prefix")
        (@arg INPUT: +required "Sets the input file to use")
    ).get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let mut f = File::open(filename).expect("file not found.");
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).unwrap();

    let mut selectors: Vec<Name> = Vec::new();
    if matches.is_present("COPYRIGHT") {
        selectors.push(Name::CopyrightNotice);
    }
    if matches.is_present("FONT_FAMILY") {
        selectors.push(Name::FontFamilyName);
    }
    if matches.is_present("FONT_SUBFAMILY") {
        selectors.push(Name::FontSubfamilyName);
    }
    if matches.is_present("UNIQUE_ID") {
        selectors.push(Name::UniqueFontID);
    }
    if matches.is_present("FULL_NAME") {
        selectors.push(Name::FullFontName);
    }
    if matches.is_present("VERSION") {
        selectors.push(Name::VersionString);
    }
    if matches.is_present("PS_NAME") {
        selectors.push(Name::PostScriptName);
    }
    if matches.is_present("TRADEMARK") {
        selectors.push(Name::Trademark);
    }
    if matches.is_present("MANUFACTURER") {
        selectors.push(Name::Manufacturer);
    }
    if matches.is_present("DESIGNER") {
        selectors.push(Name::Designer);
    }
    if matches.is_present("DESCRIPTION") {
        selectors.push(Name::Description);
    }
    if matches.is_present("VENDOR") {
        selectors.push(Name::VendorUrl);
    }
    if matches.is_present("DESIGNER_URL") {
        selectors.push(Name::DesignerUrl);
    }
    if matches.is_present("LICENSE") {
        selectors.push(Name::License);
    }
    if matches.is_present("LICENSE_URL") {
        selectors.push(Name::LicenseInfoUrl);
    }
    if matches.is_present("TYPOGRAPHIC_FAMILY") {
        selectors.push(Name::TypographicFamilyName);
    }
    if matches.is_present("TYPOGRAPHIC_SUBFAMILY") {
        selectors.push(Name::TypographicSubfamilyName);
    }
    if matches.is_present("COMPATIBLE_NAME") {
        selectors.push(Name::CompatibleFullName);
    }
    if matches.is_present("SAMPLE_TEXT") {
        selectors.push(Name::SampleText);
    }
    if matches.is_present("PS_CID") {
        selectors.push(Name::PostScriptCIDFindFontName);
    }
    if matches.is_present("WWS_FAMILY") {
        selectors.push(Name::WWSFamilyName);
    }
    if matches.is_present("WWS_SUBFAMILY") {
        selectors.push(Name::WWSSubfamilyName);
    }
    if matches.is_present("LIGHT_PALETTE") {
        selectors.push(Name::LightBackgroundPalette);
    }
    if matches.is_present("DARK_PALETTE") {
        selectors.push(Name::DarkBackgroundPalette);
    }
    if matches.is_present("PS_VARS") {
        selectors.push(Name::VariationsPostScriptNamePrefix);
    }

    match Font::from_bytes(&data) {
        Ok(parsed) => {
            if selectors.is_empty() {
                let fields = parsed.available_strings();
                for (f, value) in fields {
                    if value == "" {
                        continue;
                    }
                    println!(
                        "{:<25} {}",
                        field_label(f).unwrap_or(&format!("{:?}", f)),
                        value
                    );
                }
            } else {
                for f in selectors {
                    if let Some(value) = parsed.read_unicode_string(f) {
                        println!(
                            "{:<25} {}",
                            field_label(f).unwrap_or(&format!("{:?}", f)),
                            value
                        );
                    }
                }
            }
        }
        Err(error) => {
            println!("Failed to parse: {:?}", error);
        }
    }
}

fn field_label(field: Name) -> Option<&'static str> {
    match field {
        Name::CopyrightNotice => Some("Copyright Notice"), // C
        Name::FontFamilyName => Some("Family Name"),  // F
        Name::FontSubfamilyName => Some("Subfamily Name"), // f
        Name::UniqueFontID => Some("Unique ID"), // I
        Name::FullFontName => Some("Full Name"), // N
        Name::VersionString => Some("Version"), // V
        Name::PostScriptName => Some("PostScript Name"), // P
        Name::Trademark => Some("Trademark"), // R
        Name::Manufacturer => Some("Manufacturer"), // M
        Name::Designer => Some("Designer"), // A
        Name::Description => Some("Description"), // D
        Name::VendorUrl => Some("Vendor Url"), // v
        Name::DesignerUrl => Some("Designer Url"), // u
        Name::License => Some("License"), // L
        Name::LicenseInfoUrl => Some("License Info Url"), // l
        Name::TypographicFamilyName => Some("Typographic Family Name"), // T
        Name::TypographicSubfamilyName => Some("Typographic Subfamily Name"), // t
        Name::CompatibleFullName => Some("Compatible Full Name"), // n
        Name::SampleText => Some("Sample Text"), // s
        Name::PostScriptCIDFindFontName => Some("PostScript CID Find Font Name"), // c
        Name::WWSFamilyName => Some("WWS Family Name"), // W
        Name::WWSSubfamilyName => Some("WWS Subfamily Name"), // w
        Name::LightBackgroundPalette => Some("Light Background Palette"), // l
        Name::DarkBackgroundPalette => Some("Dark Background Palette"), // d
        Name::VariationsPostScriptNamePrefix => Some("Variations PostScript Name Prefix"), // p
    }
}
