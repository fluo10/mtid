use clap::Args;

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
        todo!()
    }
}