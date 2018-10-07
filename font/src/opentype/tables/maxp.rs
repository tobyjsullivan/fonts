use fixed::{frac, FixedI32};

use opentype::types::{DataType, I32, U16};

const OFFSET_VERSION: usize = 0;
const OFFSET_NUM_GLYPHS: usize = 4;
const OFFSET_MAX_POINTS: usize = 6;
const OFFSET_MAX_CONTOURS: usize = 8;
const OFFSET_MAX_COMPOSITE_POINTS: usize = 10;
const OFFSET_MAX_COMPOSITE_CONTOURS: usize = 12;
const OFFSET_MAX_ZONES: usize = 14;
const OFFSET_MAX_TWILIGHT_POINTS: usize = 16;
const OFFSET_MAX_STORAGE: usize = 18;
const OFFSET_MAX_FUNC_DEFS: usize = 20;
const OFFSET_MAX_INSTRUCTION_DEFS: usize = 22;
const OFFSET_MAX_STACK_ELEMENTS: usize = 24;
const OFFSET_MAX_SIZE_OF_INSTRUCTIONS: usize = 26;
const OFFSET_MAX_COMPONENT_ELEMENTS: usize = 28;
const OFFSET_MAX_COMPONENT_DEPTH: usize = 30;

type Fixed = FixedI32<frac::U16>;

#[derive(Debug)]
pub struct MaxpTable {
    version: Version,
    num_glyphs: u16,
    max_points: Option<u16>,
    max_contours: Option<u16>,
    max_comp_pts: Option<u16>,
    max_comp_contours: Option<u16>,
    max_zones: Option<u16>,
    max_twilight_points: Option<u16>,
    max_storage: Option<u16>,
    max_func_defs: Option<u16>,
    max_instruction_defs: Option<u16>,
    max_stack_elements: Option<u16>,
    max_size_of_instructions: Option<u16>,
    max_comp_elements: Option<u16>,
    max_comp_depth: Option<u16>,
}

impl MaxpTable {
    pub fn parse(table_data: &[u8]) -> Self {
        let version = Self::parse_version(table_data);

        let mut max_points = None;
        let mut max_contours = None;
        let mut max_comp_pts = None;
        let mut max_comp_contours = None;
        let mut max_zones = None;
        let mut max_twilight_points = None;
        let mut max_storage = None;
        let mut max_func_defs = None;
        let mut max_instruction_defs = None;
        let mut max_stack_elements = None;
        let mut max_size_of_instructions = None;
        let mut max_comp_elements = None;
        let mut max_comp_depth = None;
        if version == Version::V1_0 {
            max_points = Some(U16::extract(table_data, OFFSET_MAX_POINTS));
            max_contours = Some(U16::extract(table_data, OFFSET_MAX_CONTOURS));
            max_comp_pts = Some(U16::extract(table_data, OFFSET_MAX_COMPOSITE_POINTS));
            max_comp_contours = Some(U16::extract(table_data, OFFSET_MAX_COMPOSITE_CONTOURS));
            max_zones = Some(U16::extract(table_data, OFFSET_MAX_ZONES));
            max_twilight_points = Some(U16::extract(table_data, OFFSET_MAX_TWILIGHT_POINTS));
            max_storage = Some(U16::extract(table_data, OFFSET_MAX_STORAGE));
            max_func_defs = Some(U16::extract(table_data, OFFSET_MAX_FUNC_DEFS));
            max_instruction_defs = Some(U16::extract(table_data, OFFSET_MAX_INSTRUCTION_DEFS));
            max_stack_elements = Some(U16::extract(table_data, OFFSET_MAX_STACK_ELEMENTS));
            max_size_of_instructions =
                Some(U16::extract(table_data, OFFSET_MAX_SIZE_OF_INSTRUCTIONS));
            max_comp_elements = Some(U16::extract(table_data, OFFSET_MAX_COMPONENT_ELEMENTS));
            max_comp_depth = Some(U16::extract(table_data, OFFSET_MAX_COMPONENT_DEPTH));
        }

        Self {
            version,
            num_glyphs: U16::extract(table_data, OFFSET_NUM_GLYPHS),
            max_points,
            max_contours,
            max_comp_pts,
            max_comp_contours,
            max_zones,
            max_twilight_points,
            max_storage,
            max_func_defs,
            max_instruction_defs,
            max_stack_elements,
            max_size_of_instructions,
            max_comp_elements,
            max_comp_depth,
        }
    }

    fn parse_version(table_data: &[u8]) -> Version {
        let value = I32::extract(table_data, OFFSET_VERSION);
        match value {
            0x00005000 => Version::V0_5,
            0x00010000 => Version::V1_0,
            _ => Version::Unknown(Fixed::from_bits(value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V0_5,
    V1_0,
    Unknown(Fixed),
}
