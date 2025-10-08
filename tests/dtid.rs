mod validator;

use mtid::{Dtid, Stid};

use validator::MtidValidator;
use rand::Rng;

impl MtidValidator for Dtid {
    type Integer = u32;
    type Tuple = (Stid, Stid);
}


#[test]
fn nil() {
    assert!(Dtid::NIL.validate_all().unwrap());
    assert_eq!(Dtid::NIL, 0);
    assert_eq!(Dtid::NIL, "000000".to_string());
    assert_eq!(Dtid::NIL, "000-000".to_string());
    assert!(Dtid::NIL.is_nil());
    assert!(!Dtid::NIL.is_max());

}

#[test]
fn max() {
    assert!(Dtid::MAX.validate_all().unwrap());
    assert_eq!(Dtid::MAX, Dtid::CAPACITY-1);
    assert_eq!(Dtid::MAX, "zzzzzz".to_string());
    assert_eq!(Dtid::MAX, "ZZZ-ZZZ".to_string());
    assert!(Dtid::MAX.is_max());
    assert!(!Dtid::MAX.is_nil());
}

#[test]
#[should_panic]
fn over_sized() {
    Dtid::try_from(Dtid::CAPACITY).unwrap();
}

#[test]
fn random() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let id: Dtid = rng.r#gen();
        assert!(id.validate_all().unwrap());
    }
}
