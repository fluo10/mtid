use core::str::Chars;

use crate::{error::TripletError};

/// Triplet Block.
/// 
/// Internally, this is tuple struct of 3 chars.
pub struct Triplet(char, char, char);


const ENCODE_CHARACTERS: &[char;32] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'
    ];
pub const BASE: u8 = ENCODE_CHARACTERS.len() as u8;
pub const CUBED_BASE: u16 = (BASE as u16).pow(3);

// Encode u8 to char.
// First 1 bit of arg is ignored.
pub fn u8_to_char(value: u8) -> char {
    let value = value & 0b00011111;
    ENCODE_CHARACTERS[usize::from(value)]
}

/// Check char is valid.
/// If valid return Some(char) and else return None. 
fn validate_char(c: char) -> Option<char> {
    char_to_u8(c).map(|_| c)
}

// Decode char to u8
fn char_to_u8(c: char) -> Option<u8> {
    Some(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        'g' => 16,
        'h' => 17,
        'i' => 1,
        'j' => 18,
        'k' => 19,
        'l' => 1,
        'm' => 20,
        'n' => 21,
        'o' => 0,
        'p' => 22,
        'q' => 23,
        'r' => 24,
        's' => 25,
        't' => 26,
        'u' => 27,
        'v' => 27,
        'w' => 28,
        'x' => 29,
        'y' => 30,
        'z' => 31,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'I' => 1,
        'J' => 18,
        'K' => 19,
        'L' => 1,
        'M' => 20,
        'N' => 21,
        'O' => 0,
        'P' => 22,
        'Q' => 23,
        'R' => 24,
        'S' => 25,
        'T' => 26,
        'U' => 27,
        'V' => 27,
        'W' => 28,
        'X' => 29,
        'Y' => 30,
        'Z' => 31,
        _ => return None
    })
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
            validate_char(value.0).ok_or(TripletError::ParseCharacter { character: value.0, index: 0 })?,
            validate_char(value.1).ok_or(TripletError::ParseCharacter { character: value.1, index: 1 })?,
            validate_char(value.2).ok_or(TripletError::ParseCharacter { character: value.2, index: 2 })?,
        ))
    }
}

impl From<Triplet> for (char, char, char) {
    fn from(value: Triplet) -> Self {
        (value.0, value.1, value.2)
    }
}

impl Triplet {
    pub fn parse_chars(value: &mut Chars<'_>) -> Result<Self, TripletError> {
        Triplet::try_from((
            value.next().ok_or(TripletError::ParseLength(0))?,
            value.next().ok_or(TripletError::ParseLength(1))?,
            value.next().ok_or(TripletError::ParseLength(2))?
        ))
    }
    pub fn from_int_lossy(int: u16) -> Triplet {
        Self(
            u8_to_char((int >> 10) as u8),
            u8_to_char((int >> 5) as u8),
            u8_to_char(int as u8)
        )
    }
}

/// Test if the character is valid delimiter.
pub fn is_delimiter(c: char) -> bool {
    matches!(c, '-' | '_' )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_u8() {
        for i in 0..BASE {
            assert_eq!(i, char_to_u8(u8_to_char(i)).unwrap());
        }
    }

    #[test]
    #[cfg(feature="std")]
    fn invalid_u8() {
        for _ in 0..BASE {
            let int = rand::random_range(BASE..=u8::MAX);
            assert_ne!(int, char_to_u8(u8_to_char(int)).unwrap());
        }
    }

    #[test]
    #[cfg(feature="std")]
    fn valid_u16() {
        for _ in 0..BASE {
            let int = rand::random_range(0..CUBED_BASE);
            assert_eq!(int, u16::try_from(Triplet::from_int_lossy(int)).unwrap())
        }
    }
    #[test]
    #[cfg(feature="std")]
    fn invalid_u16() {
        for _ in 0..BASE {
            let int = rand::random_range(CUBED_BASE..=u16::MAX);
            assert_ne!(int, u16::try_from(Triplet::from_int_lossy(int)).unwrap())
        }
    }
}
