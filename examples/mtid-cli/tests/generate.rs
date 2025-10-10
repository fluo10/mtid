use std::{path::PathBuf, process::Command};
use mtid::{Stid, Dtid, Ttid, Qtid};

macro_rules! assert_generate {
    (
        $length_option:literal,
        $Mtid:ty
    ) => {
        let path = PathBuf::from(std::env!("CARGO_BIN_EXE_mtid-cli"));
        let output = String::from_utf8_lossy(&Command::new(path)
            .arg("generate")
            .arg($length_option)
            .output()
            .unwrap()
            .stdout).trim().to_owned();
        let _ = output.parse::<$Mtid>().unwrap();  
    };
}

#[test]
fn default() {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_mtid-cli"));
    let output = String::from_utf8_lossy(&Command::new(path)
        .arg("generate")
        .output()
        .unwrap()
        .stdout).trim().to_owned();

    let _ = output.parse::<Stid>().unwrap();

}
#[test]
fn stid() {
    assert_generate!("-s", Stid);
}

#[test]
fn dtid() {
    assert_generate!("-d", Dtid);
}
#[test]
fn ttid() {
    assert_generate!("-t", Ttid);
}
#[test]
fn qtid() {
    assert_generate!("-q", Qtid);
}
