#![cfg(feature = "rand")]
mod common;
use caretta_id::*;
use common::*;

#[test]
fn random() {
    for _ in 0..10 {
        let id: CarettaId = rand::random();
        assert!(<CarettaId>::NIL < id);
        assert!(<CarettaId>::MAX >= id);
    }
}

#[test]
fn random_int() {
    for _ in 0..10 {
        let value: u64 = rand::random_range(1..<CarettaId>::CAPACITY);
        let id = <CarettaId>::try_from(value).unwrap();
        assert_integer_conversion(id);
        assert_string_convertion(id);
    }
}

#[test]
#[cfg(feature = "rand")]
fn oversized_random_int() {
    for _ in 0..10 {
        let value: u64 = rand::random_range(<CarettaId>::CAPACITY..<u64>::MAX);
        let _ = <CarettaId>::try_from(value).unwrap_err();
    }
}
