#[macro_use]
extern crate flassor;

#[derive(Accessor, PartialEq, Debug)]
#[repr(C)]
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

use fields::TestFields;
use flassor::Le;
use test_accessor::*;

pub type TestFlatIntel<'a> = TestFlat<'a, Le>;
#[no_mangle]
fn example_sanfusu() {
    let test_data = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xee, 0xdd,
        0xff,
    ];
    let test_le = TestFlatIntel::from_raw(&test_data);
    let value = test_le.get::<fields::Field2>().raw();

    print!("{}", value);
}
#[cfg(test)]
mod test {
    use super::*;
    fn test_data() -> [u8; 16] {
        [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xee,
            0xdd, 0xff,
        ]
    }
    #[test]
    fn meta_into_array() {
        let data = test_data();

        let test = TestFlat::<flassor::Le>::from_raw(&data);

        let meta_arr: [u8; Test::flat_size()] = test.to_meta().into();
        assert_eq!(data, meta_arr);
    }
    #[test]
    fn offset() {
        let b =
            unsafe { &((*(0 as *const super::Test)).field3) as *const u64 as *const u8 as usize };
        println!("{}", b);
    }
}

fn main() {
    example_sanfusu();
    println!("{:?}", fields::Field1::layout_range());
    let mut test_data = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xee, 0xdd,
        0xff,
    ];
    println!(
        "{:#x?}",
        TestFlatMut::<flassor::Le>::new(&mut test_data).set(fields::Field1::new(0x12345678))
    );
    let test_le: TestFlat<flassor::Le> = TestFlat::from_raw(&test_data);
    // assert_eq!(test_le.get::<fields::field1>().value().unwrap(), 0x12345678);
    // assert_eq!(test_le.get::<fields::field2>().raw(), 0x9a);
    let value = test_le.get::<fields::Field2>().raw();
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
