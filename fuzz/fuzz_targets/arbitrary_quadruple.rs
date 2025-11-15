#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use caretta_id::CarettaIdQ;

impl_arbitrary_test!{ caretta_id = CarettaIdQ, Int = u64, }
