use crate::error::{Error, Result};
use serde::Serialize;
use serde::ser;
use serde::ser::{
    Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use std::cmp::Ordering;
use std::io::Write;

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::new();
    to_writer(&mut writer, value)?;
    Ok(writer)
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}

struct Serializer<W> {
    writer: W,
}

struct ListSerializer<'s, W> {
    serializer: &'s mut Serializer<W>,
}

struct MapSerializer<'s, W> {
    serializer: &'s mut Serializer<W>,
    last_key: Vec<u8>,
    map_state: MapState,
}

struct KeySerializer<'k, 'm, W> {
    map_serializer: &'k mut MapSerializer<'m, W>,
}

struct VariantSerializer<I> {
    inner_serializer: I,
}

enum MapState {
    New,
    AwaitingKey,
    AwaitingValue,
}

impl<'s, W: Write> ser::Serializer for &'s mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ListSerializer<'s, W>;
    type SerializeTuple = ListSerializer<'s, W>;
    type SerializeTupleStruct = ListSerializer<'s, W>;
    type SerializeTupleVariant = VariantSerializer<ListSerializer<'s, W>>;
    type SerializeMap = MapSerializer<'s, W>;
    type SerializeStruct = MapSerializer<'s, W>;
    type SerializeStructVariant = VariantSerializer<MapSerializer<'s, W>>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.write_number(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        self.serialize_i64(v.try_into()?)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.serialize_i64(v.try_into()?)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        self.serialize_i64(v.try_into()?)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::NotSupported("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::NotSupported("f64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buffer = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buffer))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.write_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::NotSupported("none"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::NotSupported("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::NotSupported("unit_struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.write_dictionary_prefix()?;
        self.serialize_str(variant)?;
        value.serialize(&mut *self)?;
        self.write_end_suffix()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.write_list_prefix()?;
        Ok(ListSerializer::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.write_dictionary_prefix()?;
        self.serialize_str(variant)?;
        let serializer = self.serialize_seq(Some(len))?;
        Ok(VariantSerializer::new(serializer))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_dictionary_prefix()?;
        Ok(MapSerializer::new(self))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.write_dictionary_prefix()?;
        self.serialize_str(variant)?;
        let serializer = self.serialize_map(Some(len))?;
        Ok(VariantSerializer::new(serializer))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<W: Write> ser::Serializer for KeySerializer<'_, '_, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(Error::NotSupported("bool"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(Error::NotSupported("i8"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(Error::NotSupported("i16"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(Error::NotSupported("i32"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(Error::NotSupported("i64"))
    }

    fn serialize_i128(self, _v: i128) -> Result<Self::Ok> {
        Err(Error::NotSupported("i128"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(Error::NotSupported("u8"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(Error::NotSupported("u16"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(Error::NotSupported("u32"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(Error::NotSupported("u64"))
    }

    fn serialize_u128(self, _v: u128) -> Result<Self::Ok> {
        Err(Error::NotSupported("u128"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::NotSupported("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::NotSupported("f64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buffer = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buffer))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.map_serializer.write_key(v)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::NotSupported("none"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::NotSupported("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::NotSupported("unit_struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::NotSupported("newtype_variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::NotSupported("seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::NotSupported("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::NotSupported("tuple_struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::NotSupported("tuple_variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::NotSupported("map"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::NotSupported("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::NotSupported("struct_variant"))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<W: Write> SerializeSeq for ListSerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        self.finish()?;
        Ok(())
    }
}

impl<W: Write> SerializeTuple for ListSerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        SerializeSeq::end(self)
    }
}

impl<W: Write> SerializeTupleStruct for ListSerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        SerializeSeq::end(self)
    }
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
            MapState::New | MapState::AwaitingKey => Err(Error::ExpectedKey),
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

impl<W: Write> SerializeTupleVariant for VariantSerializer<ListSerializer<'_, W>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(&mut self.inner_serializer, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner_serializer.finish()?.write_end_suffix()
    }
}

impl<W: Write> SerializeStructVariant for VariantSerializer<MapSerializer<'_, W>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(&mut self.inner_serializer, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner_serializer.finish()?.write_end_suffix()
    }
}

impl<W: Write> Serializer<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }

    fn write_number(&mut self, number: i64) -> Result<()> {
        self.write_number_prefix()?;
        write!(self.writer, "{}", number)?;
        self.write_end_suffix()
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        write!(self.writer, "{}", bytes.len())?;
        self.writer.write_all(b":")?;
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn write_number_prefix(&mut self) -> Result<()> {
        self.writer.write_all(b"i")?;
        Ok(())
    }

    fn write_list_prefix(&mut self) -> Result<()> {
        self.writer.write_all(b"l")?;
        Ok(())
    }

    fn write_dictionary_prefix(&mut self) -> Result<()> {
        self.writer.write_all(b"d")?;
        Ok(())
    }

    fn write_end_suffix(&mut self) -> Result<()> {
        self.writer.write_all(b"e")?;
        Ok(())
    }
}

impl<'s, W: Write> ListSerializer<'s, W> {
    fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self { serializer }
    }

    fn finish(self) -> Result<&'s mut Serializer<W>> {
        self.serializer.write_end_suffix()?;
        Ok(self.serializer)
    }
}

impl<'s, W: Write> MapSerializer<'s, W> {
    fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self {
            serializer,
            last_key: Vec::new(),
            map_state: MapState::New,
        }
    }

    fn write_key(&mut self, key: &[u8]) -> Result<()> {
        match (&self.map_state, key.cmp(&self.last_key)) {
            (MapState::AwaitingKey, Ordering::Less) => Err(Error::UnsortedKey),
            (MapState::AwaitingKey, Ordering::Equal) => Err(Error::DuplicateKey),
            (MapState::AwaitingValue, _) => Err(Error::ExpectedValue),
            (MapState::New, _) | (MapState::AwaitingKey, Ordering::Greater) => {
                self.serializer.write_bytes(key)?;
                self.last_key.clear();
                self.last_key.extend_from_slice(key);
                self.map_state = MapState::AwaitingValue;
                Ok(())
            }
        }
    }

    fn finish(self) -> Result<&'s mut Serializer<W>> {
        match self.map_state {
            MapState::AwaitingValue => Err(Error::ExpectedValue),
            MapState::New | MapState::AwaitingKey => {
                self.serializer.write_end_suffix()?;
                Ok(self.serializer)
            }
        }
    }
}

impl<'k, 'm, W> KeySerializer<'k, 'm, W> {
    fn new(map_serializer: &'k mut MapSerializer<'m, W>) -> Self {
        Self { map_serializer }
    }
}

impl<I> VariantSerializer<I> {
    fn new(inner_serializer: I) -> Self {
        Self { inner_serializer }
    }
}
