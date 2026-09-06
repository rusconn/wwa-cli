use std::io::Write;

use anyhow::Result;

use wwa::{Analysis, Enemy};

pub(in super::super) fn write(
    w: &mut impl Write,
    analyses: &[Analysis],
    enemies: &[Enemy],
) -> Result<()> {
    let mut first = true;

    for (enemy, analysis) in enemies.iter().zip(analyses) {
        if matches!(analysis, Analysis::NoDamage) {
            continue;
        }

        if !first {
            writeln!(w)?;
        }
        first = false;
        writeln!(w, "{}", enemy.name)?;

        match analysis {
            Analysis::Blocked { atk_inc } => {
                write!(w, "  †+{atk_inc} ⇒ ok")?;
            }
            Analysis::Damage {
                atk_inc,
                atk_reduction,
                def_reduction,
            } => {
                writeln!(w, "  †+{atk_inc} ⇒ -{atk_reduction}")?;
                write!(w, "  ⛨+1 ⇒ -{def_reduction}")?;
            }
            Analysis::NoDamage => {
                unreachable!("NoDamage is skipped by `continue`");
            }
        }
    }

    Ok(())
}
