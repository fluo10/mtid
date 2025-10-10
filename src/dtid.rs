use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[cfg(feature="prost")]
use crate::DtidMessage;
use crate::{utils::{is_delimiter, CUBED_BASE}, macros::mtid_impl, Error, Stid,};

/// Double length Triplet ID.
/// 
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Dtid(u32);

impl Dtid {
    mtid_impl!{
        Self = Dtid,
        ActualT = u32,
        BITS = 30,
        CAPACITY = (Stid::CAPACITY as u32).pow(2),
        NIL_STR = "000-000",
        MAX_STR = "zzz-zzz",
        MAX_INT = 1073741823,
    }
}

impl Display for Dtid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuple: (Stid, Stid) = (*self).into();
        write!(f, "{}-{}", tuple.0, tuple.1)
    }
}

impl From<(Stid, Stid)> for Dtid {
    fn from(value: (Stid, Stid)) -> Self {
        Self((u32::from(u16::from(value.0)) << Stid::BITS)+ u32::from(u16::from(value.1)))
    }
} 

impl From<Dtid> for (Stid, Stid) {
    fn from(value: Dtid) -> Self {
        (
            Stid::from_int_lossy((value.0 >> Stid::BITS) as u16),
            Stid::from_int_lossy(value.0 as u16)
        )
    }
}

impl FromStr for Dtid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let tuple = match s.len() {
            7 => {
                let delimiter = s[3..4].chars().next().unwrap();
                if is_delimiter(delimiter) {
                    Ok((Stid::from_str(&s[0..3])?,Stid::from_str(&s[4..7])?))
                } else {
                    Err(Error::InvalidDelimiter{
                        found: vec![delimiter],
                        raw: s.to_string()
                    })
                }
                
            },
            6 => {
                Ok((Stid::from_str(&s[0..3])?,Stid::from_str(&s[3..6])?))
            },
            x => { 
                Err(Error::InvalidLength{
                    expected: vec![6, 7],
                    found: x,
                    raw: s.to_string()
                })
            }
        }?;
        Ok(Self::from(tuple))
    }
}


impl Distribution<Dtid> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dtid {
        Dtid(rng.gen_range(0..Dtid::CAPACITY))

    }
}

impl TryFrom<u32> for Dtid {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self(value))
        } else {
            Err(Error::OutsideOfRange{
                expected: Self::CAPACITY as u64,
                found: value as u64
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

impl PartialEq<String> for Dtid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}