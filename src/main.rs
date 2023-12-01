mod internal;
mod parser;

use clap::Parser;
use internal::Runtime;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, short)]
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(cli.input).unwrap();
    let mut r = Runtime::new();
    r.run(input);
}
