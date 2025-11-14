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

use core::str::FromStr;
use std::fmt::Display;

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

/// Alias of [`proto::CarettaId`]
#[cfg(feature = "prost")]
pub type CarettaIdProto = proto::CarettaId;

/// Caretta id struct
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CarettaId(u64);

impl CarettaId {
    /// The size of the integer type in bits.
    /// 
    /// This is not equal actually stored size.
    pub const BITS: u32 = 40;

    /// The size of the integer type in bytes.
    /// 
    /// This is not equal actually stored size.
    pub const BYTES: u32 = 5;

    /// The capacity value of the caretta id
    /// 
    pub const CAPACITY: u64 = (u8::MAX as u64).pow(Self::BYTES);

    pub(crate) const CAPACITY_MINUS_ONE: u64 = Self::CAPACITY -1;

    pub(crate) const fn from_u64_unchecked(value: u64) -> Self {
        Self(value)
    }
    
    /// The smallest value that can be represented by [`CarettaId`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(CarettaId::NIL, "0000000".parse<CarettaId>()?);
    /// assert_eq!(CarettaId::NIL, CarettaId::try_from(0)?);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub const NIL: Self = Self::from_u64_unchecked(0);

    /// The largest value that can be represented by [`CarettaId`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert_eq!(CarettaId::MAX, "zzzzzzz".parse<CarettaId>()?);
    /// assert_eq!(CarettaId::MAX, CarettaId::try_from(0)?);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub const MAX: Self = Self::from_u64_unchecked(Self::CAPACITY_MINUS_ONE);

    /// Test if the [`CarettaId`] is nil.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert!("0000000".parse::<CarettaId>()?.is_nil())
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_nil(self) -> bool {
        self == Self::NIL
    }

    /// Test if the [`CarettaId`] is max.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// # fn main() -> Result<(), Error> {
    /// assert!("zzzzzzz".parse::<CarettaId>()?.is_max())
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_max(self) -> bool {
        self == Self::MAX
    }

    /// "Converts an unsigned integer to [`CarettaId`] by truncating bits that exceed the valid range.")]
    ///
    /// This is a lossy conversion that masks the input value to fit within the ID's bit limit.
    /// If you need to detect out-of-range values, use [`TryFrom`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// // Values within range are preserved
    /// let valid_int = 123;
    /// let id = CarettaId::from_u64_lossy(valid_int);
    /// assert_eq!(u64::from(id), valid_int);
    ///
    /// // values exceeding 35 bits are truncated (MSB(s) dropped
    /// let oversized_int = 0;
    /// let overflowed_id = CarettaId::from_uint_lossy(oversized_int);
    /// assert_nq!(u64::from(overflowed_id), oversized_int);
    /// // Only lower 35 bits retained
    /// assert_eq!(u64::from(overflowed_id), valid_int)
    /// ```
    pub const fn from_u64_lossy(int: u64) -> Self {
        Self::from_u64_unchecked(int & Self::CAPACITY_MINUS_ONE)
    }

    pub const fn from_u64(value: u64) -> Result<Self, crate::Error> {
        if value < Self::CAPACITY {
            Ok(Self::from_u64_unchecked(value))
        } else {
            Err(Error::ParseInteger {
                expected: Self::CAPACITY as u64,
                found: value,
            })
        }
    }
    pub const fn to_u64(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for CarettaId {
    type Error = Error;
    #[doc = concat!("Attempts to convert a [`", stringify!($Uint), "`]  to [`", stringify!($SelfT), "`].")]
    ///
    /// Return error if the value is equal [`CAPACITY`](Self::CAPACITY) or more.
    /// If you don't need to detect out-of-range values, use [`from_uint_lossy`](Self::from_uint_lossy).
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// assert!(CarettaId::try_from(Self::CAPACITY - 1).is_ok());
    /// assert!(CarettaId::try_from(Self::CAPACITY).is_err());
    /// ```
    ///
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::from_u64(value)
    }
}

impl From<CarettaId> for u64 {
    fn from(value: CarettaId) -> Self {
        value.0
    }
}

impl Display for CarettaId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
impl FromStr for CarettaId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len == 7 {
            let mut chars = s.chars();
            todo!()
        } else {
            Err(Error::ParseLength { expected_without_delimiter: 7, expected_with_delimiter: None, found: len })
        }
    }
}

#[cfg(feature = "arbitrary")]
mod arbitrary {
    use ::arbitrary::{Arbitrary, Unstructured, Result};
    use super::*;
    impl<'a> Arbitrary<'a> for CarettaId {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(Self::from_u64_lossy(u64::arbitrary(u)?))
        }
    }
}

#[cfg(feature = "rand")]
mod rand {
    use super::*;
    use ::rand::{distr::{Distribution, StandardUniform}, Rng};

    impl Distribution<CarettaId> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CarettaId {
            <CarettaId>::from_u64_lossy(rng.random())
        }
    }
    impl CarettaId {
        /// Generate a new random [`CarettaId`].
        ///
        /// This method generate a random ID.
        /// The generated ID is guaranteed to not be the [`NIL`](Self::NIL) value.
        ///
        /// # Examples
        ///
        /// ```
        /// # use caretta_id::*;
        /// let id = CarettaId::random();
        /// assert_ne!(id, CarettaId::NIL);
        /// ```
        pub fn random() -> Self {
            <CarettaId>::from_u64_lossy(::rand::random())
        }
    }
}
#[cfg(feature = "serde")]
mod serde {
    use super::*;
    use ::serde::{Deserialize, Serialize, de::Error};

    impl Serialize for CarettaId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for CarettaId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            (&s).parse::<CarettaId>().map_err(|e| D::Error::custom(e))
        }
    }
}


