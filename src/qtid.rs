#[cfg(feature="prost")]
use crate::QtidMessage;
use crate::{dtid::Dtid, macros::mtid_impl, utils::is_delimiter, Error, Stid, Ttid};

use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

mtid_impl!{
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let tuple: (Stid, Stid, Stid, Stid) = (*self).into();
        write!(f, "{}-{}-{}-{}", tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl FromStr for Qtid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        
        Ok(match chars.len() {
                15 => {

                    let delimiters = (
                        chars[3],
                        chars[7],
                        chars[11]
                    );
                    if is_delimiter(delimiters.0) 
                        && is_delimiter(delimiters.1)
                        && is_delimiter(delimiters.2) {
                        Ok(Self::from((
                            Stid::try_from((chars[0], chars[1], chars[2]))?,
                            Stid::try_from((chars[4], chars[5], chars[6]))?,
                            Stid::try_from((chars[8], chars[9], chars[10]))?,
                            Stid::try_from((chars[12], chars[13], chars[14]))?
                        )))
                    } else {
                        Err(Error::InvalidDelimiter{
                            found: vec![delimiters.0,delimiters.1, delimiters.2],
                            raw: s.to_string()
                        })
                    }

                }
                12 => {
                    Ok(Self::from((
                        Stid::try_from((chars[0], chars[1], chars[2]))?,
                        Stid::try_from((chars[3], chars[4], chars[5]))?,
                        Stid::try_from((chars[6], chars[7], chars[8]))?,
                        Stid::try_from((chars[9], chars[10], chars[11]))?
                    )))
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


impl Distribution<Qtid> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Qtid {
        Qtid(rng.gen_range(0..Qtid::CAPACITY))

    }
}

impl TryFrom<u64> for Qtid {
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

impl From<Qtid> for u64 {
    fn from(value: Qtid) -> Self {
        value.0
    }
}

impl From<(Stid, Stid, Stid, Stid)> for Qtid {
    fn from(value: (Stid, Stid, Stid, Stid)) -> Self {
        Self(
            ((u16::from(value.0) as u64) << Ttid::BITS)
                | ((u16::from(value.1) as u64) << Dtid::BITS)
                | ((u16::from(value.2) as u64) << Stid::BITS) 
                | (u16::from(value.3) as u64)
        )
    }
}

impl From<Qtid> for (Stid, Stid, Stid, Stid) {
    fn from(value: Qtid) -> Self {
        (
            Stid::from_int_lossy((value.0 >> Ttid::BITS) as u16),
            Stid::from_int_lossy((value.0 >> Dtid::BITS) as u16),
            Stid::from_int_lossy((value.0 >> Stid::BITS) as u16),
            Stid::from_int_lossy(value.0 as u16)
        )
    }
}

impl PartialEq<u64> for Qtid {
    fn eq(&self, other: &u64) -> bool {
        &u64::from(*self) == other
    }
}

impl PartialEq<String> for Qtid {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}
