use clap::Parser;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    args.run();
}
