use std::{fs, path::Path};

use anyhow::{Context, Result};

use wwa::Enemy;

pub(crate) fn load_enemies(path: &Path) -> Result<Vec<Enemy>> {
    let content = fs::read_to_string(path) //
        .with_context(|| format!("failed to read {}", path.display()))?;

    json5::from_str(&content) //
        .with_context(|| format!("failed to parse {}", path.display()))
}
