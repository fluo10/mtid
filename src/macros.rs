macro_rules! tripod_id_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        BITS = $BITS:literal,
        CAPACITY = $CAPACITY:expr,
        NIL_STR = $NIL_STR:literal,
        MAX_STR = $MAX_STR:literal,
    ) => {

        /// The size of the integer type in bits.
        /// This is not equal actually stored size.
        pub const BITS: u32 = $BITS;
        
        /// The capacity value of this tripod id type
        /// 
        /// #Examples
        /// 
        /// ```
        /// # use tripod_id::*;
        /// # fn main() -> Result<(), Error> {
        #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($SelfT), "::CAPACITY -1);")]
        /// # Ok(())
        /// # }
        /// ```
        pub const CAPACITY: $ActualT = $CAPACITY;

        const CAPACITY_MINUS_ONE: $ActualT = Self::CAPACITY - 1;

        /// The smallest value that can be represented by this tripod id type.
        /// 
        /// # Examples
        /// 
        /// ```
        /// # use tripod_id::*;
        /// # fn main() -> Result<(), Error> {
        #[doc = concat!("assert_eq!(", stringify!($SelfT), "::NIL, ", stringify!($NIL_STR), ".parse::<", stringify!($SelfT), ">()?);")]
        /// # Ok(())
        /// # }
        /// ```
        /// 
        pub const NIL: Self = Self(0);

        /// The largest value that can be represent by this tripod id type.
        /// 
        /// # Examples
        /// 
        /// ```
        /// # use tripod_id::*;
        /// # fn main() -> Result<(), Error> {
        #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MAX, ", stringify!($MAX_STR), ".parse::<",stringify!($SelfT),">()?);")]
        /// # Ok(())
        /// # }
        /// ```
        pub const MAX: Self = Self(Self::CAPACITY_MINUS_ONE);
        
        /// Test if the tripod id is nil.
        /// 
        /// # Examples
        /// 
        /// ```
        /// # use tripod_id::*;
        /// # fn main() -> Result<(), Error> {
        #[doc = concat!("assert!(", stringify!($NIL_STR), ".parse::<",stringify!($SelfT),">()?.is_nil());")]
        /// # Ok(())
        /// # }
        /// ```
        pub fn is_nil(self) -> bool {
            self.0 == 0
        }

        /// Test if the tripod id is max.
        /// 
        /// # Examples
        /// 
        /// ```
        /// # use tripod_id::*;
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
        /// # use tripod_id::*;
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
    };
}

pub(crate) use tripod_id_impl;