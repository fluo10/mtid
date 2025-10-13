//! A human-friendly identifier format based on 3-character blocks ("triplet").
//! This crate provide multiple fixed-length variants:
//! 
//! - `Stid`: Single triplet ID (e.g. `123`)
//! - `Dtid`: Doulbe Triplet ID (e.g. `456-789`)
//! - `Ttid`: Triple Triplet ID (e.g. `abc-def-ghj`)
//! - `Qtid`: Quadruple triplet ID (e.g. `kmn-pqr-stv-wxy`)
//! 
//! For a language agnostic specification of the MTID format, see [SPECS.md](https://github.com/fluo10/mtid/blob/main/SPECS.md)
//! 
//! ## Quick Start
//! 
//! ```
//! use mtid::Dtid;
//! use rand::{Rng, thread_rng};
//! 
//! let id = Dtid::random(&mut thread_rng());
//! println!("{}", id); // e.g. "1a2-b3c"
//! 
//! ```
//! 
//! ## Why MTID?
//! 
//! Traditional identifier systems face challenges in distributed environments:
//! 
//! - **Sequential numbers** (like GitHub issue numbers) cause collisions in distributed systems
//! - **UUIDs** are too long and not human-friendly
//! - **Short hashes** (like Git commit hashes) lack standardization
//! 
//! MTID bridges the gap between human readability and technical requirements.
//! 
//! ## Which length should I use?
//! 
//! - DTID(Double length triplet ID) is recommended for the personal data
//!     because this is short enouph to satisfy the Magic Number 7±2 principle and have enough range of value
//!     (for the data entered manually by individuals (such as pocketbooks, journals, or activity logs)).
//! - STID(Single length triplet ID) is recommended if the data is expected to be so few that they can be counted.
//! - TTID(Triple length triplet ID) is recommended if it is expected that one or more data will be added every second.
//! - QTID(Quadruple length Triplet ID) is recommended if, the number of data could potentially become so large that it's impossible to predict
//!     (for example, in a multi-user application where the IDs must be unique across users).
//! 
//! ## Installation
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies.mtid]
//! version = "0.1.0"
//! 
//! # Optional features
//! [dependencies.mtid]
//! version = "0.1.0-alpha"
//! features = ["serde", "rusqlite", "sea-orm", "prost"] }
//! ```
//! 
//! ## Features
//! 
//! - **Human-friendly**: Easy to read, type, and communicate
//! - **Collision-resistant**: Sufficient entropy for distributed systems
//! - **Compact**: Shorter than UUIDs while maintaining uniqueness
//! - **Type-safe**: Rust implementation with strong typing
//! - **Multiple integrations**: Support for serde, rusqlite, sea-orm, and protobuf
//! 
//! ### Optional Feature Flags
//! 
//! - `serde`: Serialization/deserialization support
//! - `rusqlite`: SQLite database integration
//! - `sea-orm`: SeaORM ORM integration  
//! - `prost`: Protocol Buffers support
//! 
//! ## Examples
//! 
//! ```rust
//! use mtid::{Stid, Dtid, Ttid, Qtid};
//! # fn main() -> Result<(), mtid::Error> {
//! // Generate random MTID
//! use rand::{Rng, thread_rng};
//! let mut rng = thread_rng();
//! let stid = Stid::random(&mut rng); 
//! let dtid = Dtid::random(&mut rng);
//! let ttid = Ttid::random(&mut rng);
//! let qtid = Qtid::random(&mut rng);
//! 
//! // '123', '456-789', 'abc-def-ghj', 'kmn-pqr-stv-wxy'
//! println!("'{}', '{}', '{}'. '{}'", stid, dtid, ttid, qtid); 
//! 
//! // Parse from string
//! let valid_id: Dtid = "012-tvw".parse()?;
//! 
//! // The code without delimiter is valid.
//! let valid_id_without_delimiter: Dtid = "012-tvw".parse()?;
//! assert_eq!(valid_id, valid_id_without_delimiter);
//! 
//! // When decoding from BASE32, ambiguous characters (1/l/I, 0/o, v/u, -/_) are treated as 1, 0, v, and - respectively, so they do not cause errors.
//! let also_valid_id: Dtid = "ol2_tuw".parse()?;
//! assert_eq!(valid_id, also_valid_id);
//! 
//! // Convert to/from integer
//! let num: u32 = valid_id.into();
//! let id_from_int: Dtid = num.try_into()?;
//! assert_eq!(valid_id, id_from_int);
//! 
//! // Lossy comversion from oversized int is allowed.
//! let id_from_overflowed_int = Dtid::from_int_lossy(Dtid::CAPACITY + num);
//! assert_eq!(valid_id, id_from_overflowed_int);
//! 
//! # Ok(())
//! # }
//! ```

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
