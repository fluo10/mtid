//! # TripodID
//! 
//! A distributable human-friendry identifier.
//! 
//! ## Example
//! ```
//! use std::str::FromStr;
//! use tripod_id::{Single, Double, Triple, Error};
//! # fn main() -> Result<(), Error> {
//! let single: Single = Single::from_str("012")?;
//! let double: Double = Double::from_str("345-678")?;
//! let triple: Triple = Triple::from_str("9ab-dce-fgh")?;
//! # Ok(())
//! # }
//! ```
//! 
//! 
//! 
//! 
//! 

mod single;
mod double;
mod error;
mod triple;
mod common;
mod macros;

#[cfg(feature="rusqlite")]
mod rusqlite;
#[cfg(feature="sea-orm")]
mod sea_orm;
#[cfg(feature="serde")]
mod serde;

pub use single::Single;
pub use double::Double;
pub use triple::Triple;
pub use error::Error;

#[cfg(feature="prost")]
pub mod prost;
#[cfg(feature="prost")]
pub use prost::{ SingleMessage, DoubleMessage, TripleMessage ,TripodIdMessage};



