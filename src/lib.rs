//! # TripodID
//! 
//! A distributable human-friendry identifier.
//! 
//! ## Example
//! ```
//! use std::str::FromStr;
//! use tripod_id::{Stid, Dtid, Ttid, Error};
//! # fn main() -> Result<(), Error> {
//! 
//! // Single length Tripod ID
//! let single: Stid = Stid::from_str("012")?;
//! 
//! // Double length Tripod ID
//! let double: Dtid = Dtid::from_str("345-678")?;
//! 
//! // Triple length Tripod ID
//! let triple: Ttid = Ttid::from_str("9ab-dce-fgh")?;
//! # Ok(())
//! # }
//! ```
//! 
//! 
//! 
//! 
//! 

mod stid;
mod dtid;
mod error;
mod ttid;
mod utils;
mod macros;

#[cfg(feature="rusqlite")]
mod rusqlite;
#[cfg(feature="sea-orm")]
mod sea_orm;
#[cfg(feature="serde")]
mod serde;

pub use stid::Stid;
pub use dtid::Dtid;
pub use ttid::Ttid;
pub use error::Error;

#[cfg(feature="prost")]
pub mod prost;
#[cfg(feature="prost")]
pub use prost::{ StidMessage, DtidMessage, TtidMessage};



