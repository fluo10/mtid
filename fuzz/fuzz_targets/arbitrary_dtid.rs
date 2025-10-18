#![no_main]

#[macro_use]
mod macros;

use libfuzzer_sys::fuzz_target;
use mtid::Dtid;

impl_arbitrary_test!{ Mtid = Dtid, Int = u32, }
