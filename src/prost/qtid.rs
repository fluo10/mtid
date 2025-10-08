use prost::Name;

use crate::{prost::Qtid, Error};

impl Name for Qtid {
    const NAME: &'static str = "Qtid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl From<crate::Qtid> for Qtid {
    fn from(value: crate::Qtid) -> Self {
        Self {
            id: u64::from(value)
        }
    }
}
impl TryFrom<Qtid> for crate::Qtid {
    type Error = Error;

    fn try_from(value: Qtid) -> Result<Self, Self::Error> {
        Self::try_from(
            value.id
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Qtid, QtidMessage};

    #[test]
    fn nil() {
        let nil = QtidMessage{id: 0};
        assert_eq!(Qtid::NIL, Qtid::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = QtidMessage{id: u64::from(Qtid::CAPACITY)-1};
        assert_eq!(Qtid::MAX, Qtid::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized () {
        let oversized = QtidMessage{id: u64::from(Qtid::CAPACITY)};
        let _ = Qtid::try_from(oversized).unwrap();
    }
}