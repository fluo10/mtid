#![cfg(feature = "rand")]
mod common;
use caretta_id::*;
use common::*;
use rand::Rng;

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
        let value: u64 = rand::random_range(1..=<CarettaId>::MAX.to_u64());
        let id = <CarettaId>::try_from(value).unwrap();
        assert_conversion(id);
    }
}

#[cfg(feature = "std")]
#[test]
fn random_str() {
    let mut rng = rand::rng();
    for _ in 0..10 {
        let mut buf = ['0'; 7];
        for i in 0..6 {
            let c = rng.sample(rand::distr::Alphanumeric) as char;
            buf[i] = c;
        }
        let s: String = buf.into_iter().collect();
        let id = (&s).parse::<CarettaId>().unwrap();
        assert_conversion(id);
    }
}

#[test]
#[cfg(feature = "rand")]
fn oversized_random_int() {
    for _ in 0..10 {
        let value: u64 = rand::random_range((<CarettaId>::MAX.to_u64() + 1)..<u64>::MAX);
        let _ = <CarettaId>::try_from(value).unwrap_err();
    }
}
