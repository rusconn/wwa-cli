use std::{collections::BTreeMap, io::Write};

use wwa::{Breakpoint, Enemy};

use crate::BreakpointsError;

pub(in super::super) fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
) -> Result<(), BreakpointsError> {
    for (i, (breakpoint, enemies)) in map.iter().enumerate() {
        if i > 0 {
            writeln!(w)?;
        }
        write!(w, "{breakpoint}: ")?;
        for (j, enemy) in enemies.iter().enumerate() {
            if j > 0 {
                write!(w, ",")?;
            }
            write!(w, "{}", enemy.name)?;
        }
    }

    Ok(())
}
