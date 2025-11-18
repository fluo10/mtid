mod common;
use caretta_id::*;
use common::*;

#[test]
fn nil_conversion() {
    assert_conversion(<CarettaId>::NIL);
}

#[test]
fn max_conversion() {
    assert_conversion(<CarettaId>::MAX);
}

#[test]
fn boundary_value() {
    let max_value = CarettaId::MAX.to_u64();
    let _ = <CarettaId>::try_from(max_value).unwrap();
    let _ = <CarettaId>::try_from(max_value + 1).unwrap_err();
}

#[test]
fn partial_ord() {
    assert!(<CarettaId>::NIL < <CarettaId>::MAX);
}

#[test]
fn sort() {
    let mut vec = vec![<CarettaId>::MAX, <CarettaId>::NIL];
    vec.sort();
    assert_eq!(vec[0], <CarettaId>::NIL);
    assert_eq!(vec[1], <CarettaId>::MAX);
}
