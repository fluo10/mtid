mod validator;

use validator::MtidValidator;
use mtid::{Stid, Ttid};
impl MtidValidator for Ttid{
    type Integer = u64;
    type Tuple = (Stid, Stid, Stid);
}

use rand::Rng;

#[test]
fn nil() {
    assert!(Ttid::NIL.validate_all().unwrap());
    assert_eq!(Ttid::NIL, 0);
    assert_eq!(Ttid::NIL, "000000000".to_string());
    assert_eq!(Ttid::NIL, "000-000-000".to_string());

}

#[test]
fn max() {
    assert!(Ttid::MAX.validate_all().unwrap());
    assert_eq!(Ttid::MAX, Ttid::CAPACITY-1);
    assert_eq!(Ttid::MAX, "zzzzzzzzz".to_string());
    assert_eq!(Ttid::MAX, "ZZZ-ZZZ-ZZZ".to_string());
    assert_eq!((Stid::MAX, Stid::MAX, Stid::MAX), Ttid::MAX.into())
}

#[test]
#[should_panic]
fn over_sized() {
    Ttid::try_from(Ttid::CAPACITY).unwrap();
}

#[test]
fn random() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let id: Ttid = rng.r#gen();
        assert!(id.validate_all().unwrap());
    }
}