mod stid;
mod dtid;
mod ttid;

const PACKAGE_NAME: &'static str = "tripod_id";

include!(concat!(env!("OUT_DIR"), "/tripod_id.rs"));

/// Alias of single tripod-id message
pub type StidMessage = Stid;

/// Alias of double tripod-id message
pub type DtidMessage = Dtid;

/// Alias of triple tripod-id message
pub type TtidMessage = Ttid;