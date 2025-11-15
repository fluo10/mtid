mod common;
use caretta_id::*;
use common::*;

#[test]
fn nil_string_convertion() {
    assert_string_convertion(<CarettaId>::NIL);
}
#[test]
fn nil_integer_conversion() {
    assert_integer_conversion(<CarettaId>::NIL);
}

#[test]
fn max_string_convertion() {
    assert_string_convertion(<CarettaId>::MAX);
}
#[test]
fn max_integer_conversion() {
    assert_integer_conversion(<CarettaId>::MAX);
}

#[test]
fn boundary_value() {
    let _ = <CarettaId>::try_from(<CarettaId>::CAPACITY - 1).unwrap();
    let _ = <CarettaId>::try_from(<CarettaId>::CAPACITY).unwrap_err();
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
