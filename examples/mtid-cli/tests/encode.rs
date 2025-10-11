use std::process::Command;
use std::path::PathBuf;

use mtid::{Stid, Dtid, Ttid, Qtid};
use rand::Rng;

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
            .arg(format!("{}",$mtid_int))
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $mtid_str).into_bytes());
    };
    ( 
        $mtid_int:expr,
        $mtid_str:expr
     ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_mtid-cli"));
        let output = Command::new(path)
            .arg("encode")
            .arg(format!("{}", $mtid_int))
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $mtid_str).into_bytes());
    };
}

#[test]
fn stid_with_length() {
    let stid: Stid = rand::thread_rng().r#gen();
    assert_encode!("-s", u16::from(stid), &stid.to_string());
}

#[test]
fn stid_without_length() {
    let stid: Stid = rand::thread_rng().r#gen();
    assert_encode!(u16::from(stid), &stid.to_string());
}

#[test]
fn dtid_with_length() {
    let dtid: Dtid = rand::thread_rng().r#gen();
    assert_encode!("-d", u32::from(dtid), &dtid.to_string());
}

#[test]
fn dtid_without_length() {
    let dtid: Dtid = rand::thread_rng().r#gen();
    assert_encode!(u32::from(dtid), &dtid.to_string());
}

#[test]
fn ttid_with_length() {
    let ttid: Ttid = rand::thread_rng().r#gen();
    assert_encode!("-t", u64::from(ttid), &ttid.to_string());
}

#[test]
fn ttid_without_length() {
    let ttid: Ttid = rand::thread_rng().r#gen();
    assert_encode!(u64::from(ttid), &ttid.to_string());
}

#[test]
fn qtid_with_length() {
    let qtid: Qtid = rand::thread_rng().r#gen();
    assert_encode!("-q", u64::from(qtid), &qtid.to_string());
}

#[test]
fn qtid_without_length() {
    let qtid: Qtid = rand::thread_rng().r#gen();
    assert_encode!(u64::from(qtid), &qtid.to_string());
}
