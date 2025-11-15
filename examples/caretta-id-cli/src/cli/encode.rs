use caretta_id::{CarettaId, CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};
use clap::Args;

use crate::cli::length_option::{LengthOption, LengthOptions};

/// Encode integer to caretta-id string.
#[derive(Args, Debug)]
pub struct EncodeArgs {
    #[command(flatten)]
    length: LengthOptions,
    value: u64,
}

impl EncodeArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Single => println!("{}", CarettaIdS::from_uint_lossy(self.value as u16)),
            LengthOption::Double => println!("{}", CarettaIdD::from_uint_lossy(self.value as u32)),
            LengthOption::Triple => println!("{}", CarettaIdT::from_uint_lossy(self.value)),
            LengthOption::Quadruple => println!("{}", CarettaIdQ::from_uint_lossy(self.value)),
            LengthOption::Unspecified => println!("{}", CarettaId::from_u64_lossy(self.value))
        }
    }
}
