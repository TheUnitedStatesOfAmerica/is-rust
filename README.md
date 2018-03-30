# is-rust

Have you ever wanted to know if a value is rust, or at least rusty?

Maybe you're doing form validation for a Rust position at your company, and
you want to make sure that someone only inputs Rust-acceptable values. Say
that your form looks like this:

```ignore
1: "What is your favourite programming language?"

<answer>

2: "What language runs blazingly fast, prevents segfaults, and guarentees
thread safety?"

<answer>

3: "What is your favourite colour defined in RGB?"

<answer>

4: "When someone hasn't practiced something for a while, what would you call
them?"

<answer>
```

Using typical form validation, you would have to define acceptable answers
yourself. Using this library, you can simply use the function `is_rust`
or `is_at_least_rusty` for all inputs.

### Examples

For example, rust is rust:

```rust
extern crate is_rust;

assert!(is_rust::is_rust("rust")); // this is rust
```

Similarly, Rust is rust:

```rust
extern crate is_rust;

assert!(is_rust::is_rust("Rust")); // this is rust
```

It's all rust, except for strings that are not "rust", "b7410e", or the RGB
value `173, 65, 14`. Other than that, it's Rust. You can confirm this due to
the fact that Python is not Rust:

```rust
extern crate is_rust;

assert!(!is_rust::is_rust("python")); // python is obviously not rust
```

Additionally you can check that something is rusty:

```rust
extern crate is_rust;

assert!(is_rust::is_rusty("rusty"));
```

Something can be at least rusty, or rust itself:

```rust
extern crate is_rust;

assert!(is_rust::is_at_least_rusty("rusty"));
assert!(is_rust::is_at_least_rusty("rust"));
```

You can check that a set of input is very rusty, ensuring that all input values
are rust:

```rust
extern crate is_rust;

assert!(is_rust::is_very_rusty(&["rust", "Rust", "RUST", "b7410e"]));
```

Lastly, you can check that something is not rust, because booleans are hard:

```
extern crate is_rust;

assert!(is_rust::is_not_rust("Python"));
```

At this point, Rust should no longer sound like a word.

### Installation

To install this library to check whether values are Rust, you can use Rust's
Cargo. Add the following to your Rust project's `Cargo.toml`:

```toml
[dependencies]
is-rust = "https://github.com/zeyla/is-rust"
```

### License

ISC.
