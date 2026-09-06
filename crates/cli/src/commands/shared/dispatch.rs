use std::io::{self, IsTerminal};

use anyhow::Result;

pub(crate) fn want_interactive(batch: bool, format_specified: bool) -> Result<bool> {
    let interactive = !batch && !format_specified && io::stdout().is_terminal();
    Ok(interactive)
}
