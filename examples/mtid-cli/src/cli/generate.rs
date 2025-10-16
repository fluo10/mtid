use clap::Args;
use mtid::{Dtid, Qtid, Stid, Ttid};

use crate::cli::length_option::{LengthOption, LengthOptions};

#[derive(Args, Debug)]
/// Generate random MTID
#[command(override_usage("mtid-cli generate [<-s|-d|-t|-q>]"))]
pub struct GenerateArgs {
    #[command(flatten)]
    length: LengthOptions,
}

impl GenerateArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Single => println!("{}", rand::random::<Stid>()),
            LengthOption::Double => println!("{}", rand::random::<Dtid>()),
            LengthOption::Triple => println!("{}", rand::random::<Ttid>()),
            LengthOption::Quadruple => println!("{}", rand::random::<Qtid>()),
        }
    }
}
