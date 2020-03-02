#[allow(unused)]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::Error::new(format!("{}", format_args!($($arg)*)))
    };
}

#[allow(unused)]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::Error::new(format!("{}", format_args!($($arg)*))))
    };
}

use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct Error(String);

impl Error {
    pub(crate) fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

