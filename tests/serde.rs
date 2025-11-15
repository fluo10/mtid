//! Test for serde feature
#![cfg(feature = "serde")]

use caretta_id::{CarettaId, CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

use serde::{Deserialize, Serialize};

use serde_test::{Token, assert_tokens};

#[test]
fn nil() {
    assert_tokens(&CarettaId::NIL, &[Token::Str("0000000")])
}

#[test]
fn max() {
    assert_tokens(&CarettaId::MAX, &[Token::Str("zzzzzzz")]);
}

#[test]
fn single_nil() {
    assert_tokens(&CarettaIdS::NIL, &[Token::Str("000")]);
}

#[test]
fn double_nil() {
    assert_tokens(&CarettaIdD::NIL, &[Token::Str("000-000")]);
}

#[test]
fn triple_nil() {
    assert_tokens(&CarettaIdT::NIL, &[Token::Str("000-000-000")]);
}

#[test]
fn quadruple_nil() {
    assert_tokens(&CarettaIdQ::NIL, &[Token::Str("000-000-000-000")]);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CarettaIdList {
    s: CarettaIdS,
    d: CarettaIdD,
    t: CarettaIdT,
    q: CarettaIdQ,
}
