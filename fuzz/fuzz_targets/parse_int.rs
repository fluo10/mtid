#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use mtid::{Stid, Dtid, Ttid, Qtid};

fuzz_target!(|data: u64| {
    if let Ok(x) = Stid::try_from(data as u16) {
        validate!{
            id = x,
            Mtid = Stid,
            Int = u16
        };
    }

    if let Ok(x) = Dtid::try_from(data as u32) {
        validate!{
            id = x,
            Mtid = Dtid,
            Int = u32
        };
    }

    if let Ok(x) = Ttid::try_from(data as u64) {
        validate!{
            id = x,
            Mtid = Ttid,
            Int = u64
        };
    }

    if let Ok(x) = Qtid::try_from(data as u64) {
        validate!{
            id = x,
            Mtid = Qtid,
            Int = u64
        };
    }
});
