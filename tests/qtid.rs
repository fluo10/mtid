#[macro_use]
mod macros;

use mtid::Qtid;

use rand::Rng;


impl_tests!{
    Self = Qtid,
    Integer = u64,
}