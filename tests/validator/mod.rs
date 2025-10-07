use std::{fmt::Display, str::FromStr};

use tripod_id::Error;


pub trait TripodIdValidator: From<Self::Tuple> + FromStr<Err = Error> + Display + TryFrom<Self::Integer, Error = Error> + Copy+ PartialEq {
    type Integer: From<Self>;
    type Tuple: From<Self>;
    
    fn validate_string_convertion(self) -> Result<bool, Error> {
        Ok(self == Self::from_str(&self.to_string())?)
    }
    
    fn validate_integer_conversion(self) -> Result<bool, Error> {
        Ok(self == Self::try_from(Self::Integer::from(self))?)
    }

    fn validate_tuple_conversion(self) -> bool {
        self == Self::from(Self::Tuple::from(self))
    }

    
    fn validate_all(self) -> Result<bool, Error> {
        Ok(self.validate_string_convertion()? 
            && self.validate_integer_conversion()?
            && self.validate_tuple_conversion()
        )
    }
}