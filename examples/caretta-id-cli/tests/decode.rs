use std::path::PathBuf;
use std::process::Command;

use caretta_id::{CarettaId, CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

macro_rules! assert_decode {
    (
        $length_option:literal,
        $caretta_id_str:expr,
        $caretta_id_int:expr
     ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
        let output = Command::new(path)
            .arg("decode")
            .arg($length_option)
            .arg($caretta_id_str)
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $caretta_id_int).into_bytes());
    };
}

#[test]
fn single() {
    let single: CarettaIdS = rand::random();
    assert_decode!("-s", &single.to_string(), u16::from(single));
}

#[test]
fn double() {
    let id: CarettaIdD = rand::random();
    assert_decode!("-d", &id.to_string(), u32::from(id));
}

#[test]
fn triple() {
    let id: CarettaIdT = rand::random();
    assert_decode!("-t", &id.to_string(), u64::from(id));
}

#[test]
fn quadruple() {
    let id: CarettaIdQ = rand::random();
    assert_decode!("-q", &id.to_string(), u64::from(id));
}

       
       
       
fn assert_decode(id: CarettaId) {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
    let output = Command::new(path)
        .arg("decode")
        .arg(&id.to_string())
        .output()
        .unwrap()
        .stdout;
    assert_eq!(output, format!("{}\n", id.as_u64()).into_bytes());
}

#[test]
fn nil() {
    assert_decode(CarettaId::NIL);
}

#[test]
fn max() {
    assert_decode(CarettaId::MAX);
}

#[test]
fn random() {
    assert_decode(CarettaId::random());
}