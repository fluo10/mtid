mod validator;
use std::str::FromStr;

use mtid::Stid;
use rand::Rng;

use crate::validator::MtidValidator;

impl MtidValidator for Stid {
    type Integer = u16;
    type Tuple = (Stid,);
}

#[test]
fn nil() {
    assert!(Stid::NIL.validate_all().unwrap());
    assert_eq!(Stid::NIL, 0);
    assert_eq!(Stid::NIL, Stid::from_str("000").unwrap());
    assert!(Stid::NIL.is_nil());
    assert!(!Stid::NIL.is_max())
}

#[test]
fn max() {
    assert!(Stid::MAX.validate_all().unwrap());
    assert_eq!(Stid::MAX, Stid::CAPACITY - 1);
    assert_eq!(Stid::MAX, Stid::from_str("zzZ").unwrap());
    assert!(Stid::MAX.is_max());
    assert!(!Stid::MAX.is_nil());
}

#[test]
fn boundary_value() {
    let _ = Stid::try_from(Stid::CAPACITY-1).unwrap();
    let _ = Stid::try_from(Stid::CAPACITY).unwrap_err();
}

#[test]
fn random_int() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let single: Stid = rng.r#gen();
        assert!(single.validate_all().unwrap());
    }
}

#[test]
fn oversized_random_int() {
    let mut rng = rand::thread_rng();
    let _ = Stid::try_from(0).unwrap();
    for _ in 0..10 {
        let value: u16 = rng.gen_range(Stid::CAPACITY..u16::MAX);
        let _ = Stid::try_from(value).unwrap_err();
    }
}

#[test]
fn random_str() {
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let value = format!("{}{}{}",
            rng.sample(rand::distributions::Alphanumeric) as char,
            rng.sample(rand::distributions::Alphanumeric) as char,
            rng.sample(rand::distributions::Alphanumeric) as char,
        );
        let single = Stid::from_str(&value).unwrap();
        assert!(single.validate_all().unwrap());
    }
}