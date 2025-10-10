#[macro_use]
mod macros;

use mtid::{Stid, Qtid};

use rand::Rng;


impl_tests!{
    Self = Qtid,
    Integer = u64,
}