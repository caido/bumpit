use std::process;

use bumpit::Arguments;
use clap::Parser;

// cargo invokes this binary as `cargo-bumpit bumpit <args>`
// so the parser below is defined with that in mind
#[derive(Parser, Debug)]
#[clap(bin_name = "cargo")]
enum Cli {
    Bumpit(Arguments),
}

fn main() {
    let cli = Cli::parse();
    let Cli::Bumpit(arguments) = cli;

    if let Err(error) = bumpit::apply(arguments) {
        println!("Failed to bump version: {}", error);
        process::exit(1);
    }
}
