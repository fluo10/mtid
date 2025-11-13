use clap::Args;
use caretta_id::{CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};

use crate::cli::length_option::{LengthOption, LengthOptions};

/// Decode caretta-id string to integer.
#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Forse to parse specified variants.
    #[command(flatten)]
    length: LengthOptions,
    /// Caretta id String
    ///
    /// Examples: "123" "456-789" "abc-def-ghj" "kmn-pqr-stv-wxy"
    caretta_id: String,
}

impl DecodeArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Single => println!("{}", u16::from(self.caretta_id.parse::<CarettaIdS>().unwrap())),
            LengthOption::Double => println!("{}", u32::from(self.caretta_id.parse::<CarettaIdD>().unwrap())),
            LengthOption::Triple => println!("{}", u64::from(self.caretta_id.parse::<CarettaIdT>().unwrap())),
            LengthOption::Quadruple => {
                println!("{}", u64::from(self.caretta_id.parse::<CarettaIdQ>().unwrap()))
            }
        }
    }
}
