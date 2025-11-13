#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use caretta_id::{CarettaIdS, CarettaIdD, CarettaIdT, CarettaIdQ};

fuzz_target!(|data: u64| {
    if let Ok(x) = CarettaIdS::try_from(data as u16) {
        validate!{
            id = x,
            caretta_id = CarettaIdS,
            Int = u16
        };
    }

    if let Ok(x) = CarettaIdD::try_from(data as u32) {
        validate!{
            id = x,
            caretta_id = CarettaIdD,
            Int = u32
        };
    }

    if let Ok(x) = CarettaIdT::try_from(data as u64) {
        validate!{
            id = x,
            caretta_id = CarettaIdT,
            Int = u64
        };
    }

    if let Ok(x) = CarettaIdQ::try_from(data as u64) {
        validate!{
            id = x,
            caretta_id = CarettaIdQ,
            Int = u64
        };
    }
});
