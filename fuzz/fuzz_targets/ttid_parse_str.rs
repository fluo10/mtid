#![no_main]

use libfuzzer_sys::fuzz_target;
use mtid::Ttid;

fuzz_target!(|data: &[u8]| {
    if let Ok(ttid) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        let _ = ttid.parse::<Ttid>();
    }
});
