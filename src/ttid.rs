use crate::{dtid::Dtid, macros::mtid_impl, utils::{is_delimiter, Triplet}, Error, Stid};

use core::{fmt::Display, str::FromStr};

mtid_impl!{
    Self = Ttid,
    ActualT = u64,
    BITS = 45,
    CAPACITY = (Stid::CAPACITY as u64).pow(3),
    NIL_STR = "000-000-000",
    MAX_STR = "zzz-zzz-zzz",
    MAX_INT = 35184372088831,
    description = "Triple length Triplet ID",
    example_str = "abc-def-ghj",
    example_int = 11386409697842
}


impl Display for Ttid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        
        let tuple: (Triplet, Triplet, Triplet) = (*self).into();
        write!(f, "{}-{}-{}", tuple.0, tuple.1, tuple.2)
    }
}

impl FromStr for Ttid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let len = s.len();
        match len {
            9 | 11 => {
                let has_delimiter = len == 11;
                let first_triplet = Triplet::parse_chars(&mut chars).map_err(|e| {
                    Error::ParseTriplet { source: e, index: 0 }
                })?;
                if has_delimiter {
                    let delimiter = chars.next().unwrap();
                    if !is_delimiter(delimiter) {
                        return Err(Error::ParseDelimiter { character: delimiter, index: 0 })
                    }
                }
                let second_triplet = Triplet::parse_chars(&mut chars).map_err(|e| {
                    Error::ParseTriplet { source: e, index: 1 }
                })?;
                if has_delimiter {
                    let delimiter = chars.next().unwrap();
                    if !is_delimiter(delimiter) {
                        return Err(Error::ParseDelimiter { character: delimiter, index: 1 })
                    }
                }
                let third_triplet = Triplet::parse_chars(&mut chars).map_err(|e| {
                    Error::ParseTriplet { source: e, index: 2 }
                })?;

                Ok(Self::from((first_triplet, second_triplet, third_triplet)))
            }
            x => {
                Err(Error::ParseLength{
                    expected_without_delimiter: 9,
                    expected_with_delimiter: Some(11),
                    found: x,
                })
            }
        }
    }
}

impl TryFrom<u64> for Ttid {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self(value))
        } else {
            Err(Error::ParseInteger { expected: Self::CAPACITY, found: value })
        }
    }
}

impl From<Ttid> for u64 {
    fn from(value: Ttid) -> Self {
        value.0
    }
}

impl From<(Triplet, Triplet, Triplet)> for Ttid {
    fn from(value: (Triplet, Triplet, Triplet)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << Dtid::BITS)
                | ((u16::from(value.1) as u64) << Stid::BITS) 
                | (u16::from(value.2) as u64)
        )
    }
}

impl From<Ttid> for (Triplet, Triplet, Triplet) {
    fn from(value: Ttid) -> Self {
        (
            Triplet::from_int_lossy((value.0 >> Dtid::BITS) as u16),
            Triplet::from_int_lossy((value.0 >> Stid::BITS) as u16),
            Triplet::from_int_lossy(value.0 as u16)
        )
    }
}

impl PartialEq<u64> for Ttid {
    fn eq(&self, other: &u64) -> bool {
        &u64::from(*self) == other
    }
}

#[cfg(feature="std")]
impl PartialEq<String> for Ttid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}
