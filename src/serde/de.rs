use ::serde::Deserializer;
use crate::*;

macro_rules! impl_de {
    ($($i:ident),+) => {
        $(
            #[inline]
            fn $i<V> (self, visitor: V) -> Result<V::Value, Self::Error> where V: ::serde::de::Visitor<'de> {
                match self {
                    Left(x) => x.$i(visitor).map_err(|e| Left(e)),
                    Right(x) => x.$i(visitor).map_err(|e| Right(e))
                }
            }
        )*
    };

    ($(fn $i:ident ($($v:ident: $t:ty),+)),+) => {
        $(
            #[inline]
            fn $i<V> (self, $($v: $t,)* visitor: V) -> Result<V::Value, Self::Error> where V: ::serde::de::Visitor<'de> {
                match self {
                    Left(x) => x.$i($($v,)* visitor).map_err(|e| Left(e)),
                    Right(x) => x.$i($($v,)* visitor).map_err(|e| Right(e))
                }
            }
        )*
    }
}

impl<'de, A, B> Deserializer<'de> for Either<A,B> where A: Deserializer<'de>, B: Deserializer<'de> {
    type Error = Either<A::Error, B::Error>;

    impl_de!(
        deserialize_any, deserialize_bool,
        deserialize_f32, deserialize_f64,
        deserialize_i8, deserialize_i16, deserialize_i32, deserialize_i64,
        deserialize_u8, deserialize_u16, deserialize_u32, deserialize_u64,
        deserialize_char, deserialize_str, deserialize_string,
        deserialize_bytes, deserialize_byte_buf,
        deserialize_option, deserialize_unit, 
        deserialize_seq, deserialize_map,
        deserialize_identifier, deserialize_ignored_any
    );

    impl_de!(
        fn deserialize_unit_struct (name: &'static str),
        fn deserialize_newtype_struct (name: &'static str),
        fn deserialize_tuple (len: usize),
        fn deserialize_tuple_struct (name: &'static str, len: usize),
        fn deserialize_struct (name: &'static str, fields: &'static [&'static str]),
        fn deserialize_enum (name: &'static str, variants: &'static [&'static str])
    );
} 