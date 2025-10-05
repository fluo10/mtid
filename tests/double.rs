mod common;
use tripod_id::{Double, TripodId};

use common::TripodIdValidator;
use rand::Rng;

impl TripodIdValidator for Double {}


#[test]
fn nil() {
    assert!(Double::NIL.validate_all().unwrap());
    assert_eq!(Double::NIL, 0);
    assert_eq!(Double::NIL, "000000".to_string());
    assert_eq!(Double::NIL, "000-000".to_string());
    assert!(Double::NIL.is_nil());
    assert!(!Double::NIL.is_max());

}

#[test]
fn max() {
    assert!(Double::MAX.validate_all().unwrap());
    assert_eq!(Double::MAX, Double::CAPACITY-1);
    assert_eq!(Double::MAX, "zzzzzz".to_string());
    assert_eq!(Double::MAX, "ZZZ-ZZZ".to_string());
    assert!(Double::MAX.is_max());
    assert!(!Double::MAX.is_nil());
}

#[test]
#[should_panic]
fn over_sized() {
    Double::try_from(Double::CAPACITY).unwrap();
}

#[test]
fn random() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let id: Double = rng.r#gen();
        assert!(id.validate_all().unwrap());
    }
}
