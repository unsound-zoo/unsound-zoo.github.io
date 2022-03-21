# Introduction to the Unsound Zoo

----

### How to Shoot Yourself in the Foot with `unsafe`

This is a resource that I wish existed. So I'm trying to will it into existence.

The Rust programming language is very powerful, and by default prevents memory unsafety bugs, most race conditions, and avoids a lot of footguns that exist in other languages.

Sometimes you may want to break the rules a little bit, to interface with code written in other languages, or to implement a high-performance data structure that can't be built under Rust's normal rules.

The [Rustonomicon][nomicon] can teach you how to write `unsafe` code correctly.

The examples here will show you how to write `unsafe` code **incorrectly**.

----

### Goals:

- Teach people how to write *good* unsafe code by giving examples of *bad* unsafe code.
- Provide a dictionary of common mistakes, that can be pointed to (e.g. in a code review) as a reference guide as to why this particular thing is bad.
- Provide a corpus of things that can be run in Miri to demonstrate detection of undefined behavior.

----

### How to run this code in Miri

The code examples are structured as a set of standalone Rust projects. Each one has unit tests that will trigger failures in Miri.

To see the failures, first install [Miri] and then run tests from one of the example directories:

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

### What does "sound" and "unsound" mean?

It's not enough to say that "sound" code is good, and "unsound" code is bad. "Unsound" code is a particular variety of bad. When we say "unsound" we mean code that could violate Rust's normal safety guarantees.

Most people who write Rust code choose the language because they care about safety guarantees. Some of the guarantees that Rust upholds are:
- Impossible to dereference null or invalid pointers
- Protection against use-after-free
- Protection from against memory races

[David Tolnay][dt-unsound] describes "sound" code as "functionality that cannot be misused, neither by mistake nor maliciously."

We often talk about unsoundness as a property of a library's API, or of a function or type. The most common kind of unsoundness is a function that correct with certain inputs, but when used in another way might be used to break Rust's safety guarantees.

Unsoundness may manifest in a variety of ways. Some unsound code may trigger unexpected results when compiled; other parts may malfunction only when certain inputs are present. Unsound code may even behave inconsistently, depending on the compiler version or optimizer settings used.

Most Rust projects consider unsoundness bugs to be extremely high priority. [The Soundness Pledge][soundness-pledg] is a good explanation of the social contract around soundness in Rust crates.

----

### What is "undefined behavior"?

The terms "undefined behavior" and "unsoundness" are often used in similar places, but they don't mean quite the same thing. "Undefined behavior" is used to describe code where the compiler makes no guarantees about what the resulting program will do.

Code containing "undefined behavior" is fundamentally untrustworthy. It may crash; it may do nothing at all; it may trigger malfunctions in some distant part of the program. It may even work as expected some of the time. But if you care about building reliable programs, it's important that there be no undefined behavior present.

The Rust Reference has a section called [Behavior considered undefined][ref-undefined] that contains more detail.

Many of the examples in the Unsound Zoo contain undefined behavior. But there could also be examples of unsound code that do not. For example, code that uses Rust's FFI to call C functions may be unsound, but may not contain any undefined behavior.

----

### Things that are bad but not unsound

There are many ways to write bad code, and most of them aren't unsound. For example:
- Security bugs may not involve memory safety. For example, if a program or library allows directory traversal or SQL injection, that program may be untrustworthy, but is not unsound.
- Code that panics may be perfectly sound. A panic in Rust is a controlled shutdown that in many cases results in the program terminating. Panics do uphold Rust's memory safety guarantees. 
- Code that requires the Rust [`unsafe`][book-unsafe] keyword is not automatically unsound. Using `unsafe` is probably a necessary step to unsoundness, but plenty of sound code exists that uses `unsafe` internally.
- Leaking memory is not considered unsound.
- Unsafe functions/APIs are not considered inherently unsound, even if they could be used improperly to create unsoundness.
- Corrupting files or sending improper data over a network connection is not considered unsound.

----

### Why does this project exist?

There are multiple reasons. First, it's fun to build things that malfunction. It's very educational to learn the ways that things can go wrong.

Second, the examples of unsoundness in real projects are often hard to understand. They may require deep understanding of the project structure and goals. It's much easier to understand those bugs if we recreate them as a standalone example, with readability as the only goal.

Third, it's a way to get people started with [Miri]. Miri can detect unsoundness at runtime, so it's a great tool for anyone looking to write `unsafe` code in Rust.

Fourth, it exists as a reference for people to point to, when they encounter potential unsoundness in their own projects. Teams that are new to Rust may not have a mental list of all the ways things that can go wrong. Hopefully, these examples will help them.


[nomicon]: https://doc.rust-lang.org/nomicon/
[dt-unsound]: https://docs.rs/dtolnay/latest/dtolnay/macro._03__soundness_bugs.html
[Miri]: https://github.com/rust-lang/miri#readme
[ref-undefined]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[book-unsafe]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
