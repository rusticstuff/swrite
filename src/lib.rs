use std::ffi::OsString;

#[macro_export]
macro_rules! swrite {
    ($dst:expr, $($arg:tt)*) => {
        $dst.swrite_fmt(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! swriteln {
    ($dst:expr $(,)?) => {
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

impl SWrite for OsString {
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
