mod args;
mod encode;
mod decode;
mod generate;

use clap::{Parser, Subcommand};

use crate::cli::{args::LengthOptions, decode::DecodeArgs, encode::EncodeArgs, generate::GenerateArgs};

#[derive(Debug, Parser)]
#[command(version,about, long_about, infer_subcommands =true)]
pub struct Cli {
    #[command(subcommand)]
    command: CliSubcommand,
}

impl Cli {
    pub fn run(self) {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    Decode(DecodeArgs),
    Encode(EncodeArgs),
    Generate(GenerateArgs),
}

impl CliSubcommand {
    pub fn run(self) {
        match self {
            Self::Decode(x) => x.run(),
            Self::Encode(x) => x.run(),

            Self::Generate(x) => x.run()
        }
    }
}