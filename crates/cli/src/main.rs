use std::process;

use anyhow::Result;
use clap::Parser;

use wwa_cli::{Args, Commands, breakpoints};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    match args.command {
        Commands::Breakpoints(args) => breakpoints(args),
    }
}
