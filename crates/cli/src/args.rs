use clap::{Parser, Subcommand};

use crate::BreakpointsArgs;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List breakpoint atks
    Breakpoints(BreakpointsArgs),
}
