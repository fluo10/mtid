use clap::Args;
use mtid::{Dtid, Qtid, Stid, Ttid};
use rand::Rng;

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
            LengthOption::Single | LengthOption::Unset => {
                println!("{}", rand::thread_rng().r#gen::<Stid>())
            }
            LengthOption::Double => println!("{}", rand::thread_rng().r#gen::<Dtid>()),
            LengthOption::Triple => println!("{}", rand::thread_rng().r#gen::<Ttid>()),
            LengthOption::Quadruple => println!("{}", rand::thread_rng().r#gen::<Qtid>()),
        }
    }
}
