/// A general error that can occur when working with Triplet.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub enum TripletError {
    #[error("Invalid length: expected 3, found {0}")]
    ParseLength(usize),
    #[error("Invalid character: expected alphanumeric character, found {character} at {index}")]
    ParseCharacter { character: char, index: usize },
}
