use std::io::{self, IsTerminal};

pub(crate) fn want_interactive(batch: bool, format_specified: bool) -> bool {
    !batch && !format_specified && io::stdout().is_terminal()
}
