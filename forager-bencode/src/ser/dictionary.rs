use crate::ser::error::{Error, Result};
use crate::ser::key::KeySerializer;
use crate::ser::serializer::Serializer;
use crate::token::Token;
use serde::Serialize;
use serde::ser::{SerializeMap, SerializeStruct};
use std::cmp::Ordering;
use std::io::Write;

pub(crate) struct DictionarySerializer<'s, W, K> {
    serializer: &'s mut Serializer<W>,
    last_key: K,
    dictionary_state: DictionaryState,
}

enum DictionaryState {
    New,
    AwaitingKey,
    AwaitingValue,
}

impl<W: Write> SerializeMap for DictionarySerializer<'_, W, Vec<u8>> {
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
        self.write_value(value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.finish()?;
        Ok(())
    }
}

impl<W: Write> SerializeStruct for DictionarySerializer<'_, W, &'static [u8]> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.write_static_key(key.as_bytes())?;
        self.write_value(value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.finish()?;
        Ok(())
    }
}

impl<W: Write> DictionarySerializer<'_, W, Vec<u8>> {
    pub(crate) fn write_transient_key(&mut self, key: &[u8]) -> Result<()> {
        self.write_key(key)?;
        key.clone_into(&mut self.last_key);
        Ok(())
    }
}

impl<W: Write> DictionarySerializer<'_, W, &'static [u8]> {
    fn write_static_key(&mut self, key: &'static [u8]) -> Result<()> {
        self.write_key(key)?;
        self.last_key = key;
        Ok(())
    }
}

impl<'s, W: Write, K: AsRef<[u8]> + Default> DictionarySerializer<'s, W, K> {
    pub(crate) fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self {
            serializer,
            last_key: K::default(),
            dictionary_state: DictionaryState::New,
        }
    }

    fn write_key(&mut self, key: &[u8]) -> Result<()> {
        match self.dictionary_state {
            DictionaryState::AwaitingValue => return Err(Error::key_without_value()),
            DictionaryState::New => {}
            DictionaryState::AwaitingKey => match key.cmp(self.last_key.as_ref()) {
                Ordering::Less => return Err(Error::unsorted_key()),
                Ordering::Equal => return Err(Error::duplicate_key()),
                Ordering::Greater => {}
            },
        }
        self.serializer.write_token(Token::Bytes(key))?;
        self.dictionary_state = DictionaryState::AwaitingValue;
        Ok(())
    }

    fn write_value<T>(&mut self, value: &T) -> Result<()>
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

    pub(crate) fn finish(self) -> Result<&'s mut Serializer<W>> {
        match self.dictionary_state {
            DictionaryState::AwaitingValue => Err(Error::key_without_value()),
            DictionaryState::New | DictionaryState::AwaitingKey => {
                self.serializer.write_token(Token::End)?;
                Ok(self.serializer)
            }
        }
    }
}
