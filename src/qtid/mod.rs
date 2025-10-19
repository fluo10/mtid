#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use crate::{
    Error, Stid, Ttid, alphabet::is_delimiter, dtid::Dtid, macros::mtid_impl, triplet::Triplet,
};

use core::{fmt::Display, str::FromStr};

mtid_impl! {
    Self = Qtid,
    ActualT = u64,
    BITS = 60,
    CAPACITY = (Stid::CAPACITY as u64).pow(4),
    NIL_STR = "000-000-000-000",
    MAX_STR = "zzz-zzz-zzz-zzz",
    MAX_INT = 1152921504606846975,
    description = "Quadruple length Triplet ID.",
    example_str = "kmn-pqr-stv-wxy",
    example_int = 707829019477668798
}

impl Display for Qtid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let tuple: (Stid, Stid, Stid, Stid) = (*self).into();
        write!(f, "{}-{}-{}-{}", tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl FromStr for Qtid {
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

impl TryFrom<u64> for Qtid {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self(value))
        } else {
            Err(Error::ParseInteger {
                expected: Self::CAPACITY,
                found: value,
            })
        }
    }
}

impl From<Qtid> for u64 {
    fn from(value: Qtid) -> Self {
        value.0
    }
}

impl From<(Triplet, Triplet, Triplet, Triplet)> for Qtid {
    fn from(value: (Triplet, Triplet, Triplet, Triplet)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << Ttid::BITS)
                | ((u16::from(value.1) as u64) << Dtid::BITS)
                | ((u16::from(value.2) as u64) << Stid::BITS)
                | (u16::from(value.3) as u64),
        )
    }
}

impl From<Qtid> for (Stid, Stid, Stid, Stid) {
    fn from(value: Qtid) -> Self {
        (
            Stid::from_int_lossy((value.0 >> Ttid::BITS) as u16),
            Stid::from_int_lossy((value.0 >> Dtid::BITS) as u16),
            Stid::from_int_lossy((value.0 >> Stid::BITS) as u16),
            Stid::from_int_lossy(value.0 as u16),
        )
    }
}

impl PartialEq<u64> for Qtid {
    fn eq(&self, other: &u64) -> bool {
        &u64::from(*self) == other
    }
}

#[cfg(feature = "std")]
impl PartialEq<String> for Qtid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}
