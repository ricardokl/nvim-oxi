//! Traits for converting between Neovim [`Object`]s and Rust types.

use std::collections::HashMap;

use thiserror::Error as ThisError;

use crate::array::ArrayFromTupleError;
use crate::{
    Array,
    Boolean,
    Dictionary,
    Float,
    Function,
    Integer,
    Object,
    ObjectKind,
};

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Was expecting a \"{expected}\" but received a \"{actual}\"")]
    FromWrongType { expected: &'static str, actual: &'static str },

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    Deserialize(#[from] crate::serde::DeserializeError),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    Serialize(#[from] crate::serde::SerializeError),

    #[doc(hidden)]
    #[error("{0}")]
    Other(String),
}

/// Trait implemented for types can be obtained from an [`Object`].
pub trait FromObject: Sized {
    fn from_object(object: Object) -> Result<Self, Error>;
}

/// Trait implemented for types can be converted into an [`Object`].
pub trait ToObject {
    fn to_object(self) -> Result<Object, Error>;
}

impl FromObject for Object {
    fn from_object(obj: Object) -> Result<Self, Error> {
        Ok(obj)
    }
}

impl FromObject for () {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Nil => Ok(()),

            other => Err(Error::FromWrongType {
                expected: "nil",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Boolean {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Boolean => Ok(unsafe { obj.as_boolean_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "bool",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for Integer {
    type Error = Error;

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        match obj.kind() {
            ObjectKind::Integer
            | ObjectKind::Buffer
            | ObjectKind::Window
            | ObjectKind::TabPage => Ok(unsafe { obj.as_integer_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "integer",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Float {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Float => Ok(unsafe { obj.as_float_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "float",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Array {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Array => Ok(unsafe { obj.into_array_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl<A, R> FromObject for Function<A, R> {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::LuaRef => {
                Ok(Self::from_ref(unsafe { obj.as_luaref_unchecked() }))
            },

            other => Err(Error::FromWrongType {
                expected: "function",
                actual: other.as_static(),
            }),
        }
    }
}

impl<T: TryFrom<Object, Error = Error>> FromObject for T {
    #[inline]
    fn from_object(obj: Object) -> Result<Self, Error> {
        T::try_from(obj)
    }
}

/// Implements `FromObject` for a type that implements `From<Integer>`.
macro_rules! from_int {
    ($integer:ty) => {
        impl FromObject for $integer {
            fn from_object(obj: Object) -> Result<Self, Error> {
                Integer::from_object(obj).map(Into::into)
            }
        }
    };
}

from_int!(i128);

/// Implements `TryFrom<Object>` for a type that implements `TryFrom<Integer>`.
macro_rules! int_try_from_obj {
    ($integer:ty) => {
        impl TryFrom<Object> for $integer {
            type Error = Error;

            fn try_from(obj: Object) -> Result<Self, Self::Error> {
                Integer::try_from(obj)
                    .and_then(|n| n.try_into().map_err(Into::into))
            }
        }
    };
}

int_try_from_obj!(i8);
int_try_from_obj!(u8);
int_try_from_obj!(i16);
int_try_from_obj!(u16);
int_try_from_obj!(i32);
int_try_from_obj!(u32);
int_try_from_obj!(u64);
int_try_from_obj!(u128);
int_try_from_obj!(isize);
int_try_from_obj!(usize);

impl FromObject for f32 {
    fn from_object(obj: Object) -> Result<Self, Error> {
        Ok(Float::from_object(obj)? as _)
    }
}

impl FromObject for String {
    fn from_object(obj: Object) -> Result<Self, Error> {
        crate::String::from_object(obj)
            .map(|nvim_str| nvim_str.to_string_lossy().into())
    }
}

impl<T> FromObject for Option<T>
where
    T: FromObject,
{
    fn from_object(obj: Object) -> Result<Self, Error> {
        (!obj.is_nil()).then(|| T::from_object(obj)).transpose()
    }
}

impl<T> FromObject for Vec<T>
where
    T: FromObject,
{
    fn from_object(obj: Object) -> Result<Self, Error> {
        Array::from_object(obj)?
            .into_iter()
            .map(FromObject::from_object)
            .collect()
    }
}

/// Implements `FromObject` for tuples `(A, B, C, ..)` where all the
/// elements in the tuple are `TryFrom<Object>` with the same error.
macro_rules! tuple_from_object {
    ($($ty:ident)*) => {
        impl<Err, $($ty,)*> FromObject for ($($ty,)*)
        where
            $($ty: TryFrom<Object, Error = Err>,)*
            Err: Into<self::Error> + core::error::Error,
        {
            #[inline]
            #[allow(non_snake_case)]
            fn from_object(obj: Object) -> Result<Self, Error> {
                Array::from_object(obj)?
                    .try_into()
                    .map_err(|err: ArrayFromTupleError<Err>| match err {
                        ArrayFromTupleError::ElementFromObject { error, .. } => error.into(),
                        err @ ArrayFromTupleError::NotEnoughElements { .. } => Error::Other(err.to_string()),
                    })
            }
        }
    };
}

tuple_from_object!(A);
tuple_from_object!(A B);
tuple_from_object!(A B C);
tuple_from_object!(A B C D);
tuple_from_object!(A B C D E);
tuple_from_object!(A B C D E F);
tuple_from_object!(A B C D E F G);
tuple_from_object!(A B C D E F G H);
tuple_from_object!(A B C D E F G H I);
tuple_from_object!(A B C D E F G H I J);
tuple_from_object!(A B C D E F G H I J K);
tuple_from_object!(A B C D E F G H I J K L);
tuple_from_object!(A B C D E F G H I J K L M);
tuple_from_object!(A B C D E F G H I J K L M N);
tuple_from_object!(A B C D E F G H I J K L M N O);
tuple_from_object!(A B C D E F G H I J K L M N O P);

impl<T> ToObject for T
where
    T: Into<Object>,
{
    fn to_object(self) -> Result<Object, Error> {
        Ok(self.into())
    }
}

/// Implements `ToObject` for "big integer" types.
macro_rules! bigint_to_obj {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_object(self) -> Result<Object, Error> {
                Ok(i64::try_from(self)?.into())
            }
        }
    };
}

bigint_to_obj!(u64);
bigint_to_obj!(isize);
bigint_to_obj!(usize);
bigint_to_obj!(i128);
bigint_to_obj!(u128);

impl<T> ToObject for Vec<T>
where
    T: ToObject,
{
    fn to_object(self) -> Result<Object, Error> {
        Ok(self
            .into_iter()
            .map(ToObject::to_object)
            .collect::<Result<Array, Error>>()?
            .into())
    }
}

impl<K, V> ToObject for HashMap<K, V>
where
    K: Into<crate::String>,
    V: ToObject,
{
    fn to_object(self) -> Result<Object, Error> {
        self.into_iter()
            .map(|(k, v)| Ok((k, v.to_object()?)))
            .collect::<Result<Dictionary, Error>>()
            .map(Into::into)
    }
}
