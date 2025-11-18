use super::*;
use ::serde::{Deserialize, Serialize, de::Error};

impl Serialize for CarettaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        if serializer.is_human_readable() {
            let chars= self.to_chars();
            let mut buf =  [0;7];
            for i in 0..7 {
                buf[i] = u8::try_from(chars[i]).unwrap();
            }
            let s = unsafe {
                str::from_utf8_unchecked(&buf)
            };
            serializer.serialize_str(s)
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
            #[cfg(feature = "std")]
            {
                (&<String as Deserialize>::deserialize(deserializer)?)
                .parse::<CarettaId>()
                .map_err(D::Error::custom)
            }
            #[cfg(not(feature = "std"))]
            {
                (<&str as Deserialize>::deserialize(deserializer)?)
                .parse::<CarettaId>()
                .map_err(D::Error::custom)
            }
        } else {
            let i = u64::deserialize(deserializer)?;
            CarettaId::from_u64(i).map_err(D::Error::custom)
        }
    }
}
