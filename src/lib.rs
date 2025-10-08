//! # TripletID
//! 
//! A distributable human-friendry identifier.
//! 
//! ## Example
//! ```
//! use mtid::{Stid, Dtid, Ttid, Qtid, Error};
//! # fn main() -> Result<(), Error> {
//! 
//! // Single length Triplet ID
//! let single: Stid = "012".parse()?;
//! 
//! // Double length Triplet ID
//! let double: Dtid = "345-678".parse()?;
//! 
//! // Triple length Triplet ID
//! let triple: Ttid = "9ab-dce-fgh".parse()?;
//! 
//! // Quadruple length Triplet ID
//! let quadruple: Qtid = "jkm-npq-rst-uwx".parse()?;
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
mod ttid;
mod qtid;
mod error;
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
pub use qtid::Qtid;
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

/// Alias of triple triplet-id message
#[cfg(feature="prost")]
pub type QtidMessage = prost::Qtid;
