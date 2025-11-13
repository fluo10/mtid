#[macro_use]
mod macros;

use caretta_id::CarettaIdQ;

use rand::Rng;

impl_tests! {
    Self = CarettaIdQ,
    Uint = u64,
}
