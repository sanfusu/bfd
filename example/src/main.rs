#[macro_use]
extern crate bfd;

#[derive(Fields, PartialEq, Debug)]
pub struct TestMeta {
    pub field1: u32,
    pub field2: u8,
}

use bfd_field::*;
use fields::TestFields;
fn main() {
    println!("{:?}", fields::field1::layout_range());
    let test_data = [0x12, 0x34, 0x56, 0x78, 0x9a];
    let test_le: Test<bfd::Le> = Test::new(&test_data);
    assert_eq!(test_le.get::<fields::field1>().raw(), 0x78563412);
    assert_eq!(test_le.get::<fields::field2>().raw(), 0x9a);
    let test_be: Test<bfd::Be> = Test::new(&test_data);
    assert_eq!(test_be.get::<fields::field1>().raw(), 0x12345678);
    assert_eq!(test_be.get::<fields::field2>().raw(), 0x9a);

    let mut test_meta_from_le = test_le.to_meta();
    assert_eq!(
        test_meta_from_le,
        TestMeta {
            field1: 0x78563412,
            field2: 0x9a
        }
    );
    test_meta_from_le.field2 = 1;
}
