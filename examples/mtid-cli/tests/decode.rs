use std::process::Command;
use std::path::PathBuf;

use mtid::{Stid, Dtid, Ttid, Qtid};
use rand::Rng;

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
    ( 
        $mtid_str:expr,
        $mtid_int:expr
     ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_mtid-cli"));
        let output = Command::new(path)
            .arg("decode")
            .arg($mtid_str)
            .output()
            .unwrap()
            .stdout;
        assert_eq!(output, format!("{}\n", $mtid_int).into_bytes());
    };
}

#[test]
fn stid_with_length() {
    let stid: Stid = rand::thread_rng().r#gen();
    assert_decode!("-s", &stid.to_string(), u16::from(stid));
}

#[test]
fn stid_without_length() {
    let stid: Stid = rand::thread_rng().r#gen();
    assert_decode!(&stid.to_string(), u16::from(stid));
}

#[test]
fn dtid_with_length() {
    let dtid: Dtid = rand::thread_rng().r#gen();
    assert_decode!("-d", &dtid.to_string(), u32::from(dtid));
}

#[test]
fn dtid_without_length() {
    let dtid: Dtid = rand::thread_rng().r#gen();
    assert_decode!(&dtid.to_string(), u32::from(dtid));
}

#[test]
fn ttid_with_length() {
    let ttid: Ttid = rand::thread_rng().r#gen();
    assert_decode!("-t", &ttid.to_string(), u64::from(ttid));
}

#[test]
fn ttid_without_length() {
    let ttid: Ttid = rand::thread_rng().r#gen();
    assert_decode!(&ttid.to_string(), u64::from(ttid));
}

#[test]
fn qtid_with_length() {
    let qtid: Qtid = rand::thread_rng().r#gen();
    assert_decode!("-q", &qtid.to_string(), u64::from(qtid));
}

#[test]
fn qtid_without_length() {
    let qtid: Qtid = rand::thread_rng().r#gen();
    assert_decode!(&qtid.to_string(), u64::from(qtid));
}
