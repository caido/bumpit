use std::process;

use bumpit::Arguments;
use clap::Parser;

fn main() {
    let arguments = Arguments::parse();

    if let Err(error) = bumpit::apply(arguments) {
        println!("Failed to bump version: {}", error);
        process::exit(1);
    }
}
