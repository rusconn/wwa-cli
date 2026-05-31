use std::process;

use clap::Parser;

use wwa_cli::{Args, CliError, Commands, breakpoints};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), CliError> {
    match args.command {
        Commands::Breakpoints(args) => breakpoints(args),
    }?;
    Ok(())
}
