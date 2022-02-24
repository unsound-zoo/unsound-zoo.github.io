#![allow(dead_code)]

use std::mem::MaybeUninit;
use std::ops::Index;
use std::{ptr, slice};

// This example was inspired by:
// https://doc.rust-lang.org/nightly/nomicon/leaking.html
//
// This is an example of undefined behavior, caused by an incorrect
// implementation of `Vec::drain`. The problem here is fairly subtle, as it
// can only be triggered by leaking the `Drain` iterator.

/// This is a simplified implementation of `Vec`.
///
/// It's not meant to be 100% correct, but works just well enough to illustrate
/// the problem.
struct MyVec<T> {
    contents: *mut T,
    capacity: usize,
    length: usize,
}

// We're lazy; this is the only way to initialize our Vec.
impl<T, const N: usize> From<Box<[T; N]>> for MyVec<T> {
    fn from(array: Box<[T; N]>) -> Self {
        let contents = Box::leak(array) as *mut T;
        MyVec {
            contents,
            capacity: N,
            length: N,
        }
    }
}

// This implementation of `Index` is too simplified to be useful in the real
// world, but it's good enough for this example.
impl<T> Index<usize> for MyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        unsafe { &*self.contents.add(index) as &T }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            // There's no easy way to deal with half-initialized arrays/slices.
            // To get a fully-uninitialized array, iterate over the contents,
            // dropping them.
            for _ in self.drain() {}
            // Now that the contents have been dropped, drop the array memory
            // without interpreting it as initialized.
            let contents = self.contents as *mut MaybeUninit<T>;
            let raw_slice = slice::from_raw_parts_mut(contents, self.capacity);
            let _dropme = Box::from_raw(raw_slice);
        }
    }
}

impl<T> MyVec<T> {
    /// This requests a draining iterator, which holds a mutable reference
    /// to the original `Vec`.
    //
    // The real `Vec::drain` accepts a range parameter, but we won't bother.
    fn drain<'a>(&'a mut self) -> Drain<'a, T> {
        Drain {
            index: 0,
            parent: self,
        }
    }
}

// The way `Drain` works is that it iterates over the parent `MyVec`,
// and we delay removing objects from the parent until the `Drain` is
// dropped.
//
// While this iteration is ongoing, the parent `MyVec` is in an invalid state;
// its `contents` still holds the already drained values.
//
// We attempt to prevent anyone from accessing the `MyVec` in the invalid state,
// by holding on to a mutable reference to it. This is unsound, because if
// code leaks the `Drain`, its `Drop` impl will never be run.
//
struct Drain<'a, T> {
    index: usize,
    parent: &'a mut MyVec<T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.parent.length {
            None
        } else {
            // Compute the pointer offset, then read the value (leaving the contents unchanged).
            let value = unsafe { ptr::read(self.parent.contents.add(self.index)) };
            self.index += 1;
            Some(value)
        }
    }
}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        // If any elements weren't drained, move them to the front of the
        // array, and then fix up the length.
        let count = self.parent.length - self.index;
        if count > 0 {
            unsafe {
                let src = self.parent.contents.add(self.index);
                let dst = self.parent.contents;
                ptr::copy(src, dst, count);
            }
        }
        self.parent.length = count;
    }
}

// To make this code sound:
//
//

#[test]
fn see_vec_works() {
    // Use an array of things that allocate from the heap, to make problems obvious.
    let array = Box::new([Box::new(0u8), Box::new(1u8), Box::new(2u8)]);
    let mut v = MyVec::from(array);
    {
        let mut drainer = v.drain();
        // Drop the first two values
        drainer.next();
        drainer.next();
        // end of scope causes `drainer` to be dropped.
    }

    // Check that the remaining value is what we expect.
    assert_eq!(2, *v[0]);
}

#[test]
fn fails_when_drain_leaked() {
    // Use an array of things that allocate from the heap, to make problems obvious.
    let array = Box::new([Box::new(0u8), Box::new(1u8), Box::new(2u8)]);
    let mut v = MyVec::from(array);
    {
        let mut drainer = v.drain();
        // Drop the first two values
        drainer.next();
        drainer.next();
        // When we forget the `Drain`, we prevent its `Drop` impl from being run.
        // This leaves the `MyVec` in an invalid state, allowing us to access
        // possibly-freed or uninitialized memory.
        std::mem::forget(drainer);
    }

    // Check that the remaining value is what we expect.
    assert_eq!(2, *v[0]);
}
