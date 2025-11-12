#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use core::{fmt::Display, str::FromStr};

#[cfg(feature = "prost")]
use crate::macros;
use crate::{error::Error, triplet::Triplet};

crate::macros::mtid_struct! {
    Self = Stid,
    ActualT = u16,
    description = "Single length Triplet ID.",
    example_str = "123",
    example_int = 1091,
}

crate::macros::mtid_impl! {
    Self = Stid,
    Uint = u16,
    BITS = 15,
    CAPACITY = Triplet::CAPACITY,
    NIL_STR = "000",
    MAX_STR = "zzz",
    MAX_INT = 32767,
    EXAMPLE_VALID_INT = 0b0010_0111_0001_0000,
    EXAMPLE_OVERSIZED_INT = 0b1010_0111_0001_0000
}

crate::macros::mtid_bytes_impl! {
    Self = Stid,
    Uint = u16,
    LEN = 2,
}

impl Display for Stid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let triplet = Triplet::from(*self);
        triplet.fmt(f)
    }
}

impl From<Stid> for Triplet {
    fn from(value: Stid) -> Self {
        Triplet::from_uint_lossy(value.0)
    }
}

impl From<Triplet> for Stid {
    fn from(value: Triplet) -> Self {
        Self::from_uint_lossy(u16::from(value))
    }
}

impl FromStr for Stid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len != 3 {
            return Err(Error::ParseLength {
                expected_without_delimiter: 3,
                expected_with_delimiter: None,
                found: len,
            });
        }
        let mut chars = s.chars();
        Ok(Self(u16::from(Triplet::parse_chars(&mut chars).map_err(
            |e| Error::ParseTriplet {
                source: e,
                index: 0,
            },
        )?)))
    }
}

#[cfg(feature = "std")]
impl PartialEq<String> for Stid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}

#[cfg(feature = "prost")]
macros::mtid_prost_impl! {
    Self = Stid,
    ActualT = u16,
    ProtoT = proto::Stid,
    BITS = 15,
    VALID_VALUE = 0b0010_0111_0001_0000,
    OVERSIZED_VALUE = 0b1010_0111_0001_0000,
}
