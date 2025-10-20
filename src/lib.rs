//! A human-friendly identifier format based on 3-character blocks ("triplet").
//! This crate provides multiple fixed-length variants:
//!
//! - `Stid`: Single triplet ID (e.g. `123`)
//! - `Dtid`: Double Triplet ID (e.g. `456-789`)
//! - `Ttid`: Triple Triplet ID (e.g. `abc-def-ghj`)
//! - `Qtid`: Quadruple triplet ID (e.g. `kmn-pqr-stv-wxy`)
//!
//! For a language agnostic specification of the MTID format, see [SPECS.md](https://github.com/fluo10/mtid/blob/main/SPECS.md)
//!
//! # Quick Start
//!
//! ```
//! use mtid::Dtid;
//!
//! let id = Dtid::random();
//! println!("{}", id); // e.g. "1a2-b3c"
//! ```
//!
//! # Why MTID?
//!
//! Traditional identifier systems face challenges in distributed environments:
//!
//! - **Sequential numbers** (like GitHub issue numbers) cause collisions in distributed systems
//! - **UUIDs** are too long and not human-friendly
//! - **Short hashes** (like Git commit hashes) lack standardization
//!
//! MTID bridges the gap between human readability and technical requirements.
//!
//! # Which length should I use?
//!
//! - DTID(Double length triplet ID) is recommended for the personal data
//!   because this is short enough to satisfy the Magic Number 7±2 principle and have enough range of value
//!   (for the data entered manually by individuals (such as pocketbooks, journals, or activity logs)).
//! - STID(Single length triplet ID) is recommended if the data is expected to be so few that they can be counted.
//! - TTID(Triple length triplet ID) is recommended if it is expected that one or more data will be added every second.
//! - QTID(Quadruple length Triplet ID) is recommended if, the number of data could potentially become so large that it's impossible to predict
//!   (for example, in a multi-user application where the IDs must be unique across users).
//!
//! # Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mtid = "0.5"
//!
//! # With optional features
//! mtid = { version = "0.5", features = ["arbitrary", "serde", "rusqlite", "sea-orm", "prost"] }
//! ```
//!
//! ## For no_std Environments
//!
//! This crate support `no_std`.
//! For `no_std` environment, you'll need to disable default features.
//!
//! ```toml
//! [dependencies]
//! mtid = { version = "0.5", default-features = false }
//! ```
//!
//! # Features
//!
//! - **Human-friendly**: Easy to read, type, and communicate
//! - **Collision-resistant**: Sufficient entropy for distributed systems
//! - **Compact**: Shorter than UUIDs while maintaining uniqueness
//! - **Type-safe**: Rust implementation with strong typing
//! - **Multiple integrations**: Support for serde, rusqlite, sea-orm, and protobuf
//!
//! ## Optional Feature Flags
//!
//! - `arbitrary`: `arbitrary::Arbitrary` support for fuzzing tests.
//! - `serde`: Serialization/deserialization support
//! - `rusqlite`: SQLite database integration
//! - `sea-orm`: SeaORM ORM integration  
//! - `prost`: Protocol Buffers support
//!
//! # Examples
//!
//! ```rust
//! use mtid::{Stid, Dtid, Ttid, Qtid};
//! # fn main() -> Result<(), mtid::Error> {
//! // Generate random MTID
//! let stid = Stid::random();
//! let dtid = Dtid::random();
//! let ttid = Ttid::random();
//! let qtid = Qtid::random();
//!
//! // '123', '456-789', 'abc-def-ghj', 'kmn-pqr-stv-wxy'
//! println!("'{}', '{}', '{}'. '{}'", stid, dtid, ttid, qtid);
//!
//! // Parse from string
//! let valid_id: Dtid = "012-tvw".parse()?;
//!
//! // The code without delimiter is valid.
//! let valid_id_without_delimiter: Dtid = "012tvw".parse()?;
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
//! // Lossy conversion from oversized int is allowed.
//! let id_from_overflowed_int = Dtid::from_int_lossy(Dtid::CAPACITY + num);
//! assert_eq!(valid_id, id_from_overflowed_int);
//!
//! # Ok(())
//! # }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

/// Provides constants and private functions about encoding/decoding alphabet.
///
/// This module implements encoding and decoding character based on [Crockford's Base32](https://www.crockford.com/base32.html) with following exceptions:
///
/// - The letter `u` (`U`) is decoded to 27, same as `v`.
/// - Characters are separated by hyphens every three characters (triplet) during encoding.
///   During decoding, hyphens may be omitted or replaced with underscores.
pub mod alphabet;

mod dtid;
mod error;
mod macros;
mod qtid;
mod stid;
mod ttid;

/// Provides [`Triplet`](triplet::Triplet) and [`TripletError`](triplet::TripletError).
pub mod triplet;

pub use dtid::Dtid;
pub use error::Error;
pub use qtid::Qtid;
pub use stid::Stid;
pub use ttid::Ttid;

/// Provides message types generated by prost-build.
#[cfg(feature = "prost")]
pub mod proto;

#[cfg(feature = "prost")]
#[deprecated(since = "6.0.0", note = "please use `StidProto` instead")]
pub type StidMessage = proto::Stid;

#[cfg(feature = "prost")]
#[deprecated(since = "6.0.0", note = "please use `DtidProto` instead")]
pub type DtidMessage = proto::Dtid;

#[cfg(feature = "prost")]
#[deprecated(since = "6.0.0", note = "please use `TtidProto` instead")]
pub type TtidMessage = proto::Ttid;

#[cfg(feature = "prost")]
#[deprecated(since = "6.0.0", note = "please use `QtidProto` instead")]
pub type QtidMessage = proto::Qtid;

/// Alias of [`proto::Stid`]
#[cfg(feature = "prost")]
pub type StidProto = proto::Stid;

/// Alias of [`proto::Dtid`]
#[cfg(feature = "prost")]
pub type DtidProto = proto::Dtid;

/// Alias of [`proto::Ttid`]
#[cfg(feature = "prost")]
pub type TtidProto = proto::Ttid;

/// Alias of [`proto::Qtid`]
#[cfg(feature = "prost")]
pub type QtidProto = proto::Qtid;
