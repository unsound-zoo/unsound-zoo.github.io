# The Unsound Zoo
## A Collection of Unsound Rust Code

![Mutant Ferris](https://github.com/ericseppanen/unsound_zoo/raw/assets/ferris-mutant.png)

### How to Shoot Yourself in the Foot with `unsafe`

This is a book that I wish existed. So I'm trying to will it into existence.

The Rust programming language is very powerful, and by default prevents memory unsafety bugs, most race conditions, and avoids a lot of footguns that exist in other languages.

Sometimes you may want to break the rules a little bit, to interface with code written in other languages, or to implement a high-performance data structure that can't be built under Rust's normal rules.

The [Rustonomicon](https://doc.rust-lang.org/nomicon/) can teach you how to write `unsafe` code correctly.

The examples here will show you how to write `unsafe` code **incorrectly**.

----

### Goals:

- Teach people how to write *good* unsafe code by giving examples of *bad* unsafe code.
- Provide a dictionary of common mistakes, that can be pointed to (e.g. in a code review) as a reference guide as to why this particular thing is bad.
- Provide a corpus of things that can be run in Miri to demonstrate detection of undefined behavior.

----

### How to run this code in Miri

The code examples are structured as a set of standalone Rust projects. Each one has unit tests that will trigger failures in Miri.

To see the failures, first install [Miri](https://github.com/rust-lang/miri#readme) and then run tests from one of the example directories:

```txt
unsound_examples/null_reference$ cargo +nightly miri test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/miri/x86_64-unknown-linux-gnu/debug/deps/null_reference-ae5570a7c650d48f)

running 1 test
test null_reference ... error: Undefined Behavior: null pointer is not a valid pointer for this operation
  --> src/lib.rs:10:29
   |
10 |     let _y: &u32 = unsafe { &*x };
   |                             ^^^ null pointer is not a valid pointer for this operation
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
           
   = note: inside `null_reference` at src/lib.rs:10:29
```

----

### Links to the code

**Simple unsound examples**

- [Constructing a malformed `bool`](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/invalid_bool/src/lib.rs)
- [Constructing a malfomed `char`](https://github.com/ericseppanen/unsound_zoo/tree/main/unsound_examples/invalid_char)
- [Constructing a null reference](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/null_reference/src/lib.rs)
- [Constructing a malformed `Box`](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/invalid_box/src/lib.rs)
- [Constructing a malformed `enum`](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/invalid_enum_discriminant/src/lib.rs)
- [Constructing an invalid `NonZero__`](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/invalid_nonzero/src/lib.rs)
- [Constructing an invalid slice](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/malformed_slice/src/lib.rs)
- [Accessing uninitialized memory](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/uninitialized_memory/src/lib.rs)
- [Reading the padding bytes of a struct](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/read_padding/src/lib.rs)
- [Extending a reference](TODO)
- [Mutating immutable data](TODO)

**Complex unsound examples**

- [UB via leaking a poorly-designed `Drain`](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/buggy_drain/src/lib.rs)
