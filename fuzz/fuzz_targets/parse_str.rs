#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use mtid::{Stid, Dtid, Ttid, Qtid};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        if let Ok(x) = s.parse::<Stid>() {
            validate!{
                id = x,
                Mtid = Stid,
                Int = u16
            };
        }

        if let Ok(x) = s.parse::<Dtid>() {
            validate!{
                id = x,
                Mtid = Dtid,
                Int = u32
            };
        }

        if let Ok(x) = s.parse::<Ttid>() {
            validate!{
                id = x,
                Mtid = Ttid,
                Int = u64
            };
        }

        if let Ok(x) = s.parse::<Qtid>() {
            validate!{
                id = x,
                Mtid = Qtid,
                Int = u64
            };
        }

    }
});
