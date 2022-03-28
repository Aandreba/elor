use ::serde::Serializer;
use crate::*;

macro_rules! impl_ser {
    ($(fn $i:ident ($($v:ident: $t:ty),*)),+) => {
        $(
            #[inline]
            fn $i (self $(,$v:$t)*) -> Result<Self::Ok, Self::Error> {
                match self {
                    Left(x) => match x.$i($($v,)*) {
                        Ok(x) => Ok(Left(x)),
                        Err(e) => Err(Left(e))
                    },
        
                    Right(x) => match x.$i($($v,)*) {
                        Ok(x) => Ok(Right(x)),
                        Err(e) => Err(Right(e))
                    }
                }
            }
        )*
    };
}

impl<A,B> Serializer for Either<A,B> where A: Serializer, B: Serializer {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;
    type SerializeSeq = Either<A::SerializeSeq, B::SerializeSeq>;
    type SerializeTuple = Either<A::SerializeTuple, B::SerializeTuple>;
    type SerializeTupleStruct = Either<A::SerializeTupleStruct, B::SerializeTupleStruct>;
    type SerializeTupleVariant = Either<A::SerializeTupleVariant, B::SerializeTupleVariant>;
    type SerializeMap = Either<A::SerializeMap, B::SerializeMap>;
    type SerializeStruct = Either<A::SerializeStruct, B::SerializeStruct>;
    type SerializeStructVariant = Either<A::SerializeStructVariant, B::SerializeStructVariant>;

    impl_ser!(
        fn serialize_bool (v: bool),
        fn serialize_i8 (v: i8),
        fn serialize_i16 (v: i16),
        fn serialize_i32 (v: i32),
        fn serialize_i64 (v: i64),
        fn serialize_u8 (v: u8),
        fn serialize_u16 (v: u16),
        fn serialize_u32 (v: u32),
        fn serialize_u64 (v: u64),
        fn serialize_f32 (v: f32),
        fn serialize_f64 (v: f64),
        fn serialize_char (v: char),
        fn serialize_str (v: &str),
        fn serialize_bytes (v: &[u8]),
        fn serialize_none (),
        fn serialize_unit (),
        fn serialize_unit_struct (name: &'static str),
        fn serialize_unit_variant (name: &'static str, variant_index: u32, variant: &'static str)
    );

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where T: ::serde::Serialize {
        match self {
            Left(x) => match x.serialize_some(value) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_some(value) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> where T: ::serde::Serialize {
        match self {
            Left(x) => match x.serialize_newtype_struct(name, value) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_newtype_struct(name, value) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match self {
            Left(x) => match x.serialize_seq(len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_seq(len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        match self {
            Left(x) => match x.serialize_tuple(len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_tuple(len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> where T: ::serde::Serialize {
        match self {
            Left(x) => match x.serialize_newtype_variant(name, variant_index, variant, value) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_newtype_variant(name, variant_index, variant, value) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match self {
            Left(x) => match x.serialize_tuple_struct(name, len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_tuple_struct(name, len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        match self {
            Left(x) => match x.serialize_tuple_variant(name, variant_index, variant, len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_tuple_variant(name, variant_index, variant, len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match self {
            Left(x) => match x.serialize_map(len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_map(len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        match self {
            Left(x) => match x.serialize_struct(name, len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_struct(name, len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        match self {
            Left(x) => match x.serialize_struct_variant(name, variant_index, variant, len) {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.serialize_struct_variant(name, variant_index, variant, len) {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            }
        }
    }
}

impl<A,B> ::serde::de::Error for Either<A,B> where A: ::serde::de::Error, B: ::serde::de::Error {
    #[inline]
    fn custom<T> (msg:T) -> Self where T:core::fmt::Display {
        Self::Left(A::custom(msg))
    }
}

impl<A,B> ::serde::ser::Error for Either<A,B> where A: ::serde::ser::Error, B: ::serde::ser::Error {
    #[inline]
    fn custom<T> (msg:T) -> Self where T:core::fmt::Display {
        Self::Left(A::custom(msg))
    }
}

impl<A,B> ::serde::ser::StdError for Either<A,B> where A: ::serde::ser::StdError, B: ::serde::ser::StdError {}

impl<A,B> ::serde::ser::SerializeSeq for Either<A,B> where A: ::serde::ser::SerializeSeq, B: ::serde::ser::SerializeSeq {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: ::serde::Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_element(value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_element(value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn end (self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}

impl<A,B> ::serde::ser::SerializeTuple for Either<A,B> where A: ::serde::ser::SerializeTuple, B: ::serde::ser::SerializeTuple {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_element(value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_element(value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}

impl<A,B> ::serde::ser::SerializeTupleStruct for Either<A,B> where A: ::serde::ser::SerializeTupleStruct, B: ::serde::ser::SerializeTupleStruct {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_field(value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_field(value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}

impl<A,B> ::serde::ser::SerializeTupleVariant for Either<A,B> where A: ::serde::ser::SerializeTupleVariant, B: ::serde::ser::SerializeTupleVariant {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_field(value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_field(value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}

impl<A,B> ::serde::ser::SerializeMap for Either<A,B> where A: ::serde::ser::SerializeMap, B: ::serde::ser::SerializeMap {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_key(key) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_key(key) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_value(value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_value(value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}

impl<A,B> ::serde::ser::SerializeStruct for Either<A,B> where A: ::serde::ser::SerializeStruct, B: ::serde::ser::SerializeStruct {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_field(key, value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_field(key, value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}

impl<A,B> ::serde::ser::SerializeStructVariant for Either<A,B> where A: ::serde::ser::SerializeStructVariant, B: ::serde::ser::SerializeStructVariant {
    type Ok = Either<A::Ok, B::Ok>;
    type Error = Either<A::Error, B::Error>;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where T: Serialize {
        match self {
            Left(x) => {
                if let Err(e) = x.serialize_field(key, value) {
                    return Err(Left(e));
                }

                return Ok(())
            },

            Right(x) => {
                if let Err(e) = x.serialize_field(key, value) {
                    return Err(Right(e));
                }

                return Ok(())
            }
        }
    }

     #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Left(x) => match x.end() {
                Ok(x) => Ok(Left(x)),
                Err(e) => Err(Left(e))
            },

            Right(x) => match x.end() {
                Ok(x) => Ok(Right(x)),
                Err(e) => Err(Right(e))
            },
        }
    }
}