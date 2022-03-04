#[cfg(test)]
mod test {

    use std::sync::Arc;
    use std::thread;

    // This test does have a race condition, and it is undefined behavior
    // according to the Rust reference, but Miri is not able to detect
    // the problem.
    //
    #[test]
    fn mutate_via_shared_reference() {
        let message = Arc::new(String::from("Hello, world!"));

        // Attempting to get Miri to detect the race...
        let message1 = Arc::clone(&message);
        let child = thread::spawn(move || {
            eprintln!("message1: {}", message1);
        });

        let message_ref: &str = &*message;
        let message_ptr = message_ref.as_ptr() as *mut u8;
        unsafe {
            message_ptr.write_bytes(b'x', 5);
        }

        eprintln!("message: {}", message);

        child.join().unwrap();
    }

    // To make this code sound:
    //
    // Use std library tools to mutate shared data: Mutex, atomics, etc.
    // Or, use well-trusted third party crates.
    // 
    // If you do need to use unsafe to create your own data structure that
    // manages concurrent access, make certain that race invariants are upheld.
}

#[cfg(test)]
mod tests_that_dont_work {

    const MESSAGE: &str = "Hello, constant!";

    // This attempt doesn't work, and will never work.
    // It was a dumb idea, but I'll leave it here just as an example.
    // It's not really "unsound" because it simply can't execute in the
    // first place: it just segfaults.
    #[test]
    #[ignore = "segfaults because MESSAGE is in a read-only part of the program"]
    fn mutate_constant() {
        let pointer = MESSAGE.as_ptr() as *mut u8;
        unsafe {
            pointer.write_bytes(b'x', 5);
        }
        println!("message: {:?}", MESSAGE);
    }

    // To make this code sound:
    //
    //
}
