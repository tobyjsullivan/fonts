extern crate byteorder;
extern crate fixed;

mod filetype;
mod font;
mod opentype;
pub mod sfnt;
mod truetype;

pub use font::{Font, FontParseErr};
