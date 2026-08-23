use forager_bencode::{Error, to_bytes};
use serde::{Serialize, Serializer};
use std::assert_matches;
use std::collections::BTreeMap;

#[test]
fn serialize_bool() {
    let value: bool = false;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_i8() {
    let value: i8 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_i16() {
    let value: i16 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_i32() {
    let value: i32 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_i64() {
    let value: i64 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_i128() {
    let value: i128 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_u8() {
    let value: u8 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_u16() {
    let value: u16 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_u32() {
    let value: u32 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_u64() {
    let value: u64 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_u128() {
    let value: u128 = 0;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_f32() {
    let value: f32 = 0.0;
    let result = to_bytes(&value);
    assert_matches!(result.unwrap_err(), Error::NotSupported("f32"));
}

#[test]
fn serialize_f64() {
    let value: f64 = 0.0;
    let result = to_bytes(&value);
    assert_matches!(result.unwrap_err(), Error::NotSupported("f64"));
}

#[test]
fn serialize_char() {
    let value: char = 'a';
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"1:a");
}

#[test]
fn serialize_str() {
    let value: &str = "a";
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"1:a");
}

#[test]
fn serialize_bytes() {
    struct Bytes(&'static [u8]);

    impl Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_bytes(self.0)
        }
    }

    let value: Bytes = Bytes(b"a");
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"1:a");
}

#[test]
fn serialize_none() {
    let value: Option<u8> = None;
    let result = to_bytes(&value);
    assert_matches!(result.unwrap_err(), Error::NotSupported("none"));
}

#[test]
fn serialize_some() {
    let value: Option<u8> = Some(0);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_unit() {
    let value: () = ();
    let result = to_bytes(&value);
    assert_matches!(result.unwrap_err(), Error::NotSupported("unit"));
}

#[test]
fn serialize_unit_struct() {
    #[derive(Serialize)]
    struct Empty;

    let value: Empty = Empty;
    let result = to_bytes(&value);
    assert_matches!(result.unwrap_err(), Error::NotSupported("unit_struct"));
}

#[test]
fn serialize_unit_variant() {
    #[derive(Serialize)]
    enum UnitVariant {
        A,
    }

    let value = UnitVariant::A;
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"1:A");
}

#[test]
fn serialize_newtype_struct() {
    #[derive(Serialize)]
    struct Wrapper(u8);

    let value: Wrapper = Wrapper(0);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"i0e");
}

#[test]
fn serialize_newtype_variant() {
    #[derive(Serialize)]
    enum NewtypeVariant {
        A(u8),
    }

    let value = NewtypeVariant::A(0);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"d1:Ai0ee");
}

#[test]
fn serialize_seq() {
    let value: Vec<u8> = vec![0];
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"li0ee");
}

#[test]
fn serialize_tuple() {
    let value: (u8,) = (0,);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"li0ee");
}

#[test]
fn serialize_tuple_struct() {
    #[derive(Serialize)]
    struct Pair(u8, u8);

    let value: Pair = Pair(0, 0);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"li0ei0ee");
}

#[test]
fn serialize_tuple_variant() {
    #[derive(Serialize)]
    enum TupleVariant {
        A(u8, u8),
    }

    let value = TupleVariant::A(0, 0);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"d1:Ali0ei0eee");
}

#[test]
fn serialize_map() {
    let mut value = BTreeMap::new();
    value.insert("a", 0);
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"d1:ai0ee");
}

#[test]
fn serialize_struct() {
    #[derive(Serialize)]
    struct Struct {
        a: u8,
    }

    let value = Struct { a: 0 };
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"d1:ai0ee");
}

#[test]
fn serialize_struct_variant() {
    #[derive(Serialize)]
    enum StructVariant {
        A { a: u8 },
    }

    let value = StructVariant::A { a: 0 };
    let result = to_bytes(&value);
    assert_eq!(result.unwrap(), b"d1:Ad1:ai0eee");
}
