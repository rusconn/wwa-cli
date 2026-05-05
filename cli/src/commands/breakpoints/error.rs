use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read {path}: {source}")]
    ReadEnemies { path: PathBuf, source: io::Error },

    #[error("failed to parse {path}: {source}")]
    ParseEnemies { path: PathBuf, source: json5::Error },
}
