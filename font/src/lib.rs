extern crate byteorder;
extern crate fixed;

mod filetype;
mod font;
mod opentype;
pub mod sfnt;
mod truetype;

pub use font::{Font, FontParseErr};
// TODO: Have a generic version of Name which maps to format-specific formats as needed.
pub use opentype::tables::name::Name;
