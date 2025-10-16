use crate::triplet::TripletError;

/// A general error that can occur when working with MTIDs.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    ParseInteger {
        expected: u64,
        found: u64,
    },
    ParseLength {
        expected_without_delimiter: usize,
        expected_with_delimiter: Option<usize>,
        found: usize,
    },
    ParseDelimiter {
        character: char,
        index: usize,
    },
    ParseTriplet {
        source: TripletError,
        index: usize,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::ParseInteger { expected, found } => {
                write!(
                    f,
                    "Invalid integer value: epected under {}, found {}.",
                    expected, found
                )
            }
            Error::ParseLength {
                expected_without_delimiter,
                expected_with_delimiter,
                found,
            } => {
                if let Some(x) = expected_with_delimiter {
                    write!(
                        f,
                        "Invalid length: expected {} or {}, found {}",
                        expected_without_delimiter, x, found
                    )
                } else {
                    write!(
                        f,
                        "Invalid length: expected {} but found {}",
                        expected_without_delimiter, found
                    )
                }
            }
            Error::ParseDelimiter { character, index } => {
                write!(
                    f,
                    "Invalid delimiter: expected: '-' or '_', found {} at {}",
                    character, index
                )
            }
            Error::ParseTriplet { source, index } => {
                write!(f, "Invalid triplet: source: {}, index: {}", source, index)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::ParseTriplet { source, index: _ } => Some(source),
            _ => None,
        }
    }
}
