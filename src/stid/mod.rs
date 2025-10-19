#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use core::{fmt::Display, str::FromStr};

use crate::{error::Error, macros::mtid_impl, triplet::Triplet};

mtid_impl! {
    Self = Stid,
    ActualT = u16,
    BITS = 15,
    CAPACITY = Triplet::CAPACITY,
    NIL_STR = "000",
    MAX_STR = "zzz",
    MAX_INT = 32767,
    description = "Single length Triplet ID.",
    example_str = "123",
    example_int = 1091
}

impl Display for Stid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let triplet = Triplet::from(*self);
        triplet.fmt(f)
    }
}

impl From<Stid> for Triplet {
    fn from(value: Stid) -> Self {
        Triplet::from_int_lossy(value.0)
    }
}

impl From<Triplet> for Stid {
    fn from(value: Triplet) -> Self {
        Self::from_int_lossy(u16::from(value))
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

impl TryFrom<u16> for Stid {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self(value))
        } else {
            Err(Error::ParseInteger {
                expected: Self::CAPACITY as u64,
                found: value as u64,
            })
        }
    }
}

impl From<Stid> for u16 {
    fn from(value: Stid) -> Self {
        value.0
    }
}

impl PartialEq<u16> for Stid {
    fn eq(&self, other: &u16) -> bool {
        &u16::from(*self) == other
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
