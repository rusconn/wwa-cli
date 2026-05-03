use thiserror::Error;

use crate::BreakpointsError;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Breakpoints(#[from] BreakpointsError),
}
