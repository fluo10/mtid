use caretta_id::{CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};
use std::{path::PathBuf, process::Command};

macro_rules! assert_generate {
    (
        $length_option:literal,
        $caretta_id:ty
    ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
        let output = String::from_utf8_lossy(
            &Command::new(path)
                .arg("generate")
                .arg($length_option)
                .output()
                .unwrap()
                .stdout,
        )
        .trim()
        .to_owned();
        let _ = output.parse::<$caretta_id>().unwrap();
    };
}

#[test]
fn single() {
    assert_generate!("-s", CarettaIdS);
}

#[test]
fn double() {
    assert_generate!("-d", CarettaIdD);
}
#[test]
fn triple() {
    assert_generate!("-t", CarettaIdT);
}
#[test]
fn quadruple() {
    assert_generate!("-q", CarettaIdQ);
}
