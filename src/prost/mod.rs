mod dtid;
mod qtid;
mod stid;
mod ttid;

const PACKAGE_NAME: &'static str = "mtid";

include!(concat!(env!("OUT_DIR"), "/mtid.rs"));
