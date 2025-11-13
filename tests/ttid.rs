#[macro_use]
mod macros;

use caretta_id::CarettaIdT;
use rand::Rng;

impl_tests! {
    Self = CarettaIdT,
    Uint = u64,
}
