use std::str::FromStr;

use serde::{Deserialize, Serialize, de::Error};

use crate::{Dtid, Stid, Ttid, Qtid};

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

impl Serialize for Qtid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Qtid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Qtid::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}