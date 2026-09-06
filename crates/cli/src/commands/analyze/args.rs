use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Args {
    /// Path to enemies.json5
    pub(super) enemies_json5_path: PathBuf,

    /// Player attack power
    #[arg(long)]
    pub(super) atk: usize,

    /// Player defense power
    #[arg(long)]
    pub(super) def: usize,

    /// Output format (implies --batch)
    #[arg(long, value_enum)]
    pub(super) format: Option<Format>,

    /// Force batch (non-interactive) mode
    #[arg(long)]
    pub(super) batch: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, ValueEnum)]
pub(super) enum Format {
    #[default]
    Plain,
    Json,
    JsonPretty,
}
