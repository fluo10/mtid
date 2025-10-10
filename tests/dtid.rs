#[macro_use]
mod macros;

use mtid::{Dtid, Stid};

use rand::Rng;


impl_tests!{
    Self = Dtid,
    Integer = u32,
}