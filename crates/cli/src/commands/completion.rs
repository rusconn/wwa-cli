mod args;

use std::io;

use anyhow::Result;
use clap::CommandFactory;

use crate::Args as CliArgs;

pub(crate) use args::Args;

pub fn completion(args: Args) -> Result<()> {
    let mut cmd = CliArgs::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(args.shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}
