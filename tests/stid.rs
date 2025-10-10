#[macro_use]
mod macros;
use std::str::FromStr;

use mtid::Stid;
use rand::Rng;

impl_tests!{
    Self = Stid,
    Integer = u16,
}

#[test]
fn random_str() {
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let value = format!("{}{}{}",
            rng.sample(rand::distributions::Alphanumeric) as char,
            rng.sample(rand::distributions::Alphanumeric) as char,
            rng.sample(rand::distributions::Alphanumeric) as char,
        );
        let _ = Stid::from_str(&value).unwrap();
    }
}