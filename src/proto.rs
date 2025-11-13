//! Provides protobuf messages.

include!(concat!(env!("OUT_DIR"), "/caretta_id.rs"));

impl From<u32> for CarettaIdS {
    fn from(value: u32) -> Self {
        Self{value}
    }
}
impl From<u32> for CarettaIdD {
    fn from(value: u32) -> Self {
        Self{value}
    }
}

impl From<u64> for CarettaIdT {
    fn from(value: u64) -> Self {
        Self {value}
    }
}

impl From<u64> for CarettaIdQ {
    fn from(value: u64) -> Self {
        Self {value}
    }
}