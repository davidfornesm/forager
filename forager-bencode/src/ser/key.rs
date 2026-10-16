use crate::ser::error::Error;
use crate::ser::dictionary::DictionarySerializer;
use serde::ser::Impossible;
use serde::{Serialize, ser};
use std::io::Write;

pub(crate) struct KeySerializer<'k, 'm, W> {
    dictionary_serializer: &'k mut DictionarySerializer<'m, W>,
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

    fn serialize_bool(self, _v: bool) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("bool"))
    }

    fn serialize_i8(self, _v: i8) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("i8"))
    }

    fn serialize_i16(self, _v: i16) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("i16"))
    }

    fn serialize_i32(self, _v: i32) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("i32"))
    }

    fn serialize_i64(self, _v: i64) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("i64"))
    }

    fn serialize_i128(self, _v: i128) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("i128"))
    }

    fn serialize_u8(self, _v: u8) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("u8"))
    }

    fn serialize_u16(self, _v: u16) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("u16"))
    }

    fn serialize_u32(self, _v: u32) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("u32"))
    }

    fn serialize_u64(self, _v: u64) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("u64"))
    }

    fn serialize_u128(self, _v: u128) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("u128"))
    }

    fn serialize_f32(self, _v: f32) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("f32"))
    }

    fn serialize_f64(self, _v: f64) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("f64"))
    }

    fn serialize_char(self, v: char) -> crate::ser::error::Result<Self::Ok> {
        let mut buffer = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buffer))
    }

    fn serialize_str(self, v: &str) -> crate::ser::error::Result<Self::Ok> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> crate::ser::error::Result<Self::Ok> {
        self.dictionary_serializer.write_key(v)
    }

    fn serialize_none(self) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("none"))
    }

    fn serialize_some<T>(self, value: &T) -> crate::ser::error::Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> crate::ser::error::Result<Self::Ok> {
        Err(Error::unsupported("unit_struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> crate::ser::error::Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> crate::ser::error::Result<Self::Ok>
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
    ) -> crate::ser::error::Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::unsupported("newtype_variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> crate::ser::error::Result<Self::SerializeSeq> {
        Err(Error::unsupported("seq"))
    }

    fn serialize_tuple(self, _len: usize) -> crate::ser::error::Result<Self::SerializeTuple> {
        Err(Error::unsupported("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> crate::ser::error::Result<Self::SerializeTupleStruct> {
        Err(Error::unsupported("tuple_struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> crate::ser::error::Result<Self::SerializeTupleVariant> {
        Err(Error::unsupported("tuple_variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> crate::ser::error::Result<Self::SerializeMap> {
        Err(Error::unsupported("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> crate::ser::error::Result<Self::SerializeStruct> {
        Err(Error::unsupported("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> crate::ser::error::Result<Self::SerializeStructVariant> {
        Err(Error::unsupported("struct_variant"))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'k, 'm, W> KeySerializer<'k, 'm, W> {
    pub(crate) fn new(dictionary_serializer: &'k mut DictionarySerializer<'m, W>) -> Self {
        Self { dictionary_serializer }
    }
}
