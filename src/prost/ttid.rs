use prost::Name;

use crate::{Error, prost::Ttid};

impl Name for Ttid {
    const NAME: &'static str = "Ttid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl From<crate::Ttid> for Ttid {
    fn from(value: crate::Ttid) -> Self {
        Self {
            id: u64::from(value),
        }
    }
}
impl TryFrom<Ttid> for crate::Ttid {
    type Error = Error;

    fn try_from(value: Ttid) -> Result<Self, Self::Error> {
        Self::try_from(value.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Ttid, TtidMessage};

    #[test]
    fn nil() {
        let nil = TtidMessage { id: 0 };
        assert_eq!(Ttid::NIL, Ttid::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = TtidMessage {
            id: u64::from(Ttid::CAPACITY) - 1,
        };
        assert_eq!(Ttid::MAX, Ttid::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized() {
        let oversized = TtidMessage {
            id: u64::from(Ttid::CAPACITY),
        };
        let _ = Ttid::try_from(oversized).unwrap();
    }
}
