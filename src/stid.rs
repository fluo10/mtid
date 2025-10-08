use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[cfg(feature="prost")]
use crate::StidMessage;
use crate::{utils::*, error::Error, macros::mtid_impl};

/// Single length Triplet ID.
/// 
/// # Examples
/// 
/// ```
/// use std::str::FromStr;
/// use mtid::Stid;
/// 
/// assert_eq!(Stid::from_str("012").unwrap(), Stid::try_from(34).unwrap());
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Stid(u16);

impl Stid {
    mtid_impl!{
        Self = Stid,
        ActualT = u16,
        BITS = 15,
        CAPACITY = CUBED_BASE,
        NIL_STR = "000",
        MAX_STR = "ZZZ",
    }
}

impl Display for Stid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = u16_to_chars(self.0);
        write!(f, "{}{}{}", chars.0, chars.1, chars.2)
    }
}

impl FromStr for Stid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len != 3 {
            return Err(Error::InvalidLength { expected: vec![3], found: len, raw: s.to_string() })
        }
        let chars: Vec<char> = s.chars().collect();
        Ok(Self(chars_to_u16((chars[0], chars[1], chars[2]))?))
    }
}

impl Distribution<Stid> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Stid {
        Stid(rng.gen_range(0..Stid::CAPACITY))
    }
}

impl TryFrom<u16> for Stid {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
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



impl From<Stid> for u16 {
    fn from(value: Stid) -> Self {
        value.0
    }
}

impl From<(Stid,)> for Stid {
    fn from(value: (Stid,)) -> Self {
        value.0
    }
}
impl From<Stid> for (Stid,) {
    fn from(value: Stid) -> Self {
        (value,)
    }
}

impl PartialEq<u16> for Stid {
    fn eq(&self, other: &u16) -> bool {
        &u16::from(*self) == other
    }
}

impl PartialEq<String> for Stid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}

