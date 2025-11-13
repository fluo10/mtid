//! Test for serde feature
#![cfg(feature = "serde")]

use caretta_id::{CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

use serde::{Deserialize, Serialize};

use serde_test::{Token, assert_tokens};

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

#[test]
fn struct_nil() {
    assert_tokens(
        &CarettaIdList {
            s: CarettaIdS::NIL,
            d: CarettaIdD::NIL,
            t: CarettaIdT::NIL,
            q: CarettaIdQ::NIL,
        },
        &[
            Token::Struct {
                name: "CarettaIdList",
                len: 4,
            },
            Token::Str("s"),
            Token::Str("000"),
            Token::Str("d"),
            Token::Str("000-000"),
            Token::Str("t"),
            Token::Str("000-000-000"),
            Token::Str("q"),
            Token::Str("000-000-000-000"),
            Token::StructEnd,
        ],
    )
}
