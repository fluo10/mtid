#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use core::{fmt::Display, str::FromStr};

use crate::{Error, Stid, alphabet::is_delimiter, macros::mtid_impl, triplet::Triplet};

mtid_impl! {
    Self = Dtid,
    ActualT = u32,
    BITS = 30,
    CAPACITY = (Stid::CAPACITY as u32).pow(2),
    NIL_STR = "000-000",
    MAX_STR = "zzz-zzz",
    MAX_INT = 1073741823,
    description = "Double length Triplet ID",
    example_str = "456-789",
    example_int = 139664649
}

impl Display for Dtid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let tuple: (Triplet, Triplet) = (*self).into();
        write!(f, "{}-{}", tuple.0, tuple.1)
    }
}

impl From<(Triplet, Triplet)> for Dtid {
    fn from(value: (Triplet, Triplet)) -> Self {
        Self((u32::from(u16::from(value.0)) << Stid::BITS) + u32::from(u16::from(value.1)))
    }
}

impl From<Dtid> for (Triplet, Triplet) {
    fn from(value: Dtid) -> Self {
        (
            Triplet::from_int_lossy((value.0 >> Stid::BITS) as u16),
            Triplet::from_int_lossy(value.0 as u16),
        )
    }
}

impl FromStr for Dtid {
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

impl TryFrom<u32> for Dtid {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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

impl From<Dtid> for u32 {
    fn from(value: Dtid) -> Self {
        value.0
    }
}

impl PartialEq<u32> for Dtid {
    fn eq(&self, other: &u32) -> bool {
        &u32::from(*self) == other
    }
}

#[cfg(feature = "std")]
impl PartialEq<String> for Dtid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}
