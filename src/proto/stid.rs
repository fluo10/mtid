use prost::Name;

use crate::{Error, proto::Stid};

impl Name for Stid {
    const NAME: &'static str = "Stid";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
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
