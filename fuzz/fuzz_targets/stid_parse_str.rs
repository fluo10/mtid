#![no_main]

use libfuzzer_sys::fuzz_target;
use mtid::Stid;

fuzz_target!(|data: &[u8]| {
    if let Ok(stid) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        let _ = stid.parse::<Stid>();
    }
});
