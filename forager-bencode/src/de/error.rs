use serde::de;
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
    Format,
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
            Reason::Custom(_) => Kind::Model,
            Reason::Trailing => Kind::Format,
        }
    }

    fn io(error: std::io::Error) -> Self {
        Self::new(Reason::Io(error))
    }

    fn custom(message: Box<str>) -> Self {
        Self::new(Reason::Custom(message))
    }

    pub(crate) fn trailing() -> Self {
        Self::new(Reason::Trailing)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.representation.reason {
            Reason::Io(_) => f.write_str("io"),
            Reason::Custom(message) => f.write_str(message),
            Reason::Trailing => f.write_str("trailing"),
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

impl de::Error for Error {
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
    Trailing,
}
