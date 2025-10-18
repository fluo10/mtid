/// A general error that can occur when working with Triplet.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TripletError {
    ParseLength(usize),
    ParseCharacter { character: char, index: usize },
}

impl core::fmt::Display for TripletError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TripletError::ParseLength(length) => {
                write!(f, "Invalid length: expected 3, found {}", length)
            }
            TripletError::ParseCharacter { character, index } => {
                write!(
                    f,
                    "Invalid character: expected alphanumeric character, found {} at {}",
                    character, index
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TripletError {}
