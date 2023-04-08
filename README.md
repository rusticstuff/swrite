# swrite

`swrite` is a tiny Rust crate providing the `swrite!` and `swriteln!` macros as
infallible alternatives to `write!` and `writeln!` for Strings. This is safe because
writing to a String never returns `Err(_)`.

The implementation uses the `SWrite` trait. It is implemented for `String`.
With the `osstring` feature is enabled, it is also implemented for `std::ffi::OsString`.

Minimum Supported Rust Version (MSRV):
- Without the `osstring` feature (default): 1.30.0
- With the `osstring` feature: 1.64.0

## Usage

In `Cargo.toml`:

```toml
[dependencies]
swrite = "0.0.1"
```

In your Rust code:

```rust
use swrite::{SWrite, swrite, swriteln};
```

## Examples

Using `swrite!` and `swriteln!` with a `String`:

```rust
use swrite::{SWrite, swrite, swriteln};

let mut s = String::new();
swrite!(s, "Hello, {}! ", "world");
swriteln!(s, "The answer is {}.", 42);

println!("{}", s);
```

Output:

```not_rust
Hello, world! The answer is 42.
```

## License

This project is dual-licensed under [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
