#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use mtid::Qtid;

impl_arbitrary_test!{ Mtid = Qtid, Int = u64, }
