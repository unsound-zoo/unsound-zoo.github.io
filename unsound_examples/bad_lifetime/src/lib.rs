#[cfg(test)]
mod test {

    // This function violates memory safety rules by artificially
    // extending the compiler's view of a reference lifetime.
    fn extend_lifetime(x: &u32) -> &'static u32 {
        unsafe {
            let x = x as *const u32;
            &*x
        }
    }

    #[test]
    fn bad_lifetime() {
        let heap_ref;
        let heap_value = Box::new(1234u32);
        heap_ref = extend_lifetime(&heap_value);
        drop(heap_value);

        // heap_value is now deallocated, but we still have a reference to its
        // contents. The existence of a reference to freed memory is undefined
        // behavior, but Miri won't catch the problem until we actually try to
        // read the data.

        eprintln!("value is {}", heap_ref);
    }

    // To make this code sound:
    //
    // Don't extend lifetimes using `unsafe`.
    //
    // If you need a value to live longer, consider putting it inside an Arc or
    // Rc (which can extend the lifetime of a value until the code is finished
    // with it), or Cow (which can convert a reference an owned type when
    // needed), or Box::leak the value to make it live forever.
    //
}
