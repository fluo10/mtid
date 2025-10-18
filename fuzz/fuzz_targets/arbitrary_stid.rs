#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use mtid::Stid;

impl_arbitrary_test!{ Mtid = Stid, Int = u16, }
