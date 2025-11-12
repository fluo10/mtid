#[cfg(feature = "rusqlite")]
mod rusqlite;

#[cfg(feature = "sea-orm")]
mod sea_orm;

use core::{fmt::Display, str::FromStr};

use crate::{Error, Stid, alphabet::is_delimiter, macros, triplet::Triplet};

macros::mtid_struct! {
    Self = Dtid,
    ActualT = u32,
    description = "Double length Triplet ID",
    example_str = "456-789",
    example_int = 139664649,
}

macros::mtid_impl! {
    Self = Dtid,
    Uint = u32,
    BITS = 30,
    CAPACITY = (Stid::CAPACITY as u32).pow(2),
    NIL_STR = "000-000",
    MAX_STR = "zzz-zzz",
    MAX_INT = 1073741823,
    EXAMPLE_VALID_INT = 0b0011_1011_1001_1010_1100_1010_0000_0000,
    EXAMPLE_OVERSIZED_INT = 0b1111_1011_1001_1010_1100_1010_0000_0000
}

macros::mtid_bytes_impl! {
    Self = Dtid,
    Uint = u32,
    LEN = 4,
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
            Triplet::from_uint_lossy((value.0 >> Stid::BITS) as u16),
            Triplet::from_uint_lossy(value.0 as u16),
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

#[cfg(feature = "std")]
impl PartialEq<String> for Dtid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false,
        }
    }
}

#[cfg(feature = "prost")]
crate::macros::mtid_prost_impl! {
    Self = Dtid,
    ActualT = u32,
    ProtoT = proto::Dtid,
    BITS = 30,
    VALID_VALUE = 0b0011_1011_1001_1010_1100_1010_0000_0000,
    OVERSIZED_VALUE = 0b1111_1011_1001_1010_1100_1010_0000_0000,
}
