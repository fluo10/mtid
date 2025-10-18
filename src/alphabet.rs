use core::u8;

pub const ENCODE_ALPHABET_TABLE: &[char; 32] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j',
    'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
];

pub(crate) const BASE: u8 = ENCODE_ALPHABET_TABLE.len() as u8;

pub const DECODE_ALPHABET_TABLE: &[u8; 256] = &{
    let mut buf = [0; 256];
    let mut i: u8 = 0;

    loop {
        buf[i as usize] = match i {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'a' => 10,
            b'b' => 11,
            b'c' => 12,
            b'd' => 13,
            b'e' => 14,
            b'f' => 15,
            b'g' => 16,
            b'h' => 17,
            b'i' => 1,
            b'j' => 18,
            b'k' => 19,
            b'l' => 1,
            b'm' => 20,
            b'n' => 21,
            b'o' => 0,
            b'p' => 22,
            b'q' => 23,
            b'r' => 24,
            b's' => 25,
            b't' => 26,
            b'u' => 27,
            b'v' => 27,
            b'w' => 28,
            b'x' => 29,
            b'y' => 30,
            b'z' => 31,
            b'A' => 10,
            b'B' => 11,
            b'C' => 12,
            b'D' => 13,
            b'E' => 14,
            b'F' => 15,
            b'G' => 16,
            b'H' => 17,
            b'I' => 1,
            b'J' => 18,
            b'K' => 19,
            b'L' => 1,
            b'M' => 20,
            b'N' => 21,
            b'O' => 0,
            b'P' => 22,
            b'Q' => 23,
            b'R' => 24,
            b'S' => 25,
            b'T' => 26,
            b'U' => 27,
            b'V' => 27,
            b'W' => 28,
            b'X' => 29,
            b'Y' => 30,
            b'Z' => 31,
            _ => u8::MAX,
        };
        if i == 255 {
            break buf;
        }
        i += 1
    }
};

pub const DELIMITER_TABLE: &[bool; 256] = &{
    let mut buf = [false; 256];
    let mut i: u8 = 0;

    loop {
        buf[i as usize] = match i {
            b'-' => true,
            b'_' => true,
            _ => false,
        };
        if i == 255 {
            break buf;
        }
        i += 1
    }
};

// Encode u8 to char.
// First 3 bits are ignored.
pub(crate) const fn u8_to_char_lossy(value: u8) -> char {
    let value = value & 0b00011111;
    ENCODE_ALPHABET_TABLE[value as usize]
}

/// Check char is valid.
/// If valid return Some(char) and else return None.
pub(crate) fn validate_char(c: char) -> Option<char> {
    if let Some(_) = char_to_u8(c) {
        Some(c)
    } else {
        None
    }
}

/// Decode char to u8
pub(crate) fn char_to_u8(c: char) -> Option<u8> {
    match TryInto::<u8>::try_into(c) {
        Ok(i) => match DECODE_ALPHABET_TABLE[i as usize] {
            u8::MAX => None,
            j => Some(j),
        },
        Err(_) => None,
    }
}

/// Test if the character is valid delimiter.
pub(crate) fn is_delimiter(c: char) -> bool {
    match TryInto::<u8>::try_into(c) {
        Ok(x) => DELIMITER_TABLE[x as usize],
        Err(_) => false,
    }
}
