mod error;
pub use error::TripletError;

use crate::alphabet::*;
use core::str::Chars;

/// Block of 3 characters.
///
/// <div class="warning">
///
/// This structure should not be used directory; instead, [`CarettaIdS`](crate::CarettaIdS), [`CarettaIdD`](crate::CarettaIdD), [`CarettaIdT`](crate::CarettaIdT) and [`CarettaIdQ`](crate::CarettaIdQ) should be used.
///
/// </div>
///
/// This serves as a interface for converting between string and caretta-ids.
/// Internally, this is simple tuple struct of 3 charcters so:
///
/// - Not memory/storage efficient.
/// - The comparison is not implemented because it's value is not normarized.
///  
pub struct Triplet(char, char, char);

impl Triplet {
    pub const CAPACITY: u16 = (BASE as u16).pow(3);

    pub fn parse_chars(value: &mut Chars<'_>) -> Result<Self, TripletError> {
        Triplet::try_from((
            value.next().ok_or(TripletError::ParseLength(0))?,
            value.next().ok_or(TripletError::ParseLength(1))?,
            value.next().ok_or(TripletError::ParseLength(2))?,
        ))
    }
    pub fn from_uint_lossy(int: u16) -> Triplet {
        Self(
            u8_to_char_lossy((int >> 10) as u8),
            u8_to_char_lossy((int >> 5) as u8),
            u8_to_char_lossy(int as u8),
        )
    }
}

impl core::fmt::Display for Triplet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}
impl From<Triplet> for u16 {
    fn from(value: Triplet) -> Self {
        ((char_to_u8(value.0).unwrap() as u16) << 10)
            | ((char_to_u8(value.1).unwrap() as u16) << 5)
            | (char_to_u8(value.2).unwrap() as u16)
    }
}

impl TryFrom<(char, char, char)> for Triplet {
    type Error = TripletError;
    fn try_from(value: (char, char, char)) -> Result<Self, Self::Error> {
        Ok(Self(
            validate_char(value.0).ok_or(TripletError::ParseCharacter {
                character: value.0,
                index: 0,
            })?,
            validate_char(value.1).ok_or(TripletError::ParseCharacter {
                character: value.1,
                index: 1,
            })?,
            validate_char(value.2).ok_or(TripletError::ParseCharacter {
                character: value.2,
                index: 2,
            })?,
        ))
    }
}

impl From<Triplet> for (char, char, char) {
    fn from(value: Triplet) -> Self {
        (value.0, value.1, value.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_u8() {
        for i in 0..BASE {
            assert_eq!(i, char_to_u8(u8_to_char_lossy(i)).unwrap());
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn invalid_u8() {
        for _ in 0..BASE {
            let int = rand::random_range(BASE..=u8::MAX);
            assert_ne!(int, char_to_u8(u8_to_char_lossy(int)).unwrap());
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn valid_u16() {
        for _ in 0..BASE {
            let int = rand::random_range(0..Triplet::CAPACITY);
            assert_eq!(int, u16::try_from(Triplet::from_uint_lossy(int)).unwrap())
        }
    }
    #[test]
    #[cfg(feature = "std")]
    fn invalid_u16() {
        for _ in 0..BASE {
            let int = rand::random_range(Triplet::CAPACITY..=u16::MAX);
            assert_ne!(int, u16::try_from(Triplet::from_uint_lossy(int)).unwrap())
        }
    }
}
