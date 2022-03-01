#[test]
fn null_box() {
    use std::ptr::null_mut;

    // A `Box` may not contain a null reference. This is undefined behavior,
    // even if the value is never dereferenced.
    let _x: Box<u32> = unsafe { Box::from_raw(null_mut()) };

    // To make this code sound:
    //
    // If creating a Box from a raw pointer, add checks to ensure the pointer
    // is non-null before converting it to a Box.
    //
    // You must also ensure the pointer refers to the start of a valid memory
    // allocation, so that the value can be freed when the Box is dropped.
}

#[test]
fn misaligned_box() {
    let valid_box = Box::new(0x07070707u32);
    let u32_ptr = Box::into_raw(valid_box);

    // Create a u8 pointer, and make it point at the second byte.
    // This is safe, because u8 has no alignment requirements.
    let u8_ptr = u32_ptr as *mut u8;
    let u8_ptr = unsafe { u8_ptr.add(1) };
    assert_eq!(unsafe { *u8_ptr }, 0x07);

    // Create a u16 pointer that also points at the second byte.
    // This is a bad idea, because it is not properly aligned.
    let u16_ptr = u8_ptr as *mut u16;

    // Convert the u16 pointer into a Box.
    // This is undefined behavior, as a Box may not contain a misaligned
    // pointer.
    let invalid_box = unsafe { Box::from_raw(u16_ptr) };

    // This assert may crash on architectures that don't allow misaligned
    // memory access. This may actually work on architectures like x86 that
    // allow misaligned access, but since we relied on undefined behavior to
    // get here, we can't make any guarantees about what might happen.
    assert_eq!(*invalid_box, 0x0707);

    // There's actually a second problem here: when the invalid `Box` goes out
    // of scope, the bad pointer will attempt to be freed, which will cause
    // the program to abort. Avoid this for this example by not deallocating.
    Box::leak(invalid_box);

    // It may not be possible to make this code sound. If it was just pointer
    // alignment, we could perform a safety check and return
    // `Option<Box<u16>>`.
    //
    // But the bigger problem is that `Box` implies ownership of a chunk of
    // allocated memory. Attempting to take ownership of any pointer that's
    // not right at the beginning of a memory allocation is doomed.
    //
    // If non-ownership is intended, perhaps returning `Option<&u16>` would
    // be possible, though computing a lifetime for that reference may be
    // tricky.
}

#[test]
fn nonsense_box() {
    let ptr = 0x8888 as *mut u32;
    // This is undefined behavior. A `Box` may not contain a pointer that
    // is not a valid address.
    let x: Box<u32> = unsafe { Box::from_raw(ptr) };

    // If this box is dropped, we will attempt to free a nonsense address
    // and the program will abort. Avoid this by "leaking" the Box.
    Box::leak(x);
}

#[test]
fn dangling_box() {
    use std::ops::DerefMut;

    let mut x = Box::new(0u32);

    // Get a pointer to the value inside x, without consuming the Box.
    let ptr_x = x.deref_mut() as *mut u32;

    // Create a second Box that "owns" the same pointer as `x`. This will
    // trigger undefined behavior as soon as either x or y is dropped
    // (because whichever one remains will contain a pointer to deallocated
    // memory.
    let y = unsafe { Box::from_raw(ptr_x) };

    // Manually drop each box so the problem is obvious in Miri.
    drop(x);
    drop(y);
}
