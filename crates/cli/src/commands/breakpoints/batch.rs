use std::{io, path::PathBuf};

use anyhow::{Context, Result};

use wwa::{BreakpointOptions as Options, EnemiesBreakpointExt};

use crate::commands::shared::enemies::load_enemies;

use super::args::Format;

pub(super) struct Config {
    enemies_json5: PathBuf,
    min: Option<usize>,
    max: Option<usize>,
    format: Format,
}

impl Config {
    pub(super) fn new(
        enemies_json5: PathBuf,
        min: Option<usize>,
        max: Option<usize>,
        format: Format,
    ) -> Self {
        Self {
            enemies_json5,
            min,
            max,
            format,
        }
    }
}

pub(super) fn run(config: Config) -> Result<()> {
    let enemies = load_enemies(&config.enemies_json5)?;

    let options = Options::new(config.min, config.max);
    let map = enemies.breakpoints(&options);

    let stdout = io::stdout().lock();
    let mut stdout = io::BufWriter::new(stdout);
    config
        .format
        .writeln(&mut stdout, &map)
        .context("failed to write output")
}
