#[cfg(test)]
mod test {

    use std::alloc::Layout;
    use std::mem::MaybeUninit;

    #[test]
    fn incorrect_vec() {
        let v = Vec::<u32>::with_capacity(4);

        // This accesses the 4th element of the vector, which has been
        // allocated but is uninitialized. This is undefined behavior.
        let value = unsafe { v.get_unchecked(3) };
        eprintln!("vec value: {}", value);

        // Vec has many other unsafe fns that can result in access
        // to uninitialized memory (e.g. set_len, as_chunks_unchecked).
        // Misuse of any of them can result in undefined behavior.
    }

    #[test]
    fn misused_allocate() {
        // It should be quite rare to need to allocate memory in this
        // way. Prefer using MaybeUninit instead.

        let layout = Layout::new::<u32>();
        let heap_u32 = unsafe { std::alloc::alloc(layout) as *mut u32 };

        // This is undefined behavior because we're directly reading from
        // a pointer, that points to uninitialized memory.
        let value = unsafe { *heap_u32 };

        eprintln!("allocated value: {}", value);
    }

    #[test]
    fn bad_assume_init() {
        // This is how MaybeUninit should be used; after a value has been
        // written to it (perhaps by an FFI call into some C library)
        // then we can convert it to a real value with assume_init().
        let mut x = MaybeUninit::<u32>::uninit();
        x.write(7);
        let good_x = unsafe { x.assume_init() };

        // This is undefined behavior; it is incorrect to call assume_init()
        // here because we have never actually loaded a value into x.
        let y = MaybeUninit::<u32>::uninit();
        let bad_y = unsafe { y.assume_init() };

        println!("x: {}, y: {}", good_x, bad_y);
    }

    // To make this code sound:
    //
    // Don't ever access memory that could be uninitialized.
}
