# Rust Quiz 2

## Problem
What is the output of this Rust program?

## Options
- [x] A The program exhibits undefined behavior
- [x] B The program does not compile
- [x] C The program is guaranteed to output: 

## Hint
One of these four closures is unlike the other three.

## Reveal
The closures `f`, `g`, and `h` are all of type `impl Fn()`. 
The closure bodies are parsed as an invocation of the user-defined bitwise-AND operator defined above by the `BitAnd` trait impl. 
When the closures are invoked, the bitwise-AND implementation prints the content of the `S` from the right-hand side and evaluates to `()`.

The closure `i` is different. Formatting the code with rustfmt makes it clearer how i is parsed.
```rust
let i = || {
    {}
    &S(4)
};
```
The closure body consists of an empty block-statement `{}` followed by a _reference_ to `S(4)`, not a bitwise-AND. 
The type of `i` is `impl Fn() -> &'static S`.

The parsing of this case is governed by [this code](1) in libsyntax.

## Answer
C, 123


[1]: https://github.com/rust-lang/rust/blob/1.30.1/src/libsyntax/parse/classify.rs#L17-L37