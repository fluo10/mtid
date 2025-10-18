#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use mtid::Ttid;

impl_arbitrary_test!{ Mtid = Ttid, Int = u64, }
