#[cfg(test)]
mod test {

    use std::sync::atomic::{AtomicU32, Ordering};

    // This function was written just to demonstrate that Rust
    // will generate code assuming two arguments may not alias.
    fn update_two_ints(a: &mut u32, b: &mut u32) {
        *a += *b;
        *b += *a;
    }

    #[test]
    fn illegal_mutable_alias() {
        let mut x = Box::new(3u32);

        // We are creating a rogue &mut to the contents of `x`.
        // If this alias (a mutable reference) ever exists at the
        // same time as another reference to the same location,
        // undefined behavior results.
        let x_alias = x.as_ref() as *const u32 as *mut u32;
        let mut x_alias = unsafe { &mut *x_alias };

        // update_values relies on the fact that it's two arguments don't
        // refer to the same location in memory. Because we violated that
        // requirement, this will return the wrong value (even when run
        // outside of Miri).
        // Miri will detect the duplicate mutable borrow, even before making
        // the function call.
        update_two_ints(&mut x, &mut x_alias);

        assert_eq!(*x, 9);
    }

    // This function shows that bad things can happen,
    // even if the aliases are shared references.
    fn update_atomic(a: &AtomicU32, b: &u32) {
        eprintln!("a: {}, b: {}", a.load(Ordering::SeqCst), *b);

        // Miri will flag this as undefined behavior.
        a.fetch_add(*b, Ordering::SeqCst);

        eprintln!("a: {}, b: {}", a.load(Ordering::SeqCst), *b);
    }

    #[test]
    fn atomic_alias() {
        let x = AtomicU32::new(1000);
        let x_alias = &x as *const AtomicU32 as *const u32;
        let x_alias = unsafe { &*x_alias };

        // This triggers undefined behavior, because we are mutating
        // the memory that is observable by a shared reference.
        update_atomic(&x, x_alias);
    }

    // To make this code sound:
    //
    // In some specialized contexts, it might be possible to construct
    // something like this that is sound, but undefined behavior lurks
    // on all sides, so there is probably no good advice that would
    // apply generally.
}
