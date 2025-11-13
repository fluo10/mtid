#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use core::{fmt::Display, str::FromStr};

use crate::{CarettaIdS, Error, alphabet::is_delimiter, macros, triplet::Triplet};

macros::caretta_id_struct! {
    Self = CarettaIdD,
    ActualT = u32,
    description = "Double length Caretta ID",
    example_str = "456-789",
    example_int = 139664649,
}

macros::caretta_id_impl! {
    Self = CarettaIdD,
    Uint = u32,
    BITS = 30,
    CAPACITY = (CarettaIdS::CAPACITY as u32).pow(2),
    NIL_STR = "000-000",
    MAX_STR = "zzz-zzz",
    MAX_INT = 1073741823,
    EXAMPLE_VALID_INT = 0b0011_1011_1001_1010_1100_1010_0000_0000,
    EXAMPLE_OVERSIZED_INT = 0b1111_1011_1001_1010_1100_1010_0000_0000
}

impl Display for CarettaIdD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let tuple: (Triplet, Triplet) = (*self).into();
        write!(f, "{}-{}", tuple.0, tuple.1)
    }
}

impl From<(Triplet, Triplet)> for CarettaIdD {
    fn from(value: (Triplet, Triplet)) -> Self {
        Self((u32::from(u16::from(value.0)) << CarettaIdS::BITS) + u32::from(u16::from(value.1)))
    }
}

impl From<CarettaIdD> for (Triplet, Triplet) {
    fn from(value: CarettaIdD) -> Self {
        (
            Triplet::from_uint_lossy((value.0 >> CarettaIdS::BITS) as u16),
            Triplet::from_uint_lossy(value.0 as u16),
        )
    }
}

impl FromStr for CarettaIdD {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let len = s.len();
        Ok(match len {
            7 | 6 => {
                let first_triplet =
                    Triplet::parse_chars(&mut chars).map_err(|e| Error::ParseTriplet {
                        source: e,
                        index: 0,
                    })?;
                if len == 7 {
                    let delimiter = chars.next().unwrap();
                    if !is_delimiter(delimiter) {
                        return Err(Error::ParseDelimiter {
                            character: delimiter,
                            index: 3,
                        });
                    }
                }
                let second_triplet =
                    Triplet::parse_chars(&mut chars).map_err(|e| Error::ParseTriplet {
                        source: e,
                        index: 1,
                    })?;
                Self::from((first_triplet, second_triplet))
            }
            x => {
                return Err(Error::ParseLength {
                    expected_without_delimiter: 6,
                    expected_with_delimiter: Some(7),
                    found: x,
                });
            }
        })
    }
}

#[cfg(feature = "std")]
impl PartialEq<String> for CarettaIdD {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}

#[cfg(feature = "prost")]
crate::macros::caretta_id_prost_impl! {
    Self = CarettaIdD,
    ActualT = u32,
    ProtoT = proto::CarettaIdD,
    BITS = 30,
    VALID_VALUE = 0b0011_1011_1001_1010_1100_1010_0000_0000,
    OVERSIZED_VALUE = 0b1111_1011_1001_1010_1100_1010_0000_0000,
}

macros::caretta_id_redb! {
    Self = CarettaIdD,
    Uint = u32,
    BYTES = 4,
}
