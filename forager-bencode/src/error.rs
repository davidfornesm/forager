use serde::{de, ser};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    NotSupported(&'static str),
    ExpectedKey,
    ExpectedValue,
    UnsortedKey,
    DuplicateKey,
    Syntax,
    Trailing,
    Eof,
    Unrepresentable,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(msg) => f.write_str(msg),
            Error::NotSupported(msg) => write!(f, "not supported: {}", msg),
            Error::UnsortedKey => f.write_str("unsorted key"),
            Error::DuplicateKey => f.write_str("duplicate key"),
            Error::ExpectedKey => f.write_str("expected key"),
            Error::ExpectedValue => f.write_str("expected value"),
            Error::Trailing => f.write_str("trailing"),
            Error::Eof => f.write_str("eof"),
            Error::Syntax => f.write_str("syntax"),
            Error::Unrepresentable => f.write_str("unrepresentable"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_value: std::str::Utf8Error) -> Self {
        Error::Syntax
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(_value: std::num::TryFromIntError) -> Self {
        Error::Unrepresentable
    }
}

impl<'a> From<nom::Err<nom::error::Error<&'a [u8]>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&'a [u8]>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => Error::Eof,
            nom::Err::Error(_) | nom::Err::Failure(_) => Error::Syntax,
        }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}
