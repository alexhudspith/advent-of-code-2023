use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

#[derive(Debug)]
pub struct ParseDataError {
    pub reason: String
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseDataError(ParseDataError),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    Utf8Error(Utf8Error),
    // When not unexpected
    EndOfFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl std::error::Error for Error { }

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl From<ParseDataError> for Error {
    fn from(value: ParseDataError) -> Self {
        Self::ParseDataError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::from(ParseDataError { reason: value })
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<u8> for Error {
    fn from(value: u8) -> Self {
        Self::ParseDataError(ParseDataError { reason: format!("{value}") })
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[cfg(feature = "nom")]
impl From<nom::error::Error<String>> for Error {
    fn from(value: nom::error::Error<String>) -> Self {
        Self::ParseDataError(ParseDataError { reason: value.to_string() })
    }
}

pub fn aoc_err<E>(value: E) -> Error where Error: From<E> {
    Error::from(value)
}
