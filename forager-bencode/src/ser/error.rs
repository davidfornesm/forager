use serde::ser;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    representation: Box<Representation>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Io,
    Model,
    Limit,
}

impl Error {
    fn new(reason: Reason) -> Self {
        Self {
            representation: Box::new(Representation { reason }),
        }
    }

    #[must_use]
    pub fn kind(&self) -> Kind {
        match self.representation.reason {
            Reason::Io(_) => Kind::Io,
            Reason::Custom(_)
            | Reason::Unsupported(_)
            | Reason::UnsupportedKey(_)
            | Reason::KeyWithoutValue
            | Reason::ValueWithoutKey
            | Reason::UnsortedKey
            | Reason::DuplicateKey => Kind::Model,
            Reason::IntegerOutOfRange => Kind::Limit,
        }
    }

    fn io(error: std::io::Error) -> Self {
        Self::new(Reason::Io(error))
    }

    fn custom(message: Box<str>) -> Self {
        Self::new(Reason::Custom(message))
    }

    pub(crate) fn unsupported(message: &'static str) -> Self {
        Self::new(Reason::Unsupported(message))
    }

    pub(crate) fn unsupported_key(message: &'static str) -> Self {
        Self::new(Reason::UnsupportedKey(message))
    }

    pub(crate) fn key_without_value() -> Self {
        Self::new(Reason::KeyWithoutValue)
    }

    pub(crate) fn value_without_key() -> Self {
        Self::new(Reason::ValueWithoutKey)
    }

    pub(crate) fn unsorted_key() -> Self {
        Self::new(Reason::UnsortedKey)
    }

    pub(crate) fn duplicate_key() -> Self {
        Self::new(Reason::DuplicateKey)
    }

    pub(crate) fn integer_out_of_range() -> Self {
        Self::new(Reason::IntegerOutOfRange)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.representation.reason {
            Reason::Io(_) => f.write_str("io"),
            Reason::Custom(message) => f.write_str(message),
            Reason::Unsupported(unsupported) => write!(f, "unsupported: {unsupported}"),
            Reason::UnsupportedKey(unsupported) => write!(f, "unsupported key: {unsupported}"),
            Reason::KeyWithoutValue => f.write_str("key without value"),
            Reason::ValueWithoutKey => f.write_str("value without key"),
            Reason::UnsortedKey => f.write_str("unsorted key"),
            Reason::DuplicateKey => f.write_str("duplicate key"),
            Reason::IntegerOutOfRange => f.write_str("integer out of range"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.representation.reason {
            Reason::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string().into_boxed_str())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::io(value)
    }
}

#[derive(Debug)]
struct Representation {
    reason: Reason,
}

#[derive(Debug)]
enum Reason {
    Io(std::io::Error),
    Custom(Box<str>),
    Unsupported(&'static str),
    UnsupportedKey(&'static str),
    KeyWithoutValue,
    ValueWithoutKey,
    UnsortedKey,
    DuplicateKey,
    IntegerOutOfRange,
}
