#![allow(dead_code)]

#[repr(u8)]
enum Count {
    One,
    Two,
    Many,
}

#[test]
fn enum_bogus_discriminant() {
    use std::intrinsics::transmute;

    let x = 4u8;

    // The `Count` enum may only contain the bit patterns that correspond
    // to a valid discriminator.  This enum is valid for values 0, 1, or
    // 2.
    //
    // Creating this malformed enum is undefined behavior, even if it is
    // never used again.

    let _y: Count = unsafe { transmute(x) };
}

#[test]
fn enum_bogus_discriminant2() {

    // Creating a malformed enum using a raw pointer instead of transmute.
    // This is also undefined behavior.
    let y = Count::One;
    let ptr: *mut Count = &y;
    unsafe {
        // Write a single byte 4.
        ptr.write_bytes(4, 1);
    }
}

// How to make this code sound:
//
// There are several crates that can help the programmer derive
// `TryFrom` or something equivalent.
// FIXME: name those crates
//
// It's also possible to code it yourself, though that's probably not a good
// idea because if additional variants are later added it would silently break
// this code.

struct CountError;

impl TryFrom<u8> for Count {
    type Error = CountError;

    fn try_from(x: u8) -> Result<Count, CountError> {
        match x {
            0 => Ok(Count::One),
            1 => Ok(Count::Two),
            2 => Ok(Count::Many),
            _ => Err(CountError),
        }
    }
}
