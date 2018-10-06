use byteorder::{BigEndian, ByteOrder};
use fixed::{frac::U16, FixedI32};

const OFFSET_VERSION: usize = 0;
const OFFSET_NUM_GLYPHS: usize = 4;
const OFFSET_MAX_POINTS: usize = 6;
const OFFSET_MAX_CONTOURS: usize = 8;
const OFFSET_MAX_COMPOSITE_POINTS: usize = 10;
const OFFSET_MAX_COMPOSITE_CONTOURS: usize = 12;
const OFFSET_MAX_ZONES: usize = 14;
const OFFSET_MAX_TWILIGHT_POINST: usize = 16;
const OFFSET_MAX_STORAGE: usize = 18;
const OFFSET_MAX_FUNC_DEFS: usize = 20;
const OFFSET_MAX_INSTRUCTION_DEFS: usize = 22;
const OFFSET_MAX_STACK_ELEMENTS: usize = 24;
const OFFSET_MAX_SIZE_OF_INSTRUCTIONS: usize = 26;
const OFFSET_MAX_COMPONENT_ELEMENTS: usize = 28;
const OFFSET_MAX_COMPONENT_DEPTH: usize = 30;

type Fixed = FixedI32<U16>;

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
            max_points = Some(Self::parse_max_points(table_data));
            max_contours = Some(Self::parse_max_contours(table_data));
            max_comp_pts = Some(Self::parse_max_comp_pts(table_data));
            max_comp_contours = Some(Self::parse_max_comp_contours(table_data));
            max_zones = Some(Self::parse_max_zones(table_data));
            max_twilight_points = Some(Self::parse_max_twilight_points(table_data));
            max_storage = Some(Self::parse_max_storage(table_data));
            max_func_defs = Some(Self::parse_max_func_defs(table_data));
            max_instruction_defs = Some(Self::parse_max_instruction_defs(table_data));
            max_stack_elements = Some(Self::parse_max_stack_elements(table_data));
            max_size_of_instructions = Some(Self::parse_max_size_of_instructions(table_data));
            max_comp_elements = Some(Self::parse_max_comp_elements(table_data));
            max_comp_depth = Some(Self::parse_max_comp_depth(table_data));
        }

        Self {
            version,
            num_glyphs: Self::parse_num_glyphs(table_data),
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
        let value = BigEndian::read_i32(&table_data[OFFSET_VERSION..OFFSET_VERSION + 4]);
        match value {
            0x00005000 => Version::V0_5,
            0x00010000 => Version::V1_0,
            _ => Version::Unknown(Fixed::from_bits(value)),
        }
    }

    fn parse_num_glyphs(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_NUM_GLYPHS..OFFSET_NUM_GLYPHS + 2])
    }

    fn parse_max_points(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_POINTS..OFFSET_MAX_POINTS + 2])
    }

    fn parse_max_contours(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_CONTOURS..OFFSET_MAX_CONTOURS + 2])
    }

    fn parse_max_comp_pts(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &table_data[OFFSET_MAX_COMPOSITE_POINTS..OFFSET_MAX_COMPOSITE_POINTS + 2],
        )
    }

    fn parse_max_comp_contours(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &table_data[OFFSET_MAX_COMPOSITE_CONTOURS..OFFSET_MAX_COMPOSITE_CONTOURS + 2],
        )
    }

    fn parse_max_zones(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_ZONES..OFFSET_MAX_ZONES + 2])
    }

    fn parse_max_twilight_points(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_TWILIGHT_POINST..OFFSET_MAX_TWILIGHT_POINST + 2])
    }

    fn parse_max_storage(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_STORAGE..OFFSET_MAX_STORAGE + 2])
    }

    fn parse_max_func_defs(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_FUNC_DEFS..OFFSET_MAX_FUNC_DEFS + 2])
    }

    fn parse_max_instruction_defs(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &table_data[OFFSET_MAX_INSTRUCTION_DEFS..OFFSET_MAX_INSTRUCTION_DEFS + 2],
        )
    }

    fn parse_max_stack_elements(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_STACK_ELEMENTS..OFFSET_MAX_STACK_ELEMENTS + 2])
    }

    fn parse_max_size_of_instructions(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &table_data[OFFSET_MAX_SIZE_OF_INSTRUCTIONS..OFFSET_MAX_SIZE_OF_INSTRUCTIONS + 2],
        )
    }

    fn parse_max_comp_elements(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &table_data[OFFSET_MAX_COMPONENT_ELEMENTS..OFFSET_MAX_COMPONENT_ELEMENTS + 2],
        )
    }

    fn parse_max_comp_depth(table_data: &[u8]) -> u16 {
        BigEndian::read_u16(&table_data[OFFSET_MAX_COMPONENT_DEPTH..OFFSET_MAX_COMPONENT_DEPTH + 2])
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V0_5,
    V1_0,
    Unknown(Fixed),
}
