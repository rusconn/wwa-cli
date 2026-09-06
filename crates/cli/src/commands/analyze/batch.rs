use std::{io, path::PathBuf};

use anyhow::{Context, Result};

use wwa::Player;

use crate::commands::shared::enemies::load_enemies;

use super::args::Format;

pub(super) struct Config {
    enemies_json5: PathBuf,
    atk: usize,
    def: usize,
    format: Format,
}

impl Config {
    pub(super) fn new(enemies_json5: PathBuf, atk: usize, def: usize, format: Format) -> Self {
        Self {
            enemies_json5,
            atk,
            def,
            format,
        }
    }
}

pub(super) fn run(config: Config) -> Result<()> {
    let enemies = load_enemies(&config.enemies_json5)?;

    let player = Player::new(config.atk, config.def);
    let analyses = enemies
        .iter()
        .map(|enemy| player.analyze(enemy))
        .collect::<Vec<_>>();

    let stdout = io::stdout().lock();
    let mut stdout = io::BufWriter::new(stdout);
    config
        .format
        .writeln(&mut stdout, &analyses, &enemies)
        .context("failed to write output")
}
