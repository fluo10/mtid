use prost::Name;

use crate::{Error, proto::Stid};

impl Name for Stid {
    const NAME: &'static str = "Stid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl From<crate::Stid> for Stid {
    fn from(value: crate::Stid) -> Self {
        Self {
            value: u32::from(u16::from(value)),
        }
    }
}
impl TryFrom<Stid> for crate::Stid {
    type Error = Error;

    fn try_from(value: Stid) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value.value).or(Err(Error::ParseInteger {
            expected: u64::from(crate::Stid::CAPACITY),
            found: value.value as u64,
        }))?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Stid, StidMessage};

    #[test]
    fn nil() {
        let nil = StidMessage { value: 0 };
        assert_eq!(Stid::NIL, Stid::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = StidMessage {
            value: u32::from(Stid::CAPACITY) - 1,
        };
        assert_eq!(Stid::MAX, Stid::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized() {
        let oversized = StidMessage {
            value: u32::from(Stid::CAPACITY),
        };
        let _ = Stid::try_from(oversized).unwrap();
    }
}
