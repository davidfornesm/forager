use nom::Parser;
use nom::branch::alt;
use nom::bytes::{tag, take};
use nom::character::complete::{digit0, i64, usize};
use nom::character::one_of;
use nom::combinator::{flat_map, map_parser, opt, recognize};
use nom::sequence::{delimited, terminated};

pub(crate) fn number<'de>() -> impl Parser<&'de [u8], Output = i64, Error = nom::error::Error<&'de [u8]>> {
    number_delimited(integer())
}

pub(crate) fn bytes<'de>() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
    flat_map(terminated(length(), colon()), take)
}

fn number_prefix<'de>()
-> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
    tag("i")
}

fn end_suffix<'de>()
-> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
    tag("e")
}

fn colon<'de>() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>>
{
    tag(":")
}

fn number_delimited<'de, O>(
    inner: impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>>,
) -> impl Parser<&'de [u8], Output = O, Error = nom::error::Error<&'de [u8]>> {
    delimited(number_prefix(), inner, end_suffix())
}

fn integer<'de>() -> impl Parser<&'de [u8], Output = i64, Error = nom::error::Error<&'de [u8]>> {
    map_parser(alt((zero(), nonzero())), i64)
}

fn length<'de>() -> impl Parser<&'de [u8], Output = usize, Error = nom::error::Error<&'de [u8]>> {
    map_parser(alt((zero(), natural())), usize)
}

fn natural<'de>() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>>
{
    recognize((one_of("123456789"), digit0))
}

fn nonzero<'de>() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>>
{
    recognize((opt(minus()), natural()))
}

fn zero<'de>() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>> {
    tag("0")
}

fn minus<'de>() -> impl Parser<&'de [u8], Output = &'de [u8], Error = nom::error::Error<&'de [u8]>>
{
    tag("-")
}
