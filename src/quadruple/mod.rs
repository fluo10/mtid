#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use crate::{Error, CarettaIdS, CarettaIdT, alphabet::is_delimiter, CarettaIdD, macros, triplet::Triplet};

use core::{fmt::Display, str::FromStr};
macros::caretta_id_struct! {
    Self = CarettaIdQ,
    ActualT = u64,
    description = "Quadruple length Caretta ID.",
    example_str = "kmn-pqr-stv-wxy",
    example_int = 707829019477668798,
}
macros::caretta_id_impl! {
    Self = CarettaIdQ,
    Uint = u64,
    BITS = 60,
    CAPACITY = (CarettaIdS::CAPACITY as u64).pow(4),
    NIL_STR = "000-000-000-000",
    MAX_STR = "zzz-zzz-zzz-zzz",
    MAX_INT = 1152921504606846975,
    EXAMPLE_VALID_INT = 0b0000_1101_1110_0000_1011_0110_1011_0011_1010_0111_0110_0100_0000_0000_0000_0000,
    EXAMPLE_OVERSIZED_INT = 0b1111_1101_1110_0000_1011_0110_1011_0011_1010_0111_0110_0100_0000_0000_0000_0000
}



impl Display for CarettaIdQ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let tuple: (CarettaIdS, CarettaIdS, CarettaIdS, CarettaIdS) = (*self).into();
        write!(f, "{}-{}-{}-{}", tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl FromStr for CarettaIdQ {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let len = s.len();
        match len {
            12 | 15 => {
                let has_delimiter = len == 15;
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
                if has_delimiter {
                    let delimiter = chars.next().unwrap();
                    if !is_delimiter(delimiter) {
                        return Err(Error::ParseDelimiter {
                            character: delimiter,
                            index: 2,
                        });
                    }
                }
                let fourth_triplet =
                    Triplet::parse_chars(&mut chars).map_err(|e| Error::ParseTriplet {
                        source: e,
                        index: 3,
                    })?;

                Ok(Self::from((
                    first_triplet,
                    second_triplet,
                    third_triplet,
                    fourth_triplet,
                )))
            }
            x => Err(Error::ParseLength {
                expected_without_delimiter: 9,
                expected_with_delimiter: Some(11),
                found: x,
            }),
        }
    }
}

impl From<(Triplet, Triplet, Triplet, Triplet)> for CarettaIdQ {
    fn from(value: (Triplet, Triplet, Triplet, Triplet)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << CarettaIdT::BITS)
                | ((u16::from(value.1) as u64) << CarettaIdD::BITS)
                | ((u16::from(value.2) as u64) << CarettaIdS::BITS)
                | (u16::from(value.3) as u64),
        )
    }
}

impl From<CarettaIdQ> for (CarettaIdS, CarettaIdS, CarettaIdS, CarettaIdS) {
    fn from(value: CarettaIdQ) -> Self {
        (
            CarettaIdS::from_uint_lossy((value.0 >> CarettaIdT::BITS) as u16),
            CarettaIdS::from_uint_lossy((value.0 >> CarettaIdD::BITS) as u16),
            CarettaIdS::from_uint_lossy((value.0 >> CarettaIdS::BITS) as u16),
            CarettaIdS::from_uint_lossy(value.0 as u16),
        )
    }
}

#[cfg(feature = "std")]
impl PartialEq<String> for CarettaIdQ {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}

#[cfg(feature = "prost")]
macros::caretta_id_prost_impl! {
    Self = CarettaIdQ,
    ActualT = u64,
    ProtoT = proto::CarettaIdQ,
    BITS = 45,
    VALID_VALUE = 0b0000_1101_1110_0000_1011_0110_1011_0011_1010_0111_0110_0100_0000_0000_0000_0000,
    OVERSIZED_VALUE = 0b1111_1101_1110_0000_1011_0110_1011_0011_1010_0111_0110_0100_0000_0000_0000_0000,
}

macros::caretta_id_redb! {
    Self = CarettaIdQ,
    Uint = u64,
    BYTES = 8,
}