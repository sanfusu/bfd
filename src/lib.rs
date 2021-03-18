#![feature(const_fn)]

pub use bfd_macro::*;
use core::array::TryFromSliceError;
use std::borrow::Borrow;
pub trait ByteOrder<'a> {
    type Bytes: core::convert::TryFrom<&'a [u8], Error = TryFromSliceError> + Borrow<[u8]>;
    fn to_ne_bytes(&self) -> Self::Bytes;
    fn to_le_bytes(&self) -> Self::Bytes;
    fn to_be_bytes(&self) -> Self::Bytes;
    fn from_ne_bytes(x: Self::Bytes) -> Self;
    fn from_le_bytes(x: Self::Bytes) -> Self;
    fn from_be_bytes(x: Self::Bytes) -> Self;
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
}
pub trait Endianess {}
#[derive(Debug)]
pub struct Le;
impl Endianess for Le {}
#[derive(Debug)]
pub struct Be;
impl Endianess for Be {}
