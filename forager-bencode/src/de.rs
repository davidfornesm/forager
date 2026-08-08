use crate::{Error, Result};
use atoi::{FromRadix10SignedChecked, atoi};
use nom::Parser;
use nom::branch::alt;
use nom::bytes::{tag, take};
use nom::character::complete::digit0;
use nom::character::one_of;
use nom::combinator::{flat_map, map_opt, opt, recognize, value};
use nom::sequence::{delimited, terminated};
use serde::de::{DeserializeOwned, Visitor};
use serde::{Deserialize, de};
use std::io::Read;
use std::str::from_utf8;

pub fn from_reader<R, T>(mut reader: R) -> Result<T>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut source = Vec::new();
    reader.read_to_end(&mut source)?;
    from_bytes(&source)
}

pub fn from_bytes<'de, T>(source: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(source);
    let value = T::deserialize(&mut deserializer)?;
    deserializer.finish()?;
    Ok(value)
}

struct Deserializer<'de> {
    source: &'de [u8],
}

impl<'de, 'd> de::Deserializer<'de> for &'d mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_number()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_number()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_number()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_number()?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(self.parse_number()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_number()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_number()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_number()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_number()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self.parse_number()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported("f32"))
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported("f64"))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'de> Deserializer<'de> {
    fn new(source: &'de [u8]) -> Self {
        Self { source }
    }

    fn finish(self) -> Result<()> {
        if self.source.is_empty() {
            Ok(())
        } else {
            Err(Error::Trailing)
        }
    }

    fn parse<O>(
        &mut self,
        mut parser: impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>>,
    ) -> Result<O> {
        let (rest, output) = parser.parse(self.source)?;
        self.source = rest;
        Ok(output)
    }

    fn parse_bool(&mut self) -> Result<bool> {
        self.parse(Self::number_delimited(alt((
            value(false, Self::zero()),
            value(true, Self::one()),
        ))))
    }

    fn parse_number<N>(&mut self) -> Result<N>
    where
        N: FromRadix10SignedChecked,
    {
        self.parse(Self::number_delimited(Self::number_signed()))
    }

    fn parse_bytes(&mut self) -> Result<&'de [u8]> {
        self.parse(flat_map(terminated(
            Self::number_unsigned::<usize>(),
            Self::colon()
        ), take))
    }

    fn parse_str(&mut self) -> Result<&'de str> {
        Ok(from_utf8(self.parse_bytes()?)?)
    }

    fn number_prefix()
    -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        tag("i")
    }

    fn end_suffix()
    -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        tag("e")
    }

    fn colon() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        tag(":")
    }

    fn number_delimited<O>(
        inner: impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>>,
    ) -> impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>> {
        delimited(Self::number_prefix(), inner, Self::end_suffix())
    }

    fn number_signed<O: FromRadix10SignedChecked>()
    -> impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>> {
        map_opt(alt((Self::zero(), Self::nonzero_signed())), atoi)
    }

    fn number_unsigned<O: FromRadix10SignedChecked>()
        -> impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>> {
        map_opt(alt((Self::zero(), Self::nonzero_unsigned())), atoi)
    }

    fn nonzero_unsigned()
    -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        recognize((one_of("123456789"), digit0))
    }

    fn nonzero_signed()
    -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        recognize((opt(Self::minus()), Self::nonzero_unsigned()))
    }

    fn zero() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        tag("0")
    }

    fn one() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        tag("1")
    }

    fn minus() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
        tag("-")
    }
}
