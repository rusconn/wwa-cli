use std::io::Write;

use anyhow::Result;
use itertools::Itertools;

use wwa::{Analysis, Enemy};

pub(in super::super) fn write(
    w: &mut impl Write,
    analyses: &[Analysis],
    enemies: &[Enemy],
) -> Result<()> {
    let blocks = enemies
        .iter()
        .zip(analyses)
        .filter(|(_, analysis)| !matches!(analysis, Analysis::NoDamage))
        .format_with("\n", |(enemy, analysis), f| match analysis {
            Analysis::Blocked { atk_inc } => {
                f(&format_args!("{}", enemy.name))?;
                f(&format_args!("\n  †+{atk_inc} ⇒ ok"))
            }
            Analysis::Damage {
                atk_inc,
                atk_reduction,
                def_reduction,
            } => {
                f(&format_args!("{}", enemy.name))?;
                f(&format_args!("\n  †+{atk_inc} ⇒ -{atk_reduction}"))?;
                f(&format_args!("\n  ⛨+1 ⇒ -{def_reduction}"))
            }
            Analysis::NoDamage => {
                unreachable!("already filtered");
            }
        });

    write!(w, "{blocks}")?;

    Ok(())
}
