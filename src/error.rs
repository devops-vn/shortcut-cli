use std::error::Error;
use std::io::prelude::*;
use std::io::stderr;
use std::process::exit;

macro_rules! print_exit_message {
    ($fmt:expr) => (write!(stderr(), concat!("error: ", $fmt, "\n")).unwrap());
    ($fmt:expr, $($arg:tt)*) =>
        (write!(stderr(), concat!("Error: ", $fmt, "\n"), $($arg)*).unwrap());
}

macro_rules! error_exit {
    ($($arg:tt)*) => { {
        print_exit_message!($($arg)*);
        exit(1);
    } }
}

pub trait EasyErrorHandling<T> {
    fn unwrap_or_panic(self, message: &str) -> T;
}

impl<T> EasyErrorHandling<T> for Option<T> {
    fn unwrap_or_panic(self, message: &str) -> T {
        match self {
            Some(r) => r,
            None => {
                error_exit!("{}", message);
            }
        }
    }
}

impl<T, E: Error> EasyErrorHandling<T> for Result<T, E> {
    fn unwrap_or_panic(self, message: &str) -> T {
        match self {
            Ok(r) => r,
            Err(e) => {
                error_exit!("{} ({})", message, e);
            }
        }
    }
}
