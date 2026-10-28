use crate::ser::dictionary::DictionarySerializer;
use crate::ser::error::{Error, Result};
use crate::ser::list::ListSerializer;
use crate::token::Token;
use serde::Serialize;
use serde::ser::{SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTupleVariant};
use std::io::Write;

pub(crate) struct VariantSerializer<I> {
    inner_serializer: I,
}

impl<W: Write> SerializeStructVariant
    for VariantSerializer<DictionarySerializer<'_, W, &'static [u8]>>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        SerializeStruct::serialize_field(&mut self.inner_serializer, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner_serializer.finish()?.write_token(Token::End)?;
        Ok(())
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
        self.inner_serializer.finish()?.write_token(Token::End)?;
        Ok(())
    }
}

impl<I> VariantSerializer<I> {
    pub(crate) fn new(inner_serializer: I) -> Self {
        Self { inner_serializer }
    }
}
