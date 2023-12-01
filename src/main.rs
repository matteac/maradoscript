mod internal;
mod parser;

use clap::Parser;
use internal::Runtime;
use std::{io::Write, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, short, default_value = None)]
    input: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let mut r = Runtime::new();
    // if input exists, run it
    if let Some(input) = cli.input {
        r.run_file(input);
        return;
    }
    // otherwise, run repl
    loop {
        print!("\n> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        r.run(input);
    }
}
