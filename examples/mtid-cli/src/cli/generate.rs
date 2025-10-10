use clap::Args;
use mtid::{Stid, Dtid, Ttid, Qtid};
use rand::Rng;

use crate::cli::args::{LengthOptions};

#[derive(Args, Debug)]
/// Generate random MTID
#[command(override_usage("mtid-cli generate [<-s|-d|-t|-q>]"))]
pub struct GenerateArgs {
    #[command(flatten)]
    length: LengthOptions
}

impl GenerateArgs {
    pub fn run(self) {
        if self.length.single {
            println!("{}", rand::thread_rng().r#gen::<Stid>())
        } else if self.length.double {
            println!("{}", rand::thread_rng().r#gen::<Dtid>())
        } else if self.length.triple {
            println!("{}", rand::thread_rng().r#gen::<Ttid>())
        } else if self.length.quadruple {
            println!("{}", rand::thread_rng().r#gen::<Qtid>())
        } else {
            println!("{}", rand::thread_rng().r#gen::<Stid>())
        }
    }
}