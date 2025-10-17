#![no_main]

use libfuzzer_sys::fuzz_target;
use mtid::Qtid;

fuzz_target!(|data: &[u8]| {
    if let Ok(qtid) = str::from_utf8(data) {
        // Ensure the parser doesn't panic
        let _ = qtid.parse::<Qtid>();
    }
});
