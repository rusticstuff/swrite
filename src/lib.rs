//! `swrite` is a tiny Rust crate providing the `swrite!` and `swriteln!` macros as
//! infallible alternatives to `write!` and `writeln!` for Strings. This is safe because
//! writing to a String never returns `Err(_)`.
//!
//! The implementation uses the `SWrite` trait. It is implemented for `String`.
//! With the `osstring` feature is enabled, it is also implemented for `std::ffi::OsString`.
//!
//! Minimum Supported Rust Version (MSRV):
//! - Without the `osstring` feature (default): 1.30.0
//! - With the `osstring` feature: 1.64.0
//!
//! # Usage
//!
//! In `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! swrite = "0.0.1"
//! ```
//!
//! In your Rust code:
//!
//! ```rust
//! use swrite::{SWrite, swrite, swriteln};
//! ```
//!
//! # Examples
//!
//! Using `swrite!` and `swriteln!` with a `String`:
//!
//! ```rust
//! use swrite::{SWrite, swrite, swriteln};
//!
//! let mut s = String::new();
//! swrite!(s, "Hello, ");
//! swriteln!(s, "world!");
//!
//! println!("{}", s);
//! ```
//!
//! Output:
//!
//! ```not_rust
//! Hello, world!
//! ```
//!
//! # License
//!
//! This project is dual-licensed under [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.

#[macro_export]
macro_rules! swrite {
    ($dst:expr, $($arg:tt)*) => {
        $dst.swrite_fmt(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! swriteln {
    ($dst:expr) => {
        $crate::swrite!($dst, "\n")
    };
    ($dst:expr,) => {
        $crate::swrite!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.swrite_fmt_nl(format_args!($($arg)*))
    };
}

pub trait SWrite {
    fn swrite_fmt(&mut self, fmt: std::fmt::Arguments<'_>);
    fn swrite_fmt_nl(&mut self, fmt: std::fmt::Arguments<'_>);
}

impl SWrite for String {
    #[inline]
    fn swrite_fmt(&mut self, fmt: std::fmt::Arguments<'_>) {
        // write_fmt() never fails for Strings.
        let _ = std::fmt::Write::write_fmt(self, fmt);
    }

    #[inline]
    fn swrite_fmt_nl(&mut self, fmt: std::fmt::Arguments<'_>) {
        self.swrite_fmt(fmt);
        self.push('\n');
    }
}

#[cfg(feature = "osstring")]
impl SWrite for std::ffi::OsString {
    #[inline]
    fn swrite_fmt(&mut self, fmt: std::fmt::Arguments<'_>) {
        // write_fmt() never fails for OsStrings.
        let _ = std::fmt::Write::write_fmt(self, fmt);
    }

    #[inline]
    fn swrite_fmt_nl(&mut self, fmt: std::fmt::Arguments<'_>) {
        self.swrite_fmt(fmt);
        self.push("\n");
    }
}

#[cfg(all(test, feature = "osstring"))]
mod osstring_tests;
