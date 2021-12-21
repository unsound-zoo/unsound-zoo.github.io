#![allow(clippy::transmute_int_to_bool)]

#[test]
fn transmute_to_bool() {
    use std::intrinsics::transmute;

    let x = 2u8;
    // A `bool` may only contain the bit pattern for 0 or 1.
    // Creating this malformed bool is undefined behavior,
    // even if it is never used again.
    let _y: bool = unsafe { transmute(x) };
}

// To make this code sound:
//
// let y1 = x != 0;
