#[cfg(feature="prost")]
use crate::TripleMessage;
use crate::{common::is_delimiter, macros::tripod_id_impl, double::Double, Error, single::Single};

use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

/// Triple length tripod id.
/// 
/// # Examples 
/// ```
/// # use tripod_id::Triple;
/// # use std::str::FromStr;
/// 
/// let _ = Triple::from_str("012-abc-def");
/// ``` 
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Triple(u64);

impl Triple {
    tripod_id_impl!{
        Self = Triple,
        ActualT = u64,
        BITS = 45,
        CAPACITY = (Single::CAPACITY as u64).pow(3),
        NIL_STR = "000-000-000",
        MAX_STR = "zzz-zzz-zzz",
    }
}

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let tuple: (Single, Single, Single) = (*self).into();
        write!(f, "{}-{}-{}", tuple.0, tuple.1, tuple.2)
    }
}

impl FromStr for Triple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.len() {
                11 => {
                    let delimiter = [
                        s[3..4].chars().next().unwrap(),
                        s[7..8].chars().next().unwrap(),
                    ];
                    if is_delimiter(delimiter[0]) && is_delimiter(delimiter[1]) {
                        Ok(Self::from((Single::from_str(&s[0..3])?,Single::from_str(&s[4..7])?,Single::from_str(&s[8..11])?)))
                    } else {
                        Err(Error::InvalidDelimiter{
                            found: Vec::from(delimiter),
                            raw: s.to_string()
                        })
                    }

                }
                9 => {
                    Ok(Self::from((Single::from_str(&s[0..3])?,Single::from_str(&s[3..6])?,Single::from_str(&s[6..9])?)))
                }
                x => {
                    Err(Self::Err::InvalidLength{
                        expected: vec![9, 11],
                        found: x,
                        raw: s.to_string()
                    })
                }
            } ?
        )
    }
}


impl Distribution<Triple> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triple {
        Triple(rng.gen_range(0..Triple::CAPACITY))

    }
}

impl TryFrom<u64> for Triple {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
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

impl From<Triple> for u64 {
    fn from(value: Triple) -> Self {
        value.0
    }
}

impl From<(Single, Single, Single)> for Triple {
    fn from(value: (Single, Single, Single)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << Double::BITS)
                | ((u16::from(value.1) as u64) << Single::BITS) 
                | (u16::from(value.2) as u64)
        )
    }
}

impl From<Triple> for (Single, Single, Single) {
    fn from(value: Triple) -> Self {
        (
            Single::from_int_lossy((value.0 >> Double::BITS) as u16),
            Single::from_int_lossy((value.0 >> Single::BITS) as u16),
            Single::from_int_lossy(value.0 as u16)
        )
    }
}

impl PartialEq<u64> for Triple {
    fn eq(&self, other: &u64) -> bool {
        &u64::from(*self) == other
    }
}

impl PartialEq<String> for Triple {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}
