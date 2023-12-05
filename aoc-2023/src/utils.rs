/// Hand-rolled printing macros.
/// Mostly just stolen from std
use core::fmt;

#[macro_export]
macro_rules! println {
    () => {
        core::writeln!($crate::utils::Stdout, "\n").ok()
    };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        core::writeln!($crate::utils::Stdout, $($arg)*).ok();
    }};
}

pub struct Stdout;
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            crate::syscall::write(crate::syscall::STDOUT_FILENO, s.as_ptr(), s.len() as _);
        }
        Ok(())
    }
}

pub struct Stderr;
impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            crate::syscall::write(crate::syscall::STDERR_FILENO, s.as_ptr(), s.len() as _);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        use core::fmt::Write;
        core::writeln!($crate::utils::Stderr, "[{}:{}]", core::file!(), core::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
            use core::fmt::Write;
                core::writeln!($crate::utils::Stderr, "[{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::stringify!($val), &tmp).ok();
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
