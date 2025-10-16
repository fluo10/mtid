use std::path::PathBuf;
use std::process::Command;

use mtid::{Dtid, Qtid, Stid, Ttid};

macro_rules! assert_encode {
    (
        $length_option:literal,
        $mtid_int:expr,
        $mtid_str:expr
     ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_mtid-cli"));
        let output = Command::new(path)
            .arg("encode")
            .arg($length_option)
            .arg(format!("{}", $mtid_int))
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $mtid_str).into_bytes());
    };
}

#[test]
fn stid() {
    let stid: Stid = rand::random();
    assert_encode!("-s", u16::from(stid), &stid.to_string());
}

#[test]
fn dtid() {
    let dtid: Dtid = rand::random();
    assert_encode!("-d", u32::from(dtid), &dtid.to_string());
}

#[test]
fn ttid() {
    let ttid: Ttid = rand::random();
    assert_encode!("-t", u64::from(ttid), &ttid.to_string());
}

#[test]
fn qtid() {
    let qtid: Qtid = rand::random();
    assert_encode!("-q", u64::from(qtid), &qtid.to_string());
}
