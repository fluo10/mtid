use std::path::PathBuf;
use std::process::Command;

use caretta_id::{CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

macro_rules! assert_encode {
    (
        $length_option:literal,
        $caretta_id_int:expr,
        $caretta_id_str:expr
     ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
        let output = Command::new(path)
            .arg("encode")
            .arg($length_option)
            .arg(format!("{}", $caretta_id_int))
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $caretta_id_str).into_bytes());
    };
}

#[test]
fn single() {
    let id: CarettaIdS = rand::random();
    assert_encode!("-s", u16::from(id), &id.to_string());
}

#[test]
fn double() {
    let id: CarettaIdD = rand::random();
    assert_encode!("-d", u32::from(id), &id.to_string());
}

#[test]
fn triple() {
    let id: CarettaIdT = rand::random();
    assert_encode!("-t", u64::from(id), &id.to_string());
}

#[test]
fn quadruple() {
    let id: CarettaIdQ = rand::random();
    assert_encode!("-q", u64::from(id), &id.to_string());
}
