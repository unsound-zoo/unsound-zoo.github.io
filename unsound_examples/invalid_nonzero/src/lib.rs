#[cfg(test)]
mod test {
    use std::num::NonZeroU32;

    #[test]
    fn valid_nonzero_u32() {
        // This is valid, because 2 is a valid NonZeroU32.
        let x = NonZeroU32::new(2u32).unwrap();

        // 0 is not a valid NonZeroU32.
        // This code is valid, however, because it gracefully returns
        // None instead of creating an invalid NonZeroU32.
        let y = NonZeroU32::new(0u32);

        eprintln!("x: {:?}, y: {:?}", x, y);
    }

    #[test]
    fn invalid_nonzero_u32() {
        // Creating this value is undefined behavior, even if it
        // is never used again.
        let z = unsafe { NonZeroU32::new_unchecked(0u32) };

        eprintln!("z: {:?}", z);
    }

    // To make this code sound:
    //
    // Don't use NonZeroU32::new_unchecked unless you can guarantee the
    // input value is not 0.
    //
    // Using `transmute` or raw pointers to achieve the same thing is
    // just as bad.
}
