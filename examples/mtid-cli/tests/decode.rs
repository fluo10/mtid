use std::path::PathBuf;
use std::process::Command;

use mtid::{Dtid, Qtid, Stid, Ttid};

macro_rules! assert_decode {
    (
        $length_option:literal,
        $mtid_str:expr,
        $mtid_int:expr
     ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_mtid-cli"));
        let output = Command::new(path)
            .arg("decode")
            .arg($length_option)
            .arg($mtid_str)
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $mtid_int).into_bytes());
    };
}

#[test]
fn stid() {
    let stid: Stid = rand::random();
    assert_decode!("-s", &stid.to_string(), u16::from(stid));
}

#[test]
fn dtid() {
    let dtid: Dtid = rand::random();
    assert_decode!("-d", &dtid.to_string(), u32::from(dtid));
}

#[test]
fn ttid() {
    let ttid: Ttid = rand::random();
    assert_decode!("-t", &ttid.to_string(), u64::from(ttid));
}

#[test]
fn qtid() {
    let qtid: Qtid = rand::random();
    assert_decode!("-q", &qtid.to_string(), u64::from(qtid));
}

