use tripod_id::{TripodId, Error};

pub trait TripodIdValidator: tripod_id::TripodId {
    fn validate_parse_strings(self, strings: &[&str]) -> Result<bool, Error> {
        let mut result: bool = true;
        for string in strings {
            result = result && (self == string.to_string())
        }
        Ok(result)
    }

    fn validate_inner_value(self) -> bool {
        let int = self.into();
        Self::CAPACITY > int
    }
    
    fn validate_string_convertion(self) -> Result<bool, Error> {
        Ok(self == Self::from_str(&self.to_string())?)
    }
    
    fn validate_integer_conversion(self) -> Result<bool, Error> {
        Ok(self == Self::try_from(Self::Integer::from(self))?)
    }

    fn validate_tuple_conversion(self) -> bool {
        self == Self::from(Self::Tuple::from(self))
    }
    #[cfg(feature="prost")]
    fn validate_message_conversion(self) -> Result<bool, Error> {
        Ok(self == Self::Message::from(self).try_into()?)        
    }
    
    fn validate_all(self) -> Result<bool, Error> {
        let mut result = self.validate_inner_value()
            && self.validate_string_convertion()? 
            && self.validate_integer_conversion()?;
        #[cfg(feature="prost")]
        {
            result = result && self.validate_message_conversion()?;
        }
        
        Ok(result)
    }
}