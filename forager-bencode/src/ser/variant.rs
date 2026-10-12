use crate::ser::error::{Error, Result};
use crate::ser::list::ListSerializer;
use crate::ser::map::MapSerializer;
use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTupleVariant};
use std::io::Write;

pub(crate) struct VariantSerializer<I> {
    inner_serializer: I,
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

impl<I> VariantSerializer<I> {
    pub(crate) fn new(inner_serializer: I) -> Self {
        Self { inner_serializer }
    }
}
