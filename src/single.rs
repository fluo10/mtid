use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[cfg(feature="prost")]
use crate::SingleMessage;
use crate::{common::*, error::Error, macros::tripod_id_impl};

/// Single size tripod id.
/// 
/// # Examples
/// 
/// ```
/// use std::str::FromStr;
/// use tripod_id::Single;
/// 
/// assert_eq!(Single::from_str("012").unwrap(), Single::try_from(34).unwrap());
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Single(u16);

impl Single {
    tripod_id_impl!{
        Self = Single,
        ActualT = u16,
        BITS = 15,
        CAPACITY = CUBED_BASE,
        NIL_STR = "000",
        MAX_STR = "ZZZ",
    }
}

impl Display for Single {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = u16_to_chars(self.0);
        write!(f, "{}{}{}", chars.0, chars.1, chars.2)
    }
}

impl FromStr for Single {
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

impl Distribution<Single> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Single {
        Single(rng.gen_range(0..Single::CAPACITY))
    }
}

impl TryFrom<u16> for Single {
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



impl From<Single> for u16 {
    fn from(value: Single) -> Self {
        value.0
    }
}

impl From<(Single,)> for Single {
    fn from(value: (Single,)) -> Self {
        value.0
    }
}
impl From<Single> for (Single,) {
    fn from(value: Single) -> Self {
        (value,)
    }
}

impl PartialEq<u16> for Single {
    fn eq(&self, other: &u16) -> bool {
        &u16::from(*self) == other
    }
}

impl PartialEq<String> for Single {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}

