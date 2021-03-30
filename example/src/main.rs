#![feature(trivial_bounds)]
#[macro_use]
extern crate flassor;

#[derive(Fields, PartialEq, Debug)]
pub struct Test {
    pub field1: u32,
    pub field2: u8,
    pub field3: u64,
}

// impl fields::field1 {
//     pub fn value(&self) -> Option<u32> {
//         if self.raw() > 0 {
//             Some(self.raw())
//         } else {
//             None
//         }
//     }
// }

use flassor_field::*;
use fields::TestFields;

#[no_mangle]
fn example_sanfusu() {
    let test_data = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ];
    let test_le: TestFlat<flassor::Le> = TestFlat::raw_from(&test_data);
    let value = test_le.get::<fields::field2>().raw();
    print!("{}", value);
}
#[cfg(test)]
mod test {
    use super::*;
    fn test_data() -> [u8; 13] {
        [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        ]
    }
    #[test]
    fn meta_into_array() {
        let data = test_data();

        let test = TestFlat::<flassor::Le>::raw_from(&data);

        let meta_arr: [u8; Test::plain_size] = test.to_meta().into();
        assert_eq!(data, meta_arr);
    }
}

fn main() {
    example_sanfusu();
    println!("{:?}", fields::field1::layout_range());
    let mut test_data = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ];
    println!(
        "{:#x?}",
        TestFlatMut::<flassor::Le>::new(&mut test_data).set(fields::field1::new(0x12345678))
    );
    let test_le: TestFlat<flassor::Le> = TestFlat::raw_from(&test_data);
    // assert_eq!(test_le.get::<fields::field1>().value().unwrap(), 0x12345678);
    // assert_eq!(test_le.get::<fields::field2>().raw(), 0x9a);
    let value = test_le.get::<fields::field2>().raw();
    print!("{}", value);
    // let test_be: Test<flassor::Be> = Test::new(&test_data);
    // assert_eq!(test_be.get::<fields::field1>().raw(), 0x78563412);
    // assert_eq!(test_be.get::<fields::field2>().raw(), 0x9a);

    // let mut test_meta_from_le = test_le.to_meta();
    // assert_eq!(
    //     test_meta_from_le,
    //     TestMeta {
    //         field1: 0x12345678,
    //         field2: 0x9a,
    //         field3: 0xf0debc9a78563412
    //     }
    // );
    // test_meta_from_le.field2 = 1;
    // println!("{:#x?}", test_meta_from_le);

    // let s = test_be.as_ref();
    // assert_eq!(test_data, s);
}
