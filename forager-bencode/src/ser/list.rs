use crate::ser::error::{Error, Result};
use crate::ser::serializer::Serializer;
use crate::token::Token;
use serde::Serialize;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct};
use std::io::Write;

pub(crate) struct ListSerializer<'s, W> {
    serializer: &'s mut Serializer<W>,
}

impl<'s, W: Write> ListSerializer<'s, W> {
    pub(crate) fn new(serializer: &'s mut Serializer<W>) -> Self {
        Self { serializer }
    }

    pub(crate) fn finish(self) -> Result<&'s mut Serializer<W>> {
        self.serializer.write_token(Token::End)?;
        Ok(self.serializer)
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
