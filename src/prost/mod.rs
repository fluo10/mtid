mod stid;
mod dtid;
mod ttid;
mod qtid;

const PACKAGE_NAME: &'static str = "mtid";

include!(concat!(env!("OUT_DIR"), "/mtid.rs"));