mod args;
mod batch;
mod tui;
mod writers;

use anyhow::Result;

use super::shared::dispatch::want_interactive;

pub(crate) use args::Args;

pub fn analyze(args: Args) -> Result<()> {
    if want_interactive(args.batch, args.format.is_some()) {
        tui::run(&tui::Config::new(
            args.enemies_json5_path,
            args.atk,
            args.def,
        ))
    } else {
        batch::run(&batch::Config::new(
            args.enemies_json5_path,
            args.atk,
            args.def,
            args.format.unwrap_or_default(),
        ))
    }
}
