#![allow(dead_code)]

// This struct is 8 bytes long:
// byte 0: x
// bytes 1-3: padding
// bytes 4-7: y
#[repr(C)]
struct StructWithPadding {
    x: u8,
    y: u32,
}

impl StructWithPadding {
    fn new(z: u16) -> Self {
        StructWithPadding {
            x: z as u8,
            y: (z as u32) << 16,
        }
    }
}

#[test]
fn read_padding() {
    let data = StructWithPadding::new(7);

    // This is more verbose than necessary, to make it easier to explain.
    // First, get a pointer to `data`.
    let data_ptr: *const StructWithPadding = &data;
    // Then, cast that pointer to a different type. This does not require
    // `unsafe`!
    let data_ptr: *const [u8; 8] = data_ptr as *const [u8; 8];
    let mut data_buf = [0u8; 8];

    // Because this reads the padding bytes, this copy is undefined behavior.
    unsafe { core::ptr::copy_nonoverlapping(data_ptr, &mut data_buf, 1) };

    // For some reason, Miri doesn't notice the problem until we read the result.
    assert_eq!(data_buf, [7, 0, 0, 0, 0, 0, 7, 0]);
}

#[test]
fn read_padding2() {
    use std::intrinsics::transmute;

    let data = StructWithPadding::new(7);

    // I'm unsure whether this reference is UB?
    let data_buf: &[u8; 8] = unsafe { transmute(&data) };

    // Because this reads the padding bytes, this copy is undefined behavior.
    let data_buf_copy: [u8; 8] = *data_buf;

    // For some reason, Miri doesn't notice the problem until we read the result.
    assert_eq!(data_buf_copy, [7, 0, 0, 0, 0, 0, 7, 0]);
}

#[test]
fn read_padding3() {
    use std::intrinsics::transmute;
    use std::mem;

    let mut data = StructWithPadding::new(7);

    // Choose this array layout so it has the same alignment as StructWithPadding.
    let mut dest = [0u32; 2];

    // Casting to a u64 reference may be UB? (Because there is uninitialized data in there)
    // mem::swap is definitely UB since it will always read the padding bytes.
    let data_cast: &mut [u32; 2] = unsafe { transmute(&mut data) };
    mem::swap(data_cast, &mut dest);

    // For some reason, Miri doesn't notice the problem until we read the result.
    assert_eq!(dest, [7, 0x70000]);
}

// To make this code sound:
//
// Use a trusted serialization crate, i.e. `serde`.
//
// It's not very common for people to attempt to copy Rust data structures
// around as untyped byte arrays, so there aren't many crates to help you.
// `zerocopy` can sometimes be useful, but it won't work on structs with
// padding bytes. It may be possible to make the padding bytes explicit,
// e.g.
//
// struct StructWithPadding {
//     x: u8,
//     __padding: [u8; 3],
//     y: u32,
// }
//
// This data structure will probably be less friendly to deal with, however.
