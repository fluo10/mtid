use crate::triplet::TripletError;

/// A general error that can occur when working with caretta-ids.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Invalid value, expected under , found {0}")]
    ValueOutOfRange(u64),
    #[error("Invalid length, expected 7, found {0}")]
    InvalidLength(usize),
    #[error("Invalid character, alphanumeric expected, found {character} at {index}")]
    InvalidCharacter{
        character: char,
        index: usize,
    },
    #[deprecated(since = "0.8.0", note = "The ParseInteger has been renamed. Use ValueOfRange instead.")]
    #[error("Invalid integer value: epected under {expected}, found {found}.")]
    ParseInteger {
        expected: u64,
        found: u64,
    },
    #[deprecated(since = "0.8.0", note = "The ParseLength has been renamed. Use InvalidLength instead.")]
    #[error("Invalid length: expected {expected_without_delimiter} {}, found {found}", expected_with_delimiter.map(|x| format!("or {}", x)).unwrap_or("".to_string()))]
    ParseLength {
        expected_without_delimiter: usize,
        expected_with_delimiter: Option<usize>,
        found: usize,
    },
    #[deprecated(since = "0.8.0")]
    #[error("Invalid delimiter: expected: '-' or '_', found {character} at {index}")]
    ParseDelimiter {
        character: char,
        index: usize,
    },
    #[deprecated(since = "0.8.0")]    
    #[error("Invalid triplet: source: {source}, index: {index}")]
    ParseTriplet {
        source: TripletError,
        index: usize,
    },
    #[deprecated(since = "0.8.0")]    
    #[error("Invalid character: expected alphanumeric character, found {character} at {index}")]
    ParseCharacter { character: char, index: usize },
}
