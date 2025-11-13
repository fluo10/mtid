#[macro_use]
mod macros;

use caretta_id::CarettaIdD;

use rand::Rng;

impl_tests! {
    Self = CarettaIdD,
    Uint = u32,
}
