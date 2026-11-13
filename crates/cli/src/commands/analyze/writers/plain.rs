use std::io::Write;

use anyhow::Result;

use wwa::{Analysis, Enemy};

pub(in super::super) fn write(
    w: &mut impl Write,
    analyses: &[Analysis],
    enemies: &[Enemy],
) -> Result<()> {
    let mut iter = enemies
        .iter()
        .zip(analyses)
        .filter(|(_, analysis)| !matches!(analysis, Analysis::NoDamage));

    if let Some((enemy, analysis)) = iter.next() {
        write_block(w, enemy, analysis)?;
        for (enemy, analysis) in iter {
            writeln!(w)?;
            write_block(w, enemy, analysis)?;
        }
    }

    Ok(())
}

fn write_block(w: &mut impl Write, enemy: &Enemy, analysis: &Analysis) -> Result<()> {
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
            unreachable!("already filtered by caller");
        }
    }

    Ok(())
}
