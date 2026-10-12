use crate::ser::error::Error;
use crate::ser::error::Result;
use crate::ser::list::ListSerializer;
use crate::ser::map::MapSerializer;
use crate::ser::variant::VariantSerializer;
use serde::{Serialize, ser};
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

pub(crate) struct Serializer<W> {
    writer: W,
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
        self.serialize_i64(v.try_into().map_err(|_| Error::integer_out_of_range())?)
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
        self.serialize_i64(v.try_into().map_err(|_| Error::integer_out_of_range())?)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        self.serialize_i64(v.try_into().map_err(|_| Error::integer_out_of_range())?)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::unsupported("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::unsupported("f64"))
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
        Err(Error::unsupported("none"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::unsupported("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::unsupported("unit_struct"))
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

impl<W: Write> Serializer<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }

    fn write_number(&mut self, number: i64) -> Result<()> {
        self.write_number_prefix()?;
        write!(self.writer, "{}", number)?;
        self.write_end_suffix()
    }

    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
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

    pub(crate) fn write_end_suffix(&mut self) -> Result<()> {
        self.writer.write_all(b"e")?;
        Ok(())
    }
}
