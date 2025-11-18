//! Test for serde feature
#![cfg(feature = "serde")]

use caretta_id::{CarettaId, CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

use serde::{Deserialize, Serialize};

use serde_test::assert_de_tokens;
use serde_test::{Compact, Configure, Readable, Token, assert_ser_tokens, assert_tokens};

fn assert_tokens_readable<'de>(value: &Readable<CarettaId>, tokens: &'de [Token]) {
    assert_ser_tokens(value, tokens);
    #[cfg(feature = "std")]
    assert_de_tokens(value, tokens);
}
fn assert_tokens_compact<'de>(value: &Compact<CarettaId>, tokens: &'de [Token]) {
    assert_tokens(value, tokens);
}

#[test]
fn nil_readable() {
    assert_tokens_readable(&CarettaId::NIL.readable(), &[Token::Str("0000000")])
}

#[test]
fn max_readable() {
    assert_tokens_readable(&CarettaId::MAX.readable(), &[Token::Str("zzzzzzz")]);
}

#[test]
fn nil_compact() {
    assert_tokens_compact(&CarettaId::NIL.compact(), &[Token::U64(0)])
}

#[test]
fn max_compact() {
    assert_tokens_compact(&CarettaId::MAX.compact(), &[Token::U64(0x7FFFFFFFF)]);
}
