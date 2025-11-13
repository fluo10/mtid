#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use caretta-id::CarettaIdQ;

impl_arbitrary_test!{ caretta-id = CarettaIdQ, Int = u64, }
