#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use caretta_id::{CarettaIdS, CarettaIdD, CarettaIdT, CarettaIdQ};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        if let Ok(x) = s.parse::<CarettaIdS>() {
            validate!{
                id = x,
                caretta_id = CarettaIdS,
                Int = u16
            };
        }

        if let Ok(x) = s.parse::<CarettaIdD>() {
            validate!{
                id = x,
                caretta_id = CarettaIdD,
                Int = u32
            };
        }

        if let Ok(x) = s.parse::<CarettaIdT>() {
            validate!{
                id = x,
                caretta_id = CarettaIdT,
                Int = u64
            };
        }

        if let Ok(x) = s.parse::<CarettaIdQ>() {
            validate!{
                id = x,
                caretta_id = CarettaIdQ,
                Int = u64
            };
        }
    }
});
