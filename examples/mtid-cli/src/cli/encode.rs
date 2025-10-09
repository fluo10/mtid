use clap::Args;

use crate::cli::args::{Length};

/// Encode integer to MTID string.
/// 
/// If variant flag is not set, most small variant will returned.
#[derive(Args, Debug)]
pub struct EncodeArgs{
    #[command(flatten)]
    length: Option<Length>
}

impl EncodeArgs {
    pub fn run(self) {
        todo!()
    }
}