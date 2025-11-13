#[macro_use]
mod macros;
use std::str::FromStr;

use caretta_id::CarettaIdS;
use rand::Rng;

impl_tests! {
    Self = CarettaIdS,
    Uint = u16,
}

#[test]
fn random_str() {
    for _ in 0..10 {
        let mut rng = rand::rng();
        let value = format!(
            "{}{}{}",
            rng.sample(rand::distr::Alphanumeric) as char,
            rng.sample(rand::distr::Alphanumeric) as char,
            rng.sample(rand::distr::Alphanumeric) as char,
        );
        let _ = CarettaIdS::from_str(&value).unwrap();
    }
}
