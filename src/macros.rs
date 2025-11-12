macro_rules! mtid_struct {
    (
        Self = $SelfT:ident,
        ActualT = $ActualT:ty,
        description = $description:literal,
        example_str = $example_str:literal,
        example_int = $example_int:literal,
    ) => {

        #[doc = concat!($description)]
        ///
        /// # Examples
        ///
        /// ```
        /// # use mtid::*;
        /// # fn main() -> Result<(), Error> {
        /// // Generate random value.
        #[doc = concat!("let random = ", stringify!($SelfT), "::random();")]
        ///
        #[doc = concat!("assert_ne!(random, ", stringify!($SelfT), "::NIL);")]
        ///
        /// // Parse from string.
        #[doc = concat!("let from_str: ", stringify!($SelfT), " = ", stringify!($example_str), ".parse()?;")]
        ///
        /// // Parse from integer.
        #[doc = concat!("let from_int: ", stringify!($SelfT), " = ", $example_int, ".try_into()?;")]
        ///
        /// assert_eq!(from_str, from_int);
        /// # Ok(())
        /// # }
        /// ```
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $SelfT($ActualT);
    };
}
macro_rules! mtid_impl {
    (
        Self = $SelfT:ty,
        Uint = $Uint:ty,
        BITS = $BITS:literal,
        CAPACITY = $CAPACITY:expr,
        NIL_STR = $NIL_STR:literal,
        MAX_STR = $MAX_STR:literal,
        MAX_INT = $MAX_INT:literal,
        EXAMPLE_VALID_INT = $EXAMPLE_VALID_INT:literal,
        EXAMPLE_OVERSIZED_INT = $EXAMPLE_OVERSIZED_INT:literal
    ) => {

        impl $SelfT {
            /// The size of the integer type in bits.
            /// This is not equal actually stored size.
            pub const BITS: u32 = $BITS;

            /// The capacity value of this triplet id type
            ///
            /// #Examples
            ///
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($SelfT), "::try_from(", stringify!($SelfT), "::CAPACITY -1)?);")]
            /// # Ok(())
            /// # }
            /// ```
            pub const CAPACITY: $Uint = $CAPACITY;

            pub(crate) const CAPACITY_MINUS_ONE: $Uint = Self::CAPACITY - 1;

            /// The smallest value that can be represented by this triplet id type.
            ///
            /// # Examples
            ///
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::NIL, ", stringify!($NIL_STR), ".parse::<", stringify!($SelfT), ">()?);")]
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::NIL, ", stringify!($SelfT), "::try_from(0)?);")]
            /// # Ok(())
            /// # }
            /// ```
            ///
            pub const NIL: Self = Self::from_uint_unchecked(0);

            /// The largest value that can be represent by this triplet id type.
            ///
            /// # Examples
            ///
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($MAX_STR), ".parse::<",stringify!($SelfT),">()?);")]
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($SelfT), "::try_from(", stringify!($MAX_INT), ")?);")]
            /// # Ok(())
            /// # }
            /// ```
            pub const MAX: Self = Self::from_uint_unchecked(Self::CAPACITY_MINUS_ONE);



            /// Test if the triplet id is nil.
            ///
            /// # Examples
            ///
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert!(", stringify!($NIL_STR), ".parse::<",stringify!($SelfT),">()?.is_nil());")]
            /// # Ok(())
            /// # }
            /// ```
            pub fn is_nil(self) -> bool {
                self == Self::NIL
            }

            /// Test if the triplet id is max.
            ///
            /// # Examples
            ///
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert!(", stringify!($MAX_STR), ".parse::<",stringify!($SelfT),">()?.is_max());")]
            /// # Ok(())
            /// # }
            /// ```
            pub fn is_max(self) -> bool {
                self == Self::MAX
            }

            #[doc = concat!("Converts an unsigned integer to `", stringify!($SelfT), "` by truncating bits that exceed the valid range.")]
            ///
            /// This is a lossy conversion that masks the input value to fit within the ID's bit limit.
            /// If you need to detect out-of-range values, use [`TryFrom`] instead.
            ///
            /// # Examples
            ///
            /// ```
            /// # use mtid::*;
            /// // Values within range are preserved
            #[doc = concat!("let id = ", stringify!($SelfT), "::from_uint_lossy(", $EXAMPLE_VALID_INT, "); // ", stringify!($EXAMPLE_VALID_INT))]
            #[doc = concat!("assert_eq!(", stringify!($Uint), "::from(id), ", $EXAMPLE_VALID_INT, ");")]
            ///
            #[doc = concat!("// values exceeding ", $BITS, "bits are truncated (MSB(s) dropped")]
            #[doc = concat!("let id = ", stringify!($SelfT), "::from_uint_lossy(", $EXAMPLE_OVERSIZED_INT, "); // ", stringify!($EXAMPLE_OVERSIZED_INT))]
            #[doc = concat!("assert_eq!(", stringify!($Uint), "::from(id), ", $EXAMPLE_VALID_INT, "); // Only lower ", $BITS, " bits retained")]
            /// ```
            pub fn from_uint_lossy(int: $Uint) -> Self {
                Self::from_uint_unchecked(int & Self::CAPACITY_MINUS_ONE)
            }

            pub(crate) const fn from_uint_unchecked(value: $Uint) -> Self {
                Self(value)
            }
        }

        impl TryFrom<$Uint> for $SelfT {
            type Error = Error;
            #[doc = concat!("Attempts to convert a [`", stringify!($Uint), "`]  to [`", stringify!($SelfT), "`].")]
            ///
            /// Return error if the value is equal [`CAPACITY`](Self::CAPACITY) or more.
            /// If you don't need to detect out-of-range values, use [`from_uint_lossy`](Self::from_uint_lossy).
            ///
            /// # Examples
            ///
            /// ```
            /// # use mtid::*;
            #[doc = concat!("assert!(", stringify!($SelfT), "::try_from(", $EXAMPLE_VALID_INT, ").is_ok());")]
            #[doc = concat!("assert!(", stringify!($SelfT), "::try_from(", $EXAMPLE_OVERSIZED_INT, ").is_err());")]
            /// ```
            ///
            fn try_from(value: $Uint) -> Result<Self, Self::Error> {
                if value < Self::CAPACITY {
                    Ok(Self::from_uint_unchecked(value))
                } else {
                    Err(Error::ParseInteger {
                        expected: Self::CAPACITY as u64,
                        found: value as u64,
                    })
                }
            }
        }

        impl From<$SelfT> for $Uint {
            fn from(value: $SelfT) -> Self {
                value.0
            }
        }

        #[cfg(feature = "arbitrary")]
        mod arbitrary {
            use ::arbitrary::{Arbitrary, Unstructured, Result};
            use super::*;
            impl<'a> Arbitrary<'a> for $SelfT {
                fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                    Ok(Self::from_uint_unchecked(u.int_in_range(0..=Self::CAPACITY_MINUS_ONE)?))
                }
            }
        }

        #[cfg(feature = "rand")]
        mod rand {
            use super::*;
            use ::rand::{distr::{Distribution, StandardUniform}, Rng};

            impl Distribution<$SelfT> for StandardUniform {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $SelfT {
                    <$SelfT>::from_uint_lossy(rng.random())
                }
            }
            impl $SelfT {
                #[doc = concat!("Generate a new random ", stringify!($SelfT), ".")]
                ///
                /// This method generate a random ID.
                /// The generated ID is guaranteed to not be the [`NIL`](Self::NIL) value.
                ///
                /// # Examples
                ///
                /// ```
                /// # use mtid::*;
                #[doc = concat!("let id = ", stringify!($SelfT), "::random();")]
                #[doc = concat!("assert_ne!(id, ", stringify!($SelfT), "::NIL);")]
                /// ```
                pub fn random() -> Self {
                    <$SelfT>::from_uint_lossy(::rand::random())
                }
            }
        }
        #[cfg(feature = "serde")]
        mod serde {
            use super::*;
            use ::serde::{Deserialize, Serialize, de::Error};

            impl Serialize for $SelfT {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    serializer.serialize_str(&self.to_string())
                }
            }

            impl<'de> Deserialize<'de> for $SelfT {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    (&s).parse::<$SelfT>().map_err(|e| D::Error::custom(e))
                }
            }
        }
    };
}

