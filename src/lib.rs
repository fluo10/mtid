mod single;
mod double;
mod error;
mod triple;
mod utils;
#[cfg(feature="rusqlite")]
mod rusqlite;
#[cfg(feature="sea-orm")]
mod sea_orm;
#[cfg(feature="serde")]
mod serde;

use std::{fmt::Display, ops::Sub, str::FromStr};

pub use single::*;
pub use double::*;
pub use triple::*;
pub use error::*;

#[cfg(feature="prost")]
pub mod prost;
#[cfg(feature="prost")]
pub use prost::{ SingleMessage, DoubleMessage, TripleMessage ,TripodIdMessage};

/// The main trait for the tripod id
pub trait TripodId: Copy + Display + TryFrom<Self::Integer, Error=Error> + From<Self::Tuple> + FromStr<Err=Error> + PartialEq + PartialEq<String> {
    
    /// An associated integer type.
    /// This type is used to store the actual value of id.
    type Integer: From<Self> + Sub + PartialOrd;
    
    /// An associated tuple type containing SingleId.
    /// This type is used to represent the id as the tuple of SingleId.
    type Tuple: From<Self>;

    /// An associated protobuf message type.
    /// This type is used for conversion between the protobuf message.
    #[cfg(feature="prost")]
    type Message: From<Self> + TryInto<Self, Error=Error>;

    /// The nil Tripod ID.
    /// 
    /// # Example
    /// 
    /// Basic usage: 
    /// 
    /// ```
    /// # use tripod_id::{TripodId, Single};
    /// let id = Single::NIL;
    /// 
    /// assert_eq!(id, 0);
    /// assert_eq!(id, "000".to_string());
    /// ```
    const NIL: Self;


    /// The max Tripod ID.
    /// 
    /// # Example
    /// 
    /// Basic usage: 
    /// 
    /// ```
    /// # use tripod_id::{TripodId, Double};
    /// let id = Double::MAX;
    /// 
    /// assert_eq!(id, Double::CAPACITY - 1);
    /// assert_eq!(id, "ZZZ-ZZZ".to_string())
    /// ```
    const MAX: Self;

    /// The capacity of the Tripod ID.
    const CAPACITY: Self::Integer;

    /// Test if the Tripod ID is nil (=0).
    fn is_nil(self) -> bool {
        self == Self::NIL
    }

    /// Test if the id is max(=Self::CAPACITY-1)
    fn is_max(self) -> bool {
        self == Self::MAX
    }

}
