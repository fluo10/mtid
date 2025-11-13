#[macro_use]
mod macros;

use mtid::Ttid;
use rand::Rng;

impl_tests! {
    Self = Ttid,
    Uint = u64,
}
