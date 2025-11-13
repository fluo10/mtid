#[cfg(feature = "rusqlite")]
mod rusqlite;
#[cfg(feature = "sea-orm")]
mod sea_orm;

use crate::{CarettaIdD, CarettaIdS, Error, alphabet::is_delimiter, macros, triplet::Triplet};

use core::{fmt::Display, str::FromStr};

crate::macros::caretta_id_struct! {
    Self = CarettaIdT,
    ActualT = u64,
    description = "Triple length Caretta ID",
    example_str = "abc-def-ghj",
    example_int = 11386409697842,
}
crate::macros::caretta_id_impl! {
    Self = CarettaIdT,
    Uint = u64,
    BITS = 45,
    CAPACITY = (CarettaIdS::CAPACITY as u64).pow(3),
    NIL_STR = "000-000-000",
    MAX_STR = "zzz-zzz-zzz",
    MAX_INT = 35184372088831,
    EXAMPLE_VALID_INT = 0b0000_0000_0000_0000_0000_1001_0001_1000_0100_1110_0111_0010_1010_0000_0000_0000,
    EXAMPLE_OVERSIZED_INT = 0b1111_1111_1111_1111_1110_1001_0001_1000_0100_1110_0111_0010_1010_0000_0000_0000
}

impl Display for CarettaIdT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let tuple: (Triplet, Triplet, Triplet) = (*self).into();
        write!(f, "{}-{}-{}", tuple.0, tuple.1, tuple.2)
    }
}

impl FromStr for CarettaIdT {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let len = s.len();
        match len {
            9 | 11 => {
                let has_delimiter = len == 11;
                let first_triplet =
                    Triplet::parse_chars(&mut chars).map_err(|e| Error::ParseTriplet {
                        source: e,
                        index: 0,
                    })?;
                if has_delimiter {
                    let delimiter = chars.next().unwrap();
                    if !is_delimiter(delimiter) {
                        return Err(Error::ParseDelimiter {
                            character: delimiter,
                            index: 0,
                        });
                    }
                }
                let second_triplet =
                    Triplet::parse_chars(&mut chars).map_err(|e| Error::ParseTriplet {
                        source: e,
                        index: 1,
                    })?;
                if has_delimiter {
                    let delimiter = chars.next().unwrap();
                    if !is_delimiter(delimiter) {
                        return Err(Error::ParseDelimiter {
                            character: delimiter,
                            index: 1,
                        });
                    }
                }
                let third_triplet =
                    Triplet::parse_chars(&mut chars).map_err(|e| Error::ParseTriplet {
                        source: e,
                        index: 2,
                    })?;

                Ok(Self::from((first_triplet, second_triplet, third_triplet)))
            }
            x => Err(Error::ParseLength {
                expected_without_delimiter: 9,
                expected_with_delimiter: Some(11),
                found: x,
            }),
        }
    }
}

impl From<(Triplet, Triplet, Triplet)> for CarettaIdT {
    fn from(value: (Triplet, Triplet, Triplet)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << CarettaIdD::BITS)
                | ((u16::from(value.1) as u64) << CarettaIdS::BITS)
                | (u16::from(value.2) as u64),
        )
    }
}

impl From<CarettaIdT> for (Triplet, Triplet, Triplet) {
    fn from(value: CarettaIdT) -> Self {
        (
            Triplet::from_uint_lossy((value.0 >> CarettaIdD::BITS) as u16),
            Triplet::from_uint_lossy((value.0 >> CarettaIdS::BITS) as u16),
            Triplet::from_uint_lossy(value.0 as u16),
        )
    }
}

#[cfg(feature = "std")]
impl PartialEq<String> for CarettaIdT {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}

#[cfg(feature = "prost")]
macros::caretta_id_prost_impl! {
    Self = CarettaIdT,
    ActualT = u64,
    ProtoT = proto::CarettaIdT,
    BITS = 45,
    VALID_VALUE = 0b0000_0000_0000_0000_0000_1001_0001_1000_0100_1110_0111_0010_1010_0000_0000_0000,
    OVERSIZED_VALUE = 0b1111_1111_1111_1111_1110_1001_0001_1000_0100_1110_0111_0010_1010_0000_0000_0000,
}

crate::macros::caretta_id_redb! {
    Self = CarettaIdT,
    Uint = u64,
    BYTES = 6,
    uint_to_bytes = const fn uint_to_bytes(uint: u64) -> [u8;6] {
        let bytes = uint.to_le_bytes();
        [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]]
    },
    bytes_to_uint = const fn bytes_to_uint(bytes: &[u8;6]) -> u64 {
        u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], 0, 0])
    },
}
