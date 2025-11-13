//! A human-friendly identifier format based on 3-character blocks.
//! This crate provides multiple fixed-length variants:
//!
//! - `CarettaIdS`: Single length caretta ID (e.g. `123`)
//! - `CarettaIdD`: Double length caretta ID (e.g. `456-789`)
//! - `CarettaIdT`: Triple length caretta ID (e.g. `abc-def-ghj`)
//! - `CarettaIdQ`: Quadruple length caretta ID (e.g. `kmn-pqr-stv-wxy`)
//!
//! For a language agnostic specification of the caretta-id format, see [SPECS.md](https://github.com/fluo10/caretta-id/blob/main/SPECS.md)
//!
//! # Quick Start
//!
//! ```
//! use caretta_id::CarettaIdD;
//!
//! let id = CarettaIdD::random();
//! println!("{}", id); // e.g. "1a2-b3c"
//! ```
//!
//! # Why caretta-id?
//!
//! Traditional identifier systems face challenges in distributed environments:
//!
//! - **Sequential numbers** (like GitHub issue numbers) cause collisions in distributed systems
//! - **UUIDs** are too long and not human-friendly
//! - **Short hashes** (like Git commit hashes) lack standardization
//!
//! caretta-id bridges the gap between human readability and technical requirements.
//!
//! # Which length should I use?
//!
//! - CarettaIdD(Double length) is recommended for the personal data
//!   because this is short enough to satisfy the Magic Number 7±2 principle and have enough range of value
//!   (for the data entered manually by individuals (such as pocketbooks, journals, or activity logs)).
//! - CarettaIdS(Single length) is recommended if the data is expected to be so few that they can be counted.
//! - CarettaIdT(Triple length) is recommended if it is expected that one or more data will be added every second.
//! - CarettaIdQ(Quadruple length) is recommended if, the number of data could potentially become so large that it's impossible to predict
//!   (for example, in a multi-user application where the IDs must be unique across users).
//!
//! # Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! caretta-id = "0.7"
//!
//! # With optional features
//! caretta-id = { version = "0.7", features = ["arbitrary", "serde", "rusqlite", "sea-orm", "prost", "redb"] }
//! ```
//!
//! ## For no_std Environments
//!
//! This crate support `no_std`.
//! For `no_std` environment, you'll need to disable default features.
//!
//! ```toml
//! [dependencies]
//! caretta-id = { version = "0.7", default-features = false }
//! ```
//!
//! # Features
//!
//! - **Human-friendly**: Easy to read, type, and communicate
//! - **Collision-resistant**: Sufficient entropy for personal distributed systems
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
//! - `redb`: `redb` integration
//!
//! # Examples
//!
//! ```rust
//! use caretta_id::{CarettaIdS, CarettaIdD, CarettaIdT, CarettaIdQ};
//! # fn main() -> Result<(), caretta_id::Error> {
//! // Generate random caretta-id
//! let caretta_id_s = CarettaIdS::random();
//! let caretta_id_d = CarettaIdD::random();
//! let caretta_id_t = CarettaIdT::random();
//! let caretta_id_q = CarettaIdQ::random();
//!
//! // '123', '456-789', 'abc-def-ghj', 'kmn-pqr-stv-wxy'
//! println!("'{}', '{}', '{}'. '{}'", caretta_id_s, caretta_id_d, caretta_id_t, caretta_id_q);
//!
//! // Parse from string
//! let valid_id: CarettaIdD = "012-tvw".parse()?;
//!
//! // The code without delimiter is valid.
//! let valid_id_without_delimiter: CarettaIdD = "012tvw".parse()?;
//! assert_eq!(valid_id, valid_id_without_delimiter);
//!
//! // When decoding from BASE32, ambiguous characters (1/l/I, 0/o, v/u, -/_) are treated as 1, 0, v, and - respectively, so they do not cause errors.
//! let also_valid_id: CarettaIdD = "ol2_tuw".parse()?;
//! assert_eq!(valid_id, also_valid_id);
//!
//! // Convert to/from integer
//! let num: u32 = valid_id.into();
//! let id_from_int: CarettaIdD = num.try_into()?;
//! assert_eq!(valid_id, id_from_int);
//!
//! // Lossy conversion from oversized int is allowed.
//! let id_from_overflowed_int = CarettaIdD::from_uint_lossy(CarettaIdD::CAPACITY + num);
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

mod double;
mod error;
mod macros;
mod quadruple;
mod single;
mod triple;

/// Provides [`Triplet`](triplet::Triplet) and [`TripletError`](triplet::TripletError).
pub mod triplet;

pub use double::CarettaIdD;
pub use error::Error;
pub use quadruple::CarettaIdQ;
pub use single::CarettaIdS;
pub use triple::CarettaIdT;

/// Provides message types generated by prost-build.
#[cfg(feature = "prost")]
pub mod proto;

/// Alias of [`proto::CarettaIdS`]
#[cfg(feature = "prost")]
pub type CarettaIdSProto = proto::CarettaIdS;

/// Alias of [`proto::CarettaIdD`]
#[cfg(feature = "prost")]
pub type CarettaIdDProto = proto::CarettaIdD;

/// Alias of [`proto::CarettaIdT`]
#[cfg(feature = "prost")]
pub type CarettaIdTProto = proto::CarettaIdT;

/// Alias of [`proto::CarettaIdQ`]
#[cfg(feature = "prost")]
pub type CarettaIdQProto = proto::CarettaIdQ;
