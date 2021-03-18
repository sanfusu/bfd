mod expand {}
pub struct TestMeta {
    pub field1: u32,
    pub field2: u8,
}
mod bfd_field {
    use core::convert::TryInto;
    pub struct Test<'a, End: crate::bfd::Endianess> {
        raw: &'a [u8; 0 + core::mem::size_of::<u32>() + core::mem::size_of::<u8>()],
        phantom: core::marker::PhantomData<End>,
    }
    impl<'a, End: crate::bfd::Endianess> Test<'a, End> {
        pub fn new(
            raw: &'a [u8; 0 + core::mem::size_of::<u32>() + core::mem::size_of::<u8>()],
        ) -> Self {
            Self {
                raw,
                phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> Test<'a, crate::bfd::Le> {
        pub fn get<T: fields::TestFields + crate::bfd::ByteOrder<'a>>(&self) -> T {
            T::from_le_bytes((&self.raw[T::layout_range()]).try_into().unwrap())
        }
        pub fn to_meta(&self) -> super::TestMeta {
            super::TestMeta {
                field1: self.get::<fields::field1>().raw(),
                field2: self.get::<fields::field2>().raw(),
            }
        }
    }
    impl<'a> Test<'a, crate::bfd::Be> {
        pub fn get<T: fields::TestFields + crate::bfd::ByteOrder<'a>>(&self) -> T {
            T::from_be_bytes((&self.raw[T::layout_range()]).try_into().unwrap())
        }
        pub fn to_meta(&self) -> super::TestMeta {
            super::TestMeta {
                field1: self.get::<fields::field1>().raw(),
                field2: self.get::<fields::field2>().raw(),
            }
        }
    }
    pub struct TestMut<'a, End: crate::bfd::Endianess> {
        raw: &'a mut [u8; 0 + core::mem::size_of::<u32>() + core::mem::size_of::<u8>()],
        phantom: core::marker::PhantomData<End>,
    }
    impl<'a, End: crate::bfd::Endianess> TestMut<'a, End> {
        pub fn new(
            raw: &'a mut [u8; 0 + core::mem::size_of::<u32>() + core::mem::size_of::<u8>()],
        ) -> Self {
            Self {
                raw,
                phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> TestMut<'a, crate::bfd::Le> {
        pub fn get<T: fields::TestFields + crate::bfd::ByteOrder<'a>>(&'a self) -> T {
            T::from_le_bytes((&self.raw[T::layout_range()]).try_into().unwrap())
        }
        pub fn to_meta(&self) -> super::TestMeta {
            super::TestMeta {
                field1: self.get::<fields::field1>().raw(),
                field2: self.get::<fields::field2>().raw(),
            }
        }
    }
    impl<'a> TestMut<'a, crate::bfd::Be> {
        pub fn get<T: fields::TestFields + crate::bfd::ByteOrder<'a>>(&'a self) -> T {
            T::from_be_bytes((&self.raw[T::layout_range()]).try_into().unwrap())
        }
        pub fn to_meta(&self) -> super::TestMeta {
            super::TestMeta {
                field1: self.get::<fields::field1>().raw(),
                field2: self.get::<fields::field2>().raw(),
            }
        }
    }
    pub mod fields {
        use core::convert::TryInto;
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
            fn new(value: u32) -> field1 {
                field1 { value }
            }
            pub fn raw(&self) -> u32 {
                self.value
            }
        }
        impl<'a> crate::bfd::ByteOrder<'a> for field1 {
            type Bytes = [u8; core::mem::size_of::<u32>()];
            fn to_ne_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
                u32::to_ne_bytes(self.value)
            }
            fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
                u32::to_le_bytes(self.value)
            }
            fn to_be_bytes(&self) -> [u8; core::mem::size_of::<u32>()] {
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
            fn new(value: u8) -> field2 {
                field2 { value }
            }
            pub fn raw(&self) -> u8 {
                self.value
            }
        }
        impl<'a> crate::bfd::ByteOrder<'a> for field2 {
            type Bytes = [u8; core::mem::size_of::<u8>()];
            fn to_ne_bytes(&self) -> [u8; core::mem::size_of::<u8>()] {
                u8::to_ne_bytes(self.value)
            }
            fn to_le_bytes(&self) -> [u8; core::mem::size_of::<u8>()] {
                u8::to_le_bytes(self.value)
            }
            fn to_be_bytes(&self) -> [u8; core::mem::size_of::<u8>()] {
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
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for TestMeta {
    #[inline]
    fn eq(&self, other: &TestMeta) -> bool {
        match *other {
            TestMeta {
                field1: ref __self_1_0,
                field2: ref __self_1_1,
            } => match *self {
                TestMeta {
                    field1: ref __self_0_0,
                    field2: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TestMeta) -> bool {
        match *other {
            TestMeta {
                field1: ref __self_1_0,
                field2: ref __self_1_1,
            } => match *self {
                TestMeta {
                    field1: ref __self_0_0,
                    field2: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
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
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "TestMeta");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "field1", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "field2", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
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
    let test_data = [0x12, 0x34, 0x56, 0x78, 0x9a];
    let test_le: Test<bfd::Le> = Test::new(&test_data);
    {
        match (&test_le.get::<fields::field1>().raw(), &0x78563412) {
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
        match (&test_be.get::<fields::field1>().raw(), &0x12345678) {
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
                field1: 0x78563412,
                field2: 0x9a,
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
}
