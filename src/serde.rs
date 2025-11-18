use super::*;
use ::serde::{Deserialize, Serialize, de::Error};

impl Serialize for CarettaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&format!("{self}"))
        } else {
            serializer.serialize_u64(self.to_u64())
        }
    }
}

impl<'de> Deserialize<'de> for CarettaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            (&String::deserialize(deserializer)?)
                .parse::<CarettaId>()
                .map_err(D::Error::custom)
        } else {
            let i = u64::deserialize(deserializer)?;
            CarettaId::from_u64(i).map_err(D::Error::custom)
        }
    }
}
