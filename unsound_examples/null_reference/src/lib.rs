#[test]
fn null_reference() {
    use std::ptr::null;

    // This is a null pointer. This is perfectly legal.
    let x: *const u32 = null();

    // This is a null reference. This is undefined behavior, even if the
    // value is never read or dereferenced.
    let _y: &u32 = unsafe { &*x };
}
