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
    last_key: Option<K>,
    state: State,
}

impl<'s, W: Write, K: AsRef<[u8]>> DictionarySerializer<'s, W, K> {
    pub(crate) const fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self {
            serializer,
            last_key: None,
            state: State::ExpectingKey,
        }
    }

    fn write_key(&mut self, key: &[u8]) -> Result<()> {
        match self.state {
            State::ExpectingValue => return Err(Error::key_without_value()),
            State::ExpectingKey => {
                if let Some(last_key) = &self.last_key {
                    match key.cmp(last_key.as_ref()) {
                        Ordering::Less => return Err(Error::unsorted_key()),
                        Ordering::Equal => return Err(Error::duplicate_key()),
                        Ordering::Greater => {}
                    }
                }
            }
        }
        self.serializer.write_token(Token::Bytes(key))?;
        self.state = State::ExpectingValue;
        Ok(())
    }

    fn write_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self.state {
            State::ExpectingKey => return Err(Error::value_without_key()),
            State::ExpectingValue => {}
        }
        value.serialize(&mut *self.serializer)?;
        self.state = State::ExpectingKey;
        Ok(())
    }

    pub(crate) fn finish(self) -> Result<&'s mut Serializer<W>> {
        match self.state {
            State::ExpectingValue => return Err(Error::key_without_value()),
            State::ExpectingKey => {}
        }
        self.serializer.write_token(Token::End)?;
        Ok(self.serializer)
    }
}

impl<W: Write> DictionarySerializer<'_, W, Vec<u8>> {
    pub(crate) fn write_transient_key(&mut self, key: &[u8]) -> Result<()> {
        self.write_key(key)?;
        match &mut self.last_key {
            Some(last_key) => key.clone_into(last_key),
            None => self.last_key = Some(key.to_vec()),
        }
        Ok(())
    }
}

impl<W: Write> DictionarySerializer<'_, W, &'static [u8]> {
    fn write_static_key(&mut self, key: &'static [u8]) -> Result<()> {
        self.write_key(key)?;
        self.last_key = Some(key);
        Ok(())
    }
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

enum State {
    ExpectingKey,
    ExpectingValue,
}
