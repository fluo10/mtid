use std::str::FromStr;

use serde::{Deserialize, Serialize, de::Error};

use crate::{Dtid, Stid, Ttid};

impl Serialize for Stid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Stid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Stid::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

impl Serialize for Dtid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Dtid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Dtid::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

impl Serialize for Ttid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Ttid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Ttid::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};
    
    #[test]
    fn Stid() {
        assert_tokens(&crate::Stid::NIL, &[Token::Str("000")]);
    }

    #[test]
    fn double() {
        assert_tokens(&crate::Dtid::NIL, &[Token::Str("000-000")]);
    }
    #[test]
    fn triple() {
        assert_tokens(&crate::Ttid::NIL, &[Token::Str("000-000-000")]);
    }
}