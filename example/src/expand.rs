#![feature(prelude_import)]
#![feature(trivial_bounds)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate bfd;
pub struct TestMeta {
    pub field1: u32,
    pub field2: u8,
    pub field3: u64,
}
mod bfd_field {
    use core::{
        convert::{AsRef, AsMut, TryInto, Into},
        borrow::Borrow,
    };
    use crate::bfd::{ByteOrder, Endianess, Le, Be};
    use fields::TestFields;
    use super::TestMeta;
    impl Into<[u8; TestMeta::plain_size]> for TestMeta {
        fn into(self) -> [u8; TestMeta::plain_size] {
            let mut ret: [u8; TestMeta::plain_size] = [0; TestMeta::plain_size];
            ret.get_mut(fields::field1::layout_range())
                .unwrap()
                .copy_from_slice(&self.field1.to_ne_bytes());
            ret.get_mut(fields::field2::layout_range())
                .unwrap()
                .copy_from_slice(&self.field2.to_ne_bytes());
            ret.get_mut(fields::field3::layout_range())
                .unwrap()
                .copy_from_slice(&self.field3.to_ne_bytes());
            ret
        }
    }
    impl TestMeta {
        pub const plain_size: usize = 0
            + core::mem::size_of::<u32>()
            + core::mem::size_of::<u8>()
            + core::mem::size_of::<u64>();
    }
    pub struct Test<'a, End: Endianess<'a>> {
        raw: &'a [u8; TestMeta::plain_size],
        phantom: core::marker::PhantomData<End>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, End: ::core::fmt::Debug + Endianess<'a>> ::core::fmt::Debug for Test<'a, End> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Test {
                    raw: ref __self_0_0,
                    phantom: ref __self_0_1,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Test");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "raw",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "phantom",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a, End: Endianess<'a>> Test<'a, End> {
        /// same as raw_from.
        pub fn new(raw: &'a [u8; TestMeta::plain_size]) -> Self {
            Self {
                raw,
                phantom: core::marker::PhantomData,
            }
        }
        /// raw_from means we didn't check the internal value.
        pub fn raw_from(raw: &'a [u8; TestMeta::plain_size]) -> Self {
            Self {
                raw,
                phantom: core::marker::PhantomData,
            }
        }
        pub fn raw(&self) -> &'a [u8; TestMeta::plain_size] {
            self.raw
        }
        pub fn get<T: fields::TestFields + ByteOrder<'a>>(&self) -> T {
            End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
        }
        pub fn to_meta(&self) -> TestMeta {
            TestMeta {
                field1: self.get::<fields::field1>().raw(),
                field2: self.get::<fields::field2>().raw(),
                field3: self.get::<fields::field3>().raw(),
            }
        }
    }
    impl<'a, End: Endianess<'a>> AsRef<[u8]> for Test<'a, End> {
        fn as_ref(&self) -> &[u8] {
            self.raw
        }
    }
    pub struct TestMut<'a, End: Endianess<'a>> {
        raw: &'a mut [u8; TestMeta::plain_size],
        phantom: core::marker::PhantomData<End>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, End: ::core::fmt::Debug + Endianess<'a>> ::core::fmt::Debug for TestMut<'a, End> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TestMut {
                    raw: ref __self_0_0,
                    phantom: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TestMut");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "raw",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "phantom",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a, End: Endianess<'a>> TestMut<'a, End> {
        pub fn new(raw: &'a mut [u8; TestMeta::plain_size]) -> Self {
            Self {
                raw,
                phantom: core::marker::PhantomData,
            }
        }
        /// raw_from means we didn't check the internal value.
        pub fn raw_from(raw: &'a mut [u8; TestMeta::plain_size]) -> Self {
            Self {
                raw,
                phantom: core::marker::PhantomData,
            }
        }
        pub fn raw_mut(&'a mut self) -> &'a mut [u8; TestMeta::plain_size] {
            self.raw
        }
        pub fn raw(&'a self) -> &'a [u8; TestMeta::plain_size] {
            self.raw
        }
        fn as_mut(&mut self) -> &mut [u8] {
            self.raw
        }
        pub fn get<T: fields::TestFields + ByteOrder<'a>>(&'a self) -> T {
            End::from_bytes((&self.raw[T::layout_range()]).try_into().unwrap())
        }
        pub fn set<T: fields::TestFields + ByteOrder<'a>>(
            &'a mut self,
            value: T,
        ) -> &'a mut TestMut<'a, End> {
            self.raw[T::layout_range()].copy_from_slice(End::to_bytes(value).borrow());
            self
        }
        pub fn to_meta(&'a self) -> TestMeta {
            TestMeta {
                field1: self.get::<fields::field1>().raw(),
                field2: self.get::<fields::field2>().raw(),
                field3: self.get::<fields::field3>().raw(),
            }
        }
    }
    impl<'a, End: Endianess<'a>> AsRef<[u8]> for TestMut<'a, End> {
        fn as_ref(&self) -> &[u8] {
            self.raw
        }
    }
    pub mod fields {
        use core::convert::TryInto;
        use crate::bfd::ByteOrder;
        pub trait TestFields {
            fn layout_range() -> core::ops::Range<usize>;
        }
        #[allow(non_camel_case_types)]
        pub struct field1 {
            value: u32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for field1 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    field1 {
                        value: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "field1");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "value",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl TestFields for field1 {
            #[inline]
            fn layout_range() -> core::ops::Range<usize> {
                0..core::mem::size_of::<u32>()
            }
        }
        impl field1 {
            pub fn new(value: u32) -> field1 {
                field1 { value }
            }
            pub fn raw(&self) -> u32 {
                self.value
            }
        }
        impl<'a> ByteOrder<'a> for field1 {
            type Bytes = [u8; core::mem::size_of::<u32>()];
            fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u32>()] {
                u32::to_ne_bytes(self.value)
            }
            fn to_le_bytes(self) -> [u8; core::mem::size_of::<u32>()] {
                u32::to_le_bytes(self.value)
            }
            fn to_be_bytes(self) -> [u8; core::mem::size_of::<u32>()] {
                u32::to_be_bytes(self.value)
            }
            fn from_le_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u32::from_le_bytes(x),
                }
            }
            fn from_be_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u32::from_be_bytes(x),
                }
            }
            fn from_ne_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u32::from_ne_bytes(x),
                }
            }
            fn from_be(x: Self) -> Self {
                Self {
                    value: u32::from_be(x.value),
                }
            }
            fn from_le(x: Self) -> Self {
                Self {
                    value: u32::from_le(x.value),
                }
            }
            fn to_be(self) -> Self {
                Self {
                    value: u32::to_be(self.value),
                }
            }
            fn to_le(self) -> Self {
                Self {
                    value: u32::to_le(self.value),
                }
            }
        }
        #[allow(non_camel_case_types)]
        pub struct field2 {
            value: u8,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for field2 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    field2 {
                        value: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "field2");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "value",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl TestFields for field2 {
            #[inline]
            fn layout_range() -> core::ops::Range<usize> {
                <field1>::layout_range().end
                    ..<field1>::layout_range().end + core::mem::size_of::<u8>()
            }
        }
        impl field2 {
            pub fn new(value: u8) -> field2 {
                field2 { value }
            }
            pub fn raw(&self) -> u8 {
                self.value
            }
        }
        impl<'a> ByteOrder<'a> for field2 {
            type Bytes = [u8; core::mem::size_of::<u8>()];
            fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                u8::to_ne_bytes(self.value)
            }
            fn to_le_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                u8::to_le_bytes(self.value)
            }
            fn to_be_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                u8::to_be_bytes(self.value)
            }
            fn from_le_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u8::from_le_bytes(x),
                }
            }
            fn from_be_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u8::from_be_bytes(x),
                }
            }
            fn from_ne_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u8::from_ne_bytes(x),
                }
            }
            fn from_be(x: Self) -> Self {
                Self {
                    value: u8::from_be(x.value),
                }
            }
            fn from_le(x: Self) -> Self {
                Self {
                    value: u8::from_le(x.value),
                }
            }
            fn to_be(self) -> Self {
                Self {
                    value: u8::to_be(self.value),
                }
            }
            fn to_le(self) -> Self {
                Self {
                    value: u8::to_le(self.value),
                }
            }
        }
        #[allow(non_camel_case_types)]
        pub struct field3 {
            value: u64,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for field3 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    field3 {
                        value: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "field3");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "value",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl TestFields for field3 {
            #[inline]
            fn layout_range() -> core::ops::Range<usize> {
                <field2>::layout_range().end
                    ..<field2>::layout_range().end + core::mem::size_of::<u64>()
            }
        }
        impl field3 {
            pub fn new(value: u64) -> field3 {
                field3 { value }
            }
            pub fn raw(&self) -> u64 {
                self.value
            }
        }
        impl<'a> ByteOrder<'a> for field3 {
            type Bytes = [u8; core::mem::size_of::<u64>()];
            fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u64>()] {
                u64::to_ne_bytes(self.value)
            }
            fn to_le_bytes(self) -> [u8; core::mem::size_of::<u64>()] {
                u64::to_le_bytes(self.value)
            }
            fn to_be_bytes(self) -> [u8; core::mem::size_of::<u64>()] {
                u64::to_be_bytes(self.value)
            }
            fn from_le_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u64::from_le_bytes(x),
                }
            }
            fn from_be_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u64::from_be_bytes(x),
                }
            }
            fn from_ne_bytes(x: Self::Bytes) -> Self {
                Self {
                    value: u64::from_ne_bytes(x),
                }
            }
            fn from_be(x: Self) -> Self {
                Self {
                    value: u64::from_be(x.value),
                }
            }
            fn from_le(x: Self) -> Self {
                Self {
                    value: u64::from_le(x.value),
                }
            }
            fn to_be(self) -> Self {
                Self {
                    value: u64::to_be(self.value),
                }
            }
            fn to_le(self) -> Self {
                Self {
                    value: u64::to_le(self.value),
                }
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for TestMeta {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for TestMeta {
    #[inline]
    fn eq(&self, other: &TestMeta) -> bool {
        match *other {
            TestMeta {
                field1: ref __self_1_0,
                field2: ref __self_1_1,
                field3: ref __self_1_2,
            } => match *self {
                TestMeta {
                    field1: ref __self_0_0,
                    field2: ref __self_0_1,
                    field3: ref __self_0_2,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TestMeta) -> bool {
        match *other {
            TestMeta {
                field1: ref __self_1_0,
                field2: ref __self_1_1,
                field3: ref __self_1_2,
            } => match *self {
                TestMeta {
                    field1: ref __self_0_0,
                    field2: ref __self_0_1,
                    field3: ref __self_0_2,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for TestMeta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            TestMeta {
                field1: ref __self_0_0,
                field2: ref __self_0_1,
                field3: ref __self_0_2,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "TestMeta");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "field1", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "field2", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "field3", &&(*__self_0_2));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl fields::field1 {
    pub fn value(&self) -> Option<u32> {
        if self.raw() > 0 {
            Some(self.raw())
        } else {
            None
        }
    }
}
use bfd_field::*;
use fields::TestFields;
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&fields::field1::layout_range(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    let mut test_data = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ];
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &match (&TestMut::<bfd::Le>::new(&mut test_data).set(fields::field1::new(0x12345678)),)
            {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
            &[::core::fmt::rt::v1::Argument {
                position: 0usize,
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 20u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Implied,
                },
            }],
        ));
    };
    let test_le: Test<bfd::Le> = Test::new(&test_data);
    {
        match (
            &test_le.get::<fields::field1>().value().unwrap(),
            &0x12345678,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
    {
        match (&test_le.get::<fields::field2>().raw(), &0x9a) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
    let test_be: Test<bfd::Be> = Test::new(&test_data);
    {
        match (&test_be.get::<fields::field1>().raw(), &0x78563412) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
    {
        match (&test_be.get::<fields::field2>().raw(), &0x9a) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
    let mut test_meta_from_le = test_le.to_meta();
    {
        match (
            &test_meta_from_le,
            &TestMeta {
                field1: 0x12345678,
                field2: 0x9a,
                field3: 0xf0debc9a78563412,
            },
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
    test_meta_from_le.field2 = 1;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &match (&test_meta_from_le,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
            &[::core::fmt::rt::v1::Argument {
                position: 0usize,
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 20u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Implied,
                },
            }],
        ));
    };
    let s = test_be.as_ref();
    {
        match (&test_data, &s) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
}
