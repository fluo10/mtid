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