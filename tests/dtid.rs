#[macro_use]
mod macros;

use mtid::Dtid;

use rand::Rng;

impl_tests! {
    Self = Dtid,
    Uint = u32,
}
