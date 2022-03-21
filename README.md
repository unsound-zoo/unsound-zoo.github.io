# The Unsound Zoo
## A Collection of Unsound Rust Code

![Mutant Ferris](https://github.com/ericseppanen/unsound_zoo/raw/assets/ferris-mutant.png)


## Table Of Contents

### [Introduction](introduction.md)


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
- [Extending a reference lifetime](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/bad_lifetime/src/lib.rs)
- [Mutating immutable data](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/mutate_immutable/src/lib.rs)
- [Constructing illegal aliases](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/illegal_aliasing/src/lib.rs)

**Complex unsound examples**

- [UB via leaking a poorly-designed `Drain`](https://github.com/ericseppanen/unsound_zoo/blob/main/unsound_examples/buggy_drain/src/lib.rs)
