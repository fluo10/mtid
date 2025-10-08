mod validator;

use validator::MtidValidator;
use mtid::{Stid, Qtid};
impl MtidValidator for Qtid{
    type Integer = u64;
    type Tuple = (Stid, Stid, Stid, Stid);
}

use rand::Rng;

#[test]
fn nil() {
    assert!(Qtid::NIL.validate_all().unwrap());
    assert_eq!(Qtid::NIL, 0);
    assert_eq!(Qtid::NIL, "000000000000".to_string());
    assert_eq!(Qtid::NIL, "000-000-000-000".to_string());

}

#[test]
fn max() {
    assert!(Qtid::MAX.validate_all().unwrap());
    assert_eq!(Qtid::MAX, Qtid::CAPACITY-1);
    assert_eq!(Qtid::MAX, "zzzzzzzzzzzz".to_string());
    assert_eq!(Qtid::MAX, "ZZZ-ZZZ-ZZZ-zzz".to_string());
    assert_eq!((Stid::MAX, Stid::MAX, Stid::MAX, Stid::MAX), Qtid::MAX.into())
}

#[test]
#[should_panic]
fn over_sized() {
    Qtid::try_from(Qtid::CAPACITY).unwrap();
}

#[test]
fn random() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let id: Qtid = rng.r#gen();
        assert!(id.validate_all().unwrap());
    }
}