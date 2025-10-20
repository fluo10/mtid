use prost::Name;

use crate::proto::Qtid;

impl Name for Qtid {
    const NAME: &'static str = "Qtid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

#[cfg(test)]
mod tests {
    use crate::{Qtid, QtidMessage};

    #[test]
    fn nil() {
        let nil = QtidMessage { value: 0 };
        assert_eq!(Qtid::NIL, Qtid::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = QtidMessage {
            value: u64::from(Qtid::CAPACITY) - 1,
        };
        assert_eq!(Qtid::MAX, Qtid::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized() {
        let oversized = QtidMessage {
            value: u64::from(Qtid::CAPACITY),
        };
        let _ = Qtid::try_from(oversized).unwrap();
    }
}
