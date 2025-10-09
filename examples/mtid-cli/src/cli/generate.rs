use clap::Args;

use crate::cli::args::{Length};

#[derive(Args, Debug)]
/// Generate random MTID
#[command(override_usage("mtid-cli generate [<-s|-d|-t|-q>]"))]
pub struct GenerateArgs {
    #[command(flatten)]
    length: Length
}

impl GenerateArgs {
    pub fn run(self) {
        todo!()
    }
}