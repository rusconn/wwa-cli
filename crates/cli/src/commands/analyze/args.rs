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

    /// Output format
    #[arg(long, value_enum, default_value_t)]
    pub(super) format: Format,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, ValueEnum)]
pub(super) enum Format {
    #[default]
    Plain,
    Json,
    JsonPretty,
}
