use std::assert_matches;
use forager_bencode::{from_bytes, Error};

#[test]
fn deserialize_bool() {
    let source = b"i0e";
    let result = from_bytes::<bool>(source);
    assert_matches!(result.unwrap_err(), Error::NotSupported("bool"));
}

#[test]
fn deserialize_i8() {
    let source = b"i0e";
    let result = from_bytes::<i8>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_i16() {
    let source = b"i0e";
    let result = from_bytes::<i16>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_i32() {
    let source = b"i0e";
    let result = from_bytes::<i32>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_i64() {
    let source = b"i0e";
    let result = from_bytes::<i64>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_i128() {
    let source = b"i0e";
    let result = from_bytes::<i128>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_u8() {
    let source = b"i0e";
    let result = from_bytes::<u8>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_u16() {
    let source = b"i0e";
    let result = from_bytes::<u16>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_u32() {
    let source = b"i0e";
    let result = from_bytes::<u32>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_u64() {
    let source = b"i0e";
    let result = from_bytes::<u64>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_u128() {
    let source = b"i0e";
    let result = from_bytes::<u128>(source);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn deserialize_char() {
    let source = b"1:a";
    let result = from_bytes::<char>(source);
    assert_matches!(result.unwrap_err(), Error::NotSupported("char"));
}

#[test]
fn deserialize_str() {
    let source = b"1:a";
    let result = from_bytes::<&str>(source);
    assert_eq!(result.unwrap(), "a");
}

#[test]
fn deserialize_string() {
    let source = b"1:a";
    let result = from_bytes::<String>(source);
    assert_eq!(result.unwrap(), "a");
}

#[test]
fn deserialize_bytes() {
    let source = b"1:a";
    let result = from_bytes::<&[u8]>(source);
    assert_eq!(result.unwrap(), b"a");
}
