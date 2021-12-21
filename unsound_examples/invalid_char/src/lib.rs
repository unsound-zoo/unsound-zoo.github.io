#[test]
fn invalid_char() {
    // Creating this invalid char is undefined behavior, even if it is
    // never used again.
    let _x = unsafe {
        // This is a surrogate value, which is used in UTF-16 encodings
        // but has no place in a Rust char.
        char::from_u32_unchecked(0x0000dd80)
    };
}

// To make this code sound:
//
// Use `char::from_u32` instead.
