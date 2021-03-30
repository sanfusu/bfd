#![feature(const_fn)]

use core::array::TryFromSliceError;
pub use flassor_macro::*;
use std::borrow::Borrow;
pub trait ByteOrder<'a> {
    type Bytes: core::convert::TryFrom<&'a [u8], Error = TryFromSliceError> + Borrow<[u8]>;
    fn to_ne_bytes(self) -> Self::Bytes;
    fn to_le_bytes(self) -> Self::Bytes;
    fn to_be_bytes(self) -> Self::Bytes;
    fn from_ne_bytes(x: Self::Bytes) -> Self;
    fn from_le_bytes(x: Self::Bytes) -> Self;
    fn from_be_bytes(x: Self::Bytes) -> Self;
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
}
pub unsafe trait RawField<T> {
    fn raw(&self) -> T;
    fn raw_ne(&self) -> T;
}

pub trait Field<T>: RawField<T> {
    type Error;
    type Output;
    fn value(&self) -> Result<Self::Error, Self::Output>;
}

pub trait Endianess<'a> {
    fn from_bytes<T: ByteOrder<'a>>(x: T::Bytes) -> T;
    fn bytes_from<T: ByteOrder<'a>>(x: T) -> T::Bytes;
}
#[derive(Debug)]
pub struct Le;
impl<'a> Endianess<'a> for Le {
    /// From le bytes(x) to ne bytes(ret)
    fn from_bytes<T: ByteOrder<'a>>(x: T::Bytes) -> T {
        T::from_le_bytes(x)
    }
    /// convert x to le bytes
    fn bytes_from<T: ByteOrder<'a>>(x: T) -> T::Bytes {
        x.to_le_bytes()
    }
}
#[derive(Debug)]
pub struct Be;
impl<'a> Endianess<'a> for Be {
    /// From be bytes(x) to ne bytes(ret)
    fn from_bytes<T: ByteOrder<'a>>(x: T::Bytes) -> T {
        T::from_be_bytes(x)
    }

    /// convert x to be bytes
    fn bytes_from<T: ByteOrder<'a>>(x: T) -> T::Bytes {
        x.to_be_bytes()
    }
}
