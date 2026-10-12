use crate::ser::error::{Error, Result};
use crate::ser::key::KeySerializer;
use crate::ser::serializer::Serializer;
use serde::Serialize;
use serde::ser::{SerializeMap, SerializeStruct};
use std::cmp::Ordering;
use std::io::Write;

pub(crate) struct MapSerializer<'s, W> {
    serializer: &'s mut Serializer<W>,
    last_key: Vec<u8>,
    map_state: MapState,
}

enum MapState {
    New,
    AwaitingKey,
    AwaitingValue,
}

impl<W: Write> SerializeMap for MapSerializer<'_, W> {
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
        match self.map_state {
            MapState::New | MapState::AwaitingKey => Err(Error::value_without_key()),
            MapState::AwaitingValue => {
                value.serialize(&mut *self.serializer)?;
                self.map_state = MapState::AwaitingKey;
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok> {
        self.finish()?;
        Ok(())
    }
}

impl<W: Write> SerializeStruct for MapSerializer<'_, W> {
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

impl<'s, W: Write> MapSerializer<'s, W> {
    pub(crate) fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self {
            serializer,
            last_key: Vec::new(),
            map_state: MapState::New,
        }
    }

    pub(crate) fn write_key(&mut self, key: &[u8]) -> Result<()> {
        match (&self.map_state, key.cmp(&self.last_key)) {
            (MapState::AwaitingKey, Ordering::Less) => Err(Error::unsorted_key()),
            (MapState::AwaitingKey, Ordering::Equal) => Err(Error::duplicate_key()),
            (MapState::AwaitingValue, _) => Err(Error::key_without_value()),
            (MapState::New, _) | (MapState::AwaitingKey, Ordering::Greater) => {
                self.serializer.write_bytes(key)?;
                self.last_key.clear();
                self.last_key.extend_from_slice(key);
                self.map_state = MapState::AwaitingValue;
                Ok(())
            }
        }
    }

    pub(crate) fn finish(self) -> Result<&'s mut Serializer<W>> {
        match self.map_state {
            MapState::AwaitingValue => Err(Error::key_without_value()),
            MapState::New | MapState::AwaitingKey => {
                self.serializer.write_end_suffix()?;
                Ok(self.serializer)
            }
        }
    }
}