macro_rules! mtid_bytes_impl {
    {
        Self = $SelfT:ty,
        Uint = $Uint:ty,
        LEN = $LEN:literal,
    } => {
        impl $SelfT {
            #[doc = concat!("Returns a byte array from ", stringify!($SelfT), ".")]
            pub fn to_bytes(self) -> [u8;$LEN] {
                let bytes = self.0.to_be_bytes();
                let start = bytes.len() - $LEN;
                self.0.to_be_bytes()[start..start+$LEN].try_into().unwrap()
            }
            fn bytes_to_uint(bytes: &[u8;$LEN]) -> $Uint {
                const LEN: usize = std::mem::size_of::<$Uint>();
                let mut buf = [0;LEN];
                let start = LEN - $LEN;
                buf[start..start+$LEN].copy_from_slice(bytes);
                <$Uint>::from_be_bytes(buf)
            }
            #[doc = concat!("Create new ", stringify!($SelfT), " from a byte array.")]
            pub fn from_bytes_lossy(bytes: &[u8;$LEN]) -> Self {
                Self::from_uint_lossy(Self::bytes_to_uint(bytes))
            }
        }
    };
}


macro_rules! mtid_prost_impl {
    {
        Self = $SelfT:ty,
        ActualT = $ActualT:ty,
        ProtoT = $ProtoT:ty,
        BITS = $BITS:literal,
        VALID_VALUE = $VALID_VALUE:literal,
        OVERSIZED_VALUE = $OVERSIZED_VALUE:literal,
    } => {
        mod prost {
            use crate::*;
            impl $SelfT {
                #[doc = concat!("Converts a [`", stringify!($ProtoT), "`]  to [`", stringify!($SelfT), "`] by truncating bits that exceed the valid range.")]
                ///
                /// This is a lossy conversion that masks the proto value to fit within the ID's bit limit.
                #[doc = concat!( "Since [`", stringify!($ProtoT), "`] is exposed via web APIs and cannot enforce range validation,")]
                /// this method safely handles any value by truncating excess bits.
                /// If you need to detect out-of-range values, use [`TryFrom`] instead.
                ///
                /// For details on the truncation behavior, see [`from_uint_lossy`](Self::from_uint_lossy).
                ///
                /// # Examples
                ///
                /// ```
                /// # use mtid::*;
                /// // Valid proto values are preserved
                #[doc = concat!("let proto = ", stringify!($ProtoT), "{ value: ", $VALID_VALUE, " }; //", stringify!($VALID_VALUE))]
                #[doc = concat!("let id = ", stringify!($SelfT), "::from_proto_lossy(proto);")]
                #[doc = concat!("assert_eq!(", stringify!($ActualT), "::from(id), ", $VALID_VALUE, ");")]
                ///
                /// // Out-of-range proto values are truncated
                #[doc = concat!("let proto = ", stringify!($ProtoT), "{ value: ", $OVERSIZED_VALUE, " }; //", stringify!($OVERSIZED_VALUE))]
                #[doc = concat!("let id = ", stringify!($SelfT), "::from_proto_lossy(proto);")]
                #[doc = concat!("assert_eq!(", stringify!($ActualT), "::from(id), ", $VALID_VALUE, "); // Only lower ", $BITS, " bits retained")]
                /// ```
                pub fn from_proto_lossy(proto: $ProtoT) -> Self {
                    Self::from_uint_lossy(proto.value as $ActualT)
                }
            }


            impl From<$SelfT> for $ProtoT {
                fn from(value: $SelfT) -> Self {
                    Self {
                        value: <$ActualT>::from(value).into(),
                    }
                }
            }

            impl TryFrom<$ProtoT> for $SelfT {
                type Error = crate::Error;
                #[doc = concat!("Attempts to convert a [`", stringify!($ProtoT), "`]  to [`", stringify!($SelfT), "`].")]
                ///
                /// Return error if its value is equal [`CAPACITY`](Self::CAPACITY) or more.
                /// If you don't need to detect out-of-range values, use [`from_proto_lossy`](Self::from_proto_lossy).
                ///
                /// # Examples
                ///
                /// ```
                /// # use mtid::*;
                #[doc = concat!("assert!(", stringify!($SelfT), "::try_from(", $VALID_VALUE, ").is_ok());")]
                #[doc = concat!("assert!(", stringify!($SelfT), "::try_from(", $OVERSIZED_VALUE, ").is_err());")]
                /// ```
                ///
                fn try_from(value: $ProtoT) -> Result<Self, Self::Error> {
                    Self::try_from(<$ActualT>::try_from(value.value).or(Err(Error::ParseInteger {
                        expected: u64::from(<$SelfT>::CAPACITY),
                        found: value.value as u64,
                    }))?)
                }
            }
        }
    };
}
pub(crate) use mtid_prost_impl;
pub(crate) use mtid_struct;
pub(crate) use mtid_impl;
pub(crate) use mtid_bytes_impl;
