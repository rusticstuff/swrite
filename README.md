# swrite

`swrite` is a tiny Rust crate providing the infallible `swrite!` and `swriteln!` macros, 
which allow writing to a `String` without having to deal with a `Result`. This is safe because
writing to a String never returns `Err(_)`.

The implementation uses the `SWrite` trait. It is implemented for `String`.
When the `osstring` feature is enabled, it is also implemented for `std::ffi::OsString`.

Minimum Supported Rust Version (MSRV):
- Without `osstring` feature: 1.30.0
- With `osstring` feature: 1.64.0

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

fn main() {
    let mut s = String::new();
    swrite!(s, "Hello, ");
    swriteln!(s, "world!");

    println!("{}", s);
}
```

Output:

```
Hello, world!
```

## License

This project is dual-licensed under [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
</pre>



