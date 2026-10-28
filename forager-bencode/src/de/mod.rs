mod deserializer;
mod error;

pub use deserializer::{from_bytes, from_reader};
pub use error::{Error, Kind, Result};
