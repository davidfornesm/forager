use crate::ser::error::{Error, Result};
use crate::ser::key::KeySerializer;
use crate::ser::serializer::Serializer;
use serde::Serialize;
use serde::ser::{SerializeMap, SerializeStruct};
use std::cmp::Ordering;
use std::io::Write;

pub(crate) struct DictionarySerializer<'s, W> {
    serializer: &'s mut Serializer<W>,
    last_key: Vec<u8>,
    dictionary_state: DictionaryState,
}

enum DictionaryState {
    New,
    AwaitingKey,
    AwaitingValue,
}

impl<W: Write> SerializeMap for DictionarySerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(KeySerializer::new(self))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        match self.dictionary_state {
            DictionaryState::New | DictionaryState::AwaitingKey => Err(Error::value_without_key()),
            DictionaryState::AwaitingValue => {
                value.serialize(&mut *self.serializer)?;
                self.dictionary_state = DictionaryState::AwaitingKey;
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok> {
        self.finish()?;
        Ok(())
    }
}

impl<W: Write> SerializeStruct for DictionarySerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        SerializeMap::end(self)
    }
}

impl<'s, W: Write> DictionarySerializer<'s, W> {
    pub(crate) fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self {
            serializer,
            last_key: Vec::new(),
            dictionary_state: DictionaryState::New,
        }
    }

    pub(crate) fn write_key(&mut self, key: &[u8]) -> Result<()> {
        match (&self.dictionary_state, key.cmp(&self.last_key)) {
            (DictionaryState::AwaitingKey, Ordering::Less) => Err(Error::unsorted_key()),
            (DictionaryState::AwaitingKey, Ordering::Equal) => Err(Error::duplicate_key()),
            (DictionaryState::AwaitingValue, _) => Err(Error::key_without_value()),
            (DictionaryState::New, _) | (DictionaryState::AwaitingKey, Ordering::Greater) => {
                self.serializer.write_bytes(key)?;
                self.last_key.clear();
                self.last_key.extend_from_slice(key);
                self.dictionary_state = DictionaryState::AwaitingValue;
                Ok(())
            }
        }
    }

    pub(crate) fn finish(self) -> Result<&'s mut Serializer<W>> {
        match self.dictionary_state {
            DictionaryState::AwaitingValue => Err(Error::key_without_value()),
            DictionaryState::New | DictionaryState::AwaitingKey => {
                self.serializer.write_end_suffix()?;
                Ok(self.serializer)
            }
        }
    }
}
