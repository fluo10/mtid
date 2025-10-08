//! # TripletID
//! 
//! A distributable human-friendry identifier.
//! 
//! ## Example
//! ```
//! use std::str::FromStr;
//! use mtid::{Stid, Dtid, Ttid, Error};
//! # fn main() -> Result<(), Error> {
//! 
//! // Single length Triplet ID
//! let single: Stid = Stid::from_str("012")?;
//! 
//! // Double length Triplet ID
//! let double: Dtid = Dtid::from_str("345-678")?;
//! 
//! // Triple length Triplet ID
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

/// Alias of single triplet-id message
#[cfg(feature="prost")]
pub type StidMessage = prost::Stid;

/// Alias of double triplet-id message
#[cfg(feature="prost")]
pub type DtidMessage = prost::Dtid;

/// Alias of triple triplet-id message
#[cfg(feature="prost")]
pub type TtidMessage = prost::Ttid;


