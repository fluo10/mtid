mod common;
use std::str::FromStr;

use tripod_id::{Single, TripodId};
use rand::Rng;

use crate::common::TripodIdValidator;

impl TripodIdValidator for Single {}

#[test]
fn nil() {
    assert!(Single::NIL.validate_all().unwrap());
    assert_eq!(Single::NIL, 0);
    assert!(Single::NIL.validate_parse_strings(&["000"]).unwrap());
    assert!(Single::NIL.is_nil());
    assert!(!Single::NIL.is_max())
}

#[test]
fn max() {
    assert!(Single::MAX.validate_all().unwrap());
    assert_eq!(Single::MAX, Single::CAPACITY - 1);
    assert!(Single::MAX.validate_parse_strings(&["zzz", "ZZZ"]).unwrap());
    assert!(Single::MAX.is_max());
    assert!(!Single::MAX.is_nil());
}

#[test]
fn boundary_value() {
    let _ = Single::try_from(Single::CAPACITY-1).unwrap();
    let _ = Single::try_from(Single::CAPACITY).unwrap_err();
}

#[test]
fn random_int() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let single: Single = rng.r#gen();
        assert!(single.validate_all().unwrap());
    }
}

#[test]
fn oversized_random_int() {
    let mut rng = rand::thread_rng();
    let _ = Single::try_from(0).unwrap();
    for _ in 0..10 {
        let value: u16 = rng.gen_range(Single::CAPACITY..u16::MAX);
        let _ = Single::try_from(value).unwrap_err();
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
        let single = Single::from_str(&value).unwrap();
        assert!(single.validate_all().unwrap());
    }
}