use prost::Name;

use crate::proto::Dtid;

impl Name for Dtid {
    const NAME: &'static str = "Dtid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
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
