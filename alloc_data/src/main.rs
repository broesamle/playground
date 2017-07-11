use std::boxed::Box;

// for testing minimal interoperability
#[no_mangle]
pub extern fn trivialfn(i: i32) -> i32 {
    println!("trivialfn({}), will return {} incremented by 1.", i, i);
    i+1
}

#[no_mangle]
pub extern fn new_storage(i: i32) -> *mut i32 {
    let b: Box<i32> = Box::new(i);
    println!("new_storage({})", i);
    Box::into_raw(b)
}

#[no_mangle]
pub extern fn store_int(i: i32, ptr: *mut i32) -> i32 {
    let mut b = unsafe { Box::from_raw(ptr) };
    let oldvalue = *b;
    *b = i;
    println!("store_int: updating value from {} to {}.", oldvalue, i);
    Box::into_raw(b);   // prevent the box beeing dropped (and memory being deallocated)
    oldvalue
}

#[no_mangle]
pub extern fn drop_storage(ptr: *mut i32) -> i32 {
    let b = unsafe { Box::from_raw(ptr) };
    let oldvalue = *b;
    println!("drop_storage({})", oldvalue);
    oldvalue
}

// needed for rustc to compile to wasm
fn main() {
    println!("rust-side main() was executed.");
}
