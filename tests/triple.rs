mod validator;

use validator::TripodIdValidator;
use tripod_id::{Single, Triple};
impl TripodIdValidator for Triple{
    type Integer = u64;
    type Tuple = (Single, Single, Single);
}

use rand::Rng;

#[test]
fn nil() {
    assert!(Triple::NIL.validate_all().unwrap());
    assert_eq!(Triple::NIL, 0);
    assert_eq!(Triple::NIL, "000000000".to_string());
    assert_eq!(Triple::NIL, "000-000-000".to_string());

}

#[test]
fn max() {
    assert!(Triple::MAX.validate_all().unwrap());
    assert_eq!(Triple::MAX, Triple::CAPACITY-1);
    assert_eq!(Triple::MAX, "zzzzzzzzz".to_string());
    assert_eq!(Triple::MAX, "ZZZ-ZZZ-ZZZ".to_string());
    assert_eq!((Single::MAX, Single::MAX, Single::MAX), Triple::MAX.into())
}

#[test]
#[should_panic]
fn over_sized() {
    Triple::try_from(Triple::CAPACITY).unwrap();
}

#[test]
fn random() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let id: Triple = rng.r#gen();
        assert!(id.validate_all().unwrap());
    }
}