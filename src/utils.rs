use crate::Error;

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

// Decode char to u8
pub fn char_to_u8(c: char) -> Result<u8, Error> {
    Ok(match c {
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
        _ => return Err(Error::InvalidChar(c)) 
    })
}

// Decode 3 chars to u16.
pub fn chars_to_u16(chars: (char, char, char)) -> Result<u16, Error> {
    Ok(
        ((char_to_u8(chars.0)? as u16) << 10) 
            | ((char_to_u8(chars.1)? as u16) << 5)
            | (char_to_u8(chars.2)? as u16)
    )
}

// Encode u16 to 3 chars.
// First 1 bit of arg is ignored.
pub fn u16_to_chars(int: u16) -> (char, char, char) {
    (
        u8_to_char((int >> 10) as u8),
        u8_to_char((int >> 5) as u8),
        u8_to_char(int as u8)
    )
}

/// Test if the character is valid delimiter.
pub fn is_delimiter(c: char) -> bool {
    match c {
        '-' | '_' => true,
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn valid_u8() {
        for i in 0..BASE {
            assert_eq!(i, char_to_u8(u8_to_char(i)).unwrap());
        }
    }

    #[test]
    fn invalid_u8() {
        let mut rng = thread_rng();

        for _ in 0..BASE {
            let int = rng.gen_range(BASE..=u8::MAX);
            assert_ne!(int, char_to_u8(u8_to_char(int)).unwrap());
        }
    }
    #[test]
    fn valid_u16() {
        let mut rng = thread_rng();
        for _ in 0..BASE {
            let int = rng.gen_range(0..CUBED_BASE);
            assert_eq!(int, chars_to_u16(u16_to_chars(int)).unwrap())
        }
    }
    #[test]
    fn invalid_u16() {
        let mut rng = thread_rng();
        for _ in 0..BASE {
            let int = rng.gen_range(CUBED_BASE..=u16::MAX);
            assert_ne!(int, chars_to_u16(u16_to_chars(int)).unwrap())
        }
    }
}
