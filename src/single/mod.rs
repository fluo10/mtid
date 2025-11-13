#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use core::{fmt::Display, str::FromStr};

use crate::{error::Error, macros, triplet::Triplet};

crate::macros::caretta_id_struct! {
    Self = CarettaIdS,
    ActualT = u16,
    description = "Single length Caretta ID.",
    example_str = "123",
    example_int = 1091,
}

crate::macros::caretta_id_impl! {
    Self = CarettaIdS,
    Uint = u16,
    BITS = 15,
    CAPACITY = Triplet::CAPACITY,
    NIL_STR = "000",
    MAX_STR = "zzz",
    MAX_INT = 32767,
    EXAMPLE_VALID_INT = 0b0010_0111_0001_0000,
    EXAMPLE_OVERSIZED_INT = 0b1010_0111_0001_0000
}

impl Display for CarettaIdS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let triplet = Triplet::from(*self);
        triplet.fmt(f)
    }
}

impl From<CarettaIdS> for Triplet {
    fn from(value: CarettaIdS) -> Self {
        Triplet::from_uint_lossy(value.0)
    }
}

impl From<Triplet> for CarettaIdS {
    fn from(value: Triplet) -> Self {
        Self::from_uint_lossy(u16::from(value))
    }
}

impl FromStr for CarettaIdS {
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
impl PartialEq<String> for CarettaIdS {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}

#[cfg(feature = "prost")]
macros::caretta_id_prost_impl! {
    Self = CarettaIdS,
    ActualT = u16,
    ProtoT = proto::CarettaIdS,
    BITS = 15,
    VALID_VALUE = 0b0010_0111_0001_0000,
    OVERSIZED_VALUE = 0b1010_0111_0001_0000,
}

crate::macros::caretta_id_redb! {
    Self = CarettaIdS,
    Uint = u16,
    BYTES = 2,
}
