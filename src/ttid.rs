use crate::{utils::is_delimiter, macros::mtid_impl, dtid::Dtid, Error, Stid};

use std::{fmt::Display, str::FromStr};

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let tuple: (Stid, Stid, Stid) = (*self).into();
        write!(f, "{}-{}-{}", tuple.0, tuple.1, tuple.2)
    }
}

impl FromStr for Ttid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            11 => {
                let delimiter = [
                    s[3..4].chars().next().unwrap(),
                    s[7..8].chars().next().unwrap(),
                ];
                if is_delimiter(delimiter[0]) && is_delimiter(delimiter[1]) {
                    Ok(Self::from((Stid::from_str(&s[0..3])?,Stid::from_str(&s[4..7])?,Stid::from_str(&s[8..11])?)))
                } else {
                    Err(Error::InvalidDelimiter{
                        found: Vec::from(delimiter),
                        raw: s.to_string()
                    })
                }

            }
            9 => {
                Ok(Self::from((Stid::from_str(&s[0..3])?,Stid::from_str(&s[3..6])?,Stid::from_str(&s[6..9])?)))
            }
            x => {
                Err(Self::Err::InvalidLength{
                    expected: vec![9, 11],
                    found: x,
                    raw: s.to_string()
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
            Err(Error::OutsideOfRange{
                expected: Self::CAPACITY,
                found: value
            })
        }
    }
}

impl From<Ttid> for u64 {
    fn from(value: Ttid) -> Self {
        value.0
    }
}

impl From<(Stid, Stid, Stid)> for Ttid {
    fn from(value: (Stid, Stid, Stid)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << Dtid::BITS)
                | ((u16::from(value.1) as u64) << Stid::BITS) 
                | (u16::from(value.2) as u64)
        )
    }
}

impl From<Ttid> for (Stid, Stid, Stid) {
    fn from(value: Ttid) -> Self {
        (
            Stid::from_int_lossy((value.0 >> Dtid::BITS) as u16),
            Stid::from_int_lossy((value.0 >> Stid::BITS) as u16),
            Stid::from_int_lossy(value.0 as u16)
        )
    }
}

impl PartialEq<u64> for Ttid {
    fn eq(&self, other: &u64) -> bool {
        &u64::from(*self) == other
    }
}

impl PartialEq<String> for Ttid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}
