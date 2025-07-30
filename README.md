# subtype\_rs

A lightweight Rust crate providing a `#[subtype]` procedural macro attribute for defining validated numeric subtypes with compile-time-specified bounds.

## Features

* Define newtype wrappers over primitive numeric types with `min` and `max` bounds.
* Automatic implementation of:

  * `new(value) -> Result<Self, SubtypeError<T>>`
  * `TryFrom<T>` for infallible conversions.
  * `Display` for easy formatting.
* Zero-cost abstractions: bounds checking inlined at compile time.

## Installation

Add `subtype_rs` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
subtype_rs = "0.1"
```

Then import the macro in your crate root or module:

```rust
use subtype_rs::subtype;
```

## Usage

Annotate a tuple struct containing exactly one numeric field with `#[subtype(min = <value>, max = <value>)]`:

```rust
use subtype_rs::subtype;

#[subtype(min = 0, max = 255)]
struct Byte(u8);

fn main() {
    // Valid
    let b = Byte::new(42).unwrap();
    println!("Byte: {}", b);

    // Below minimum
    assert!(matches!(Byte::new(300), Err(SubtypeError::AboveMaximum(300))));
}
```

### Generated API

* `struct Name(pub T);`
* `fn Name::new(value: T) -> Result<Name, SubtypeError<T>>`
* `impl TryFrom<T> for Name`
* `impl Display for Name`
* `enum SubtypeError<T> { BelowMinimum(T), AboveMaximum(T) }`

Use `SubtypeError<T>` to handle out-of-bounds errors.

## Error Handling

```rust
use subtype_rs::{subtype, SubtypeError};

#[subtype(min = 10, max = 20)]
struct Score(i32);

fn parse_score(n: i32) -> i32 {
    match Score::new(n) {
        Ok(score) => score.into_inner(),
        Err(SubtypeError::BelowMinimum(val)) => {
            eprintln!("Value {} is too small", val);
            10 // default
        }
        Err(SubtypeError::AboveMaximum(val)) => {
            eprintln!("Value {} is too large", val);
            20 // clamp
        }
    }
}
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions welcome! Please open issues or pull requests on [GitHub](https://github.com/Feralthedogg/subtype_rs).
