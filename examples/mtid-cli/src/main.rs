#![deprecated(
    note = "This crate has been renamed from 'mtid-cli' to 'caretta-id-cli', please use the new crate"
)]

use clap::Parser;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    args.run();
}
