#[test]
fn malformed_slice() {
    use core::slice;

    let x = String::from("hello world");
    let ptr = x.as_ptr();

    // Creating a slice that extends outside the valid (allocated,
    // initialized) range is undefined behavior, even if it is never
    // used to access memory outside the original String.

    let _y: &[u8] = unsafe { slice::from_raw_parts(ptr, 100) };
}

// To make this code sound:
//
// When assembling a slice from raw parts, the code must guarantee that
// the entire slice is valid memory; properly aligned; initialized; and
// part of the same memory allocation.
