use clap::Args;
use mtid::{Dtid, Qtid, Stid, Ttid};

use crate::cli::length_option::{LengthOption, LengthOptions};

/// Decode MTID string to integer.
#[derive(Args, Debug)]
#[command(override_usage("mtid-cli decode [<-s|-d|-t|-q>] <MTID>"))]
pub struct DecodeArgs {
    /// Forse to parse specified variants.
    #[command(flatten)]
    length: LengthOptions,
    /// MDID String
    ///
    /// Examples: "123" "456-789" "abc-def-ghj" "kmn-pqr-stv-wxy"
    mtid: String,
}

impl DecodeArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Single => println!("{}", u16::from(self.mtid.parse::<Stid>().unwrap())),
            LengthOption::Double => println!("{}", u32::from(self.mtid.parse::<Dtid>().unwrap())),
            LengthOption::Triple => println!("{}", u64::from(self.mtid.parse::<Ttid>().unwrap())),
            LengthOption::Quadruple => {
                println!("{}", u64::from(self.mtid.parse::<Qtid>().unwrap()))
            }
        }
    }
}
