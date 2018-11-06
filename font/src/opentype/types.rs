use byteorder::{BigEndian, ByteOrder};
use fixed::{frac, FixedI32};

pub trait DataType<T> {
    fn extract(data: &[u8], offset: usize) -> T;
}

pub enum U8 {}

impl DataType<u8> for U8 {
    fn extract(data: &[u8], offset: usize) -> u8 {
        data[offset]
    }
}

pub enum U16 {}

impl DataType<u16> for U16 {
    fn extract(data: &[u8], offset: usize) -> u16 {
        BigEndian::read_u16(&data[offset..offset + 2])
    }
}

pub enum U32 {}

impl DataType<u32> for U32 {
    fn extract(data: &[u8], offset: usize) -> u32 {
        BigEndian::read_u32(&data[offset..offset + 4])
    }
}

pub type I16 = i16;

impl DataType<I16> for I16 {
    fn extract(data: &[u8], offset: usize) -> I16 {
        BigEndian::read_i16(&data[offset..offset + 2])
    }
}

pub enum I32 {}

impl DataType<i32> for I32 {
    fn extract(data: &[u8], offset: usize) -> i32 {
        BigEndian::read_i32(&data[offset..offset + 4])
    }
}

pub type Datetime = i64;

impl DataType<Datetime> for Datetime {
    fn extract(data: &[u8], offset: usize) -> Datetime {
        BigEndian::read_i64(&data[offset..offset + 8])
    }
}

pub type Fixed = FixedI32<frac::U16>;

impl DataType<Fixed> for Fixed {
    fn extract(data: &[u8], offset: usize) -> Fixed {
        let i = I32::extract(data, offset);
        FixedI32::<frac::U16>::from_bits(i)
    }
}

pub type Offset = usize;

pub enum Offset16 {}

impl DataType<Offset> for Offset16 {
    fn extract(data: &[u8], offset: usize) -> Offset {
        U16::extract(data, offset) as usize
    }
}

pub enum Offset32 {}

impl DataType<Offset> for Offset32 {
    fn extract(data: &[u8], offset: usize) -> Offset {
        U32::extract(data, offset) as usize
    }
}
