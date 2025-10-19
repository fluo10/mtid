use prost::Name;

use crate::{Error, proto::Dtid};

impl Name for Dtid {
    const NAME: &'static str = "Dtid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl From<crate::Dtid> for Dtid {
    fn from(value: crate::Dtid) -> Self {
        Self {
            value: u32::from(value),
        }
    }
}
impl TryFrom<Dtid> for crate::Dtid {
    type Error = Error;

    fn try_from(value: Dtid) -> Result<Self, Self::Error> {
        Self::try_from(value.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dtid, DtidMessage};

    #[test]
    fn nil() {
        let nil = DtidMessage { value: 0 };
        assert_eq!(Dtid::NIL, Dtid::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = DtidMessage {
            value: u32::from(Dtid::CAPACITY) - 1,
        };
        assert_eq!(Dtid::MAX, Dtid::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized() {
        let oversized = DtidMessage {
            value: u32::from(Dtid::CAPACITY),
        };
        let _ = Dtid::try_from(oversized).unwrap();
    }
}
