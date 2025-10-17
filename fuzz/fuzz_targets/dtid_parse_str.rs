#![no_main]

use libfuzzer_sys::fuzz_target;
use mtid::Dtid;

fuzz_target!(|data: &[u8]| {
    if let Ok(dtid) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        let _ = dtid.parse::<Dtid>();
    }
});
