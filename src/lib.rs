#![feature(const_fn)]
#[macro_use]
extern crate bfd_macro;

pub trait ByteOrder<'a> {
    type Bytes: core::convert::TryFrom<&'a [u8], Error = TryFromSliceError>;
    fn to_ne_bytes(&self) -> Self::Bytes;
    fn to_le_bytes(&self) -> Self::Bytes;
    fn to_be_bytes(&self) -> Self::Bytes;
    fn from_ne_bytes(x: &[u8]) -> Self;
    fn from_le_bytes(x: Self::Bytes) -> Self;
    fn from_be_bytes(x: &[u8]) -> Self;
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
}
pub trait Endianess {}
pub struct Le;
impl Endianess for Le {}
pub struct Be;
impl Endianess for Be {}
// impl ByteOrder for u32 {
//     type Bytes = [u8;4];
//     fn to_ne_bytes(&self) -> [u8;4] {
//         todo!()
//     }
// }

#[derive(Fields)]
pub struct TestMeta {
    pub field1: u32,
    pub field2: u8,
}

use std::array::TryFromSliceError;

pub use bfd::fields::*;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn layout_test() {
        println!("{:?}", field1::layout_range());
        println!("{:?}", field2::layout_range());
    }
}
