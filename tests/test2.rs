use std::{cell::{RefCell, UnsafeCell}, hint::black_box};

fn main() {
    struct MyStruct(u32);
    let mut unsafe_cell = UnsafeCell::new(MyStruct(123));
    let value = unsafe_cell.get_mut();
    let value2 = unsafe_cell.get_mut();
    black_box((value, value2));
    RefCell
}
