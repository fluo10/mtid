macro_rules! mtid_impl {
    (
        Self = $SelfT:ident,
        ActualT = $ActualT:ident,
        BITS = $BITS:literal,
        CAPACITY = $CAPACITY:expr,
        NIL_STR = $NIL_STR:literal,
        MAX_STR = $MAX_STR:literal,
        MAX_INT = $MAX_INT:literal,
        description = $description:literal,
        example_str = $example_str:literal,
        example_int = $example_int:literal
    ) => {
        #[cfg(feature = "rand")]
        use rand::{distr::{Distribution, StandardUniform}, Rng};


        #[doc = concat!($description)]
        /// 
        /// # Examples
        /// 
        /// ```
        /// # use mtid::*;
        /// # fn main() -> Result<(), Error> {
        /// // Generate random value.
        #[doc = concat!("let random = ", stringify!($SelfT), "::random();")]
        /// 
        #[doc = concat!("assert_ne!(random, ", stringify!($SelfT), "::NIL);")]
        /// 
        /// // Parse from string.
        #[doc = concat!("let from_str: ", stringify!($SelfT), " = ", stringify!($example_str), ".parse()?;")]
        /// 
        /// // Parse from integer.
        #[doc = concat!("let from_int: ", stringify!($SelfT), " = ", $example_int, ".try_into()?;")]
        /// 
        /// assert_eq!(from_str, from_int);
        /// # Ok(())
        /// # }
        /// ```
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $SelfT($ActualT);

        impl $SelfT {
            /// The size of the integer type in bits.
            /// This is not equal actually stored size.
            pub const BITS: u32 = $BITS;
            
            /// The capacity value of this triplet id type
            /// 
            /// #Examples
            /// 
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($SelfT), "::CAPACITY -1);")]
            /// # Ok(())
            /// # }
            /// ```
            pub const CAPACITY: $ActualT = $CAPACITY;

            const CAPACITY_MINUS_ONE: $ActualT = Self::CAPACITY - 1;

            /// The smallest value that can be represented by this triplet id type.
            /// 
            /// # Examples
            /// 
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::NIL, ", stringify!($NIL_STR), ".parse::<", stringify!($SelfT), ">()?);")]
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::NIL, ", stringify!($SelfT), "::try_from(0)?);")]
            /// # Ok(())
            /// # }
            /// ```
            /// 
            pub const NIL: Self = Self(0);

            /// The largest value that can be represent by this triplet id type.
            /// 
            /// # Examples
            /// 
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($MAX_STR), ".parse::<",stringify!($SelfT),">()?);")]
            #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($SelfT), "::try_from(", stringify!($MAX_INT), ")?);")]
            /// # Ok(())
            /// # }
            /// ```
            pub const MAX: Self = Self(Self::CAPACITY_MINUS_ONE);

            #[doc = concat!("Generate a new random ", stringify!($SelfT), ".")]
            /// 
            /// This method generate a cryptgraphicaly random ID.
            /// The generated ID is guaranteed to not be the [`NIL`](Self::NIL) value.
            /// 
            /// # Examples
            /// 
            /// ```
            /// # use mtid::*;
            #[doc = concat!("let id = ", stringify!($SelfT), "::random();")]
            #[doc = concat!("assert_ne!(id, ", stringify!($SelfT), "::NIL);")]
            /// ```
            #[cfg(feature = "rand")]
            pub fn random() -> Self {
                Self(rand::random_range(1..=Self::CAPACITY_MINUS_ONE))
            }
            
            /// Test if the triplet id is nil.
            /// 
            /// # Examples
            /// 
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert!(", stringify!($NIL_STR), ".parse::<",stringify!($SelfT),">()?.is_nil());")]
            /// # Ok(())
            /// # }
            /// ```
            pub fn is_nil(self) -> bool {
                self.0 == 0
            }

            /// Test if the triplet id is max.
            /// 
            /// # Examples
            /// 
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!("assert!(", stringify!($MAX_STR), ".parse::<",stringify!($SelfT),">()?.is_max());")]
            /// # Ok(())
            /// # }
            /// ```
            pub fn is_max(self) -> bool {
                self.0 == Self::CAPACITY_MINUS_ONE
            }

            /// Convert from uint.
            /// If the value is equal Self::Capacity or more, the higher bits will be lost.
            /// 
            /// # Examples
            /// 
            /// ```
            /// # use mtid::*;
            /// # fn main() -> Result<(), Error> {
            #[doc = concat!(
                "assert!(",
                stringify!($SelfT),
                "::from_int_lossy(",
                stringify!($SelfT),
                "::CAPACITY - 1).is_max());"
            )]
            #[doc = concat!(
                "assert!(",
                stringify!($SelfT),
                "::from_int_lossy(",
                stringify!($SelfT),
                "::CAPACITY).is_nil());")]
            /// # Ok(())
            /// # }
            /// ```
            pub fn from_int_lossy(int: $ActualT) -> Self {
                Self(int & Self::CAPACITY_MINUS_ONE)
            }
        }

        #[cfg(feature = "rand")]
        impl Distribution<$SelfT> for StandardUniform {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $SelfT {
                $SelfT(rng.random_range(1..=$SelfT::CAPACITY_MINUS_ONE))
            }
        }
    };
}
pub(crate) use mtid_impl;