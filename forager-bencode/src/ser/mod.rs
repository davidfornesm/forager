mod dictionary;
mod error;
mod key;
mod list;
mod serializer;
mod variant;

pub use error::{Error, Kind, Result};
pub use serializer::{to_bytes, to_writer};
