#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use caretta-id::CarettaIdS;

impl_arbitrary_test!{ caretta-id = CarettaIdS, Int = u16, }
