//! Test for serde feature
#![cfg(feature = "serde")]

use mtid::{Stid, Dtid, Ttid, Qtid};

use serde::{Serialize, Deserialize };

use serde_test::{assert_tokens, Token};

#[test]
fn stid_nil() {
    assert_tokens(&Stid::NIL, &[Token::Str("000")]);
}

#[test]
fn dtid_nil() {
    assert_tokens(&Dtid::NIL, &[Token::Str("000-000")]);
}

#[test]
fn ttid_nil() {
    assert_tokens(&Ttid::NIL, &[Token::Str("000-000-000")]);
}

#[test]
fn qtid_nil() {
    assert_tokens(&Qtid::NIL, &[Token::Str("000-000-000-000")]);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Mtids {
    stid: Stid,
    dtid: Dtid,
    ttid: Ttid,
    qtid: Qtid,
}

#[test]
fn struct_nil () {
    assert_tokens(
        &Mtids {
            stid: Stid::NIL,
            dtid: Dtid::NIL,
            ttid: Ttid::NIL,
            qtid: Qtid::NIL,
        },
        &[
            Token::Struct { name: "Mtids", len: 4},
            Token::Str("stid"),
            Token::Str("000"),
            Token::Str("dtid"),
            Token::Str("000-000"),
            Token::Str("ttid"),
            Token::Str("000-000-000"),
            Token::Str("qtid"),
            Token::Str("000-000-000-000"),
            Token::StructEnd,
        ]
    )
}
