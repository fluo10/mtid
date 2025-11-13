use caretta_id::{CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};
use clap::Args;

use crate::cli::length_option::{LengthOption, LengthOptions};

#[derive(Args, Debug)]
/// Generate random caretta-id
pub struct GenerateArgs {
    #[command(flatten)]
    length: LengthOptions,
}

impl GenerateArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Single => println!("{}", rand::random::<CarettaIdS>()),
            LengthOption::Double => println!("{}", rand::random::<CarettaIdD>()),
            LengthOption::Triple => println!("{}", rand::random::<CarettaIdT>()),
            LengthOption::Quadruple => println!("{}", rand::random::<CarettaIdQ>()),
        }
    }
}
