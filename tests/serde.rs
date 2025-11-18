//! Test for serde feature
#![cfg(feature = "serde")]

use caretta_id::{CarettaId, CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

use serde::{Deserialize, Serialize};

use serde_test::{Configure, Token, assert_tokens};

#[test]
fn nil_readable() {
    assert_tokens(&CarettaId::NIL.readable(), &[Token::Str("0000000")])
}

#[test]
fn max_readable() {
    assert_tokens(&CarettaId::MAX.readable(), &[Token::Str("zzzzzzz")]);
}

#[test]
fn nil_compact() {
    assert_tokens(&CarettaId::NIL.compact(), &[Token::U64(0)])
}

#[test]
fn max_compact() {
    assert_tokens(&CarettaId::MAX.compact(), &[Token::U64(0x7FFFFFFFF)]);
}
