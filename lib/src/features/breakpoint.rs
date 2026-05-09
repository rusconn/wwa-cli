mod options;

use std::collections::BTreeMap;

use rustc_hash::FxHashMap;

use crate::Enemy;

pub use options::Options;

pub type Breakpoint = usize;

/// Computes attack value breakpoints for multiple enemies.
///
/// Returns a map where keys are attack values (breakpoints) and values are
/// lists of enemies that hit a breakpoint at that value.
/// The enemies preserve the original order of the `enemies` slice.
///
/// # Example
///
/// ```
/// use std::{
///     collections::BTreeMap,
///     num::NonZeroUsize,
/// };
///
/// use wwa::{Enemy, BreakpointOptions, breakpoint_map};
///
/// let enemies = [
///     Enemy {
///         name: "goblin".to_owned(),
///         hp: NonZeroUsize::new(30).unwrap(),
///         def: 1,
///     },
///     Enemy {
///         name: "wolf".to_owned(),
///         hp: NonZeroUsize::new(25).unwrap(),
///         def: 3,
///     }
/// ];
///
/// assert_eq!(
///     breakpoint_map(
///         &enemies,
///         &BreakpointOptions::new(Some(11), Some(20))
///     ),
///     BTreeMap::from_iter([
///         (11, vec![&enemies[0]]),
///         (12, vec![&enemies[1]]),
///         (16, vec![&enemies[0],&enemies[1]]), // enemies preserve the original order of the `enemies` slice
///     ]),
/// );
///
/// assert_eq!(
///     breakpoint_map(
///         &enemies,
///         &BreakpointOptions::new(Some(20), Some(11))
///     ),
///     BTreeMap::new(), // if min > max, it returns empty
/// );
/// ```
pub fn breakpoint_map<'a>(
    enemies: &'a [Enemy],
    options: &Options,
) -> BTreeMap<Breakpoint, Vec<&'a Enemy>> {
    let mut map = FxHashMap::<Breakpoint, Vec<&Enemy>>::default();

    for enemy in enemies {
        for breakpoint in breakpoints(enemy, options) {
            map.entry(breakpoint).or_default().push(enemy);
        }
    }

    map.into_iter().collect()
}

fn breakpoints(enemy: &Enemy, options: &Options) -> Vec<Breakpoint> {
    let hp = enemy.hp.get();
    let def = enemy.def;

    let min = options.min.unwrap_or(1);
    let max = options.max.unwrap_or(hp + def); // TODO: handle overflow

    if max <= enemy.def {
        return Vec::new();
    }

    let n_min = hp.div_ceil(max - def);
    let n_max = if min <= def {
        hp
    } else {
        hp.div_ceil(min - def)
    };

    let mut breakpoints = Vec::new();

    // TODO: improve efficiency
    for n in (n_min..=n_max).rev() {
        let atk = hp.div_ceil(n) + def;
        if (min <= atk && atk <= max) && (breakpoints.last() != Some(&atk)) {
            breakpoints.push(atk);
        }
    }

    breakpoints
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    #[test]
    fn test_breakpoints_basic() {
        let enemy = Enemy {
            name: "slime".to_string(),
            hp: NonZeroUsize::new(10).unwrap(),
            def: 5,
        };
        let options = Options::default();
        let bs = breakpoints(&enemy, &options);
        assert_eq!(bs, vec![6, 7, 8, 9, 10, 15]);
    }

    #[test]
    fn test_breakpoints_with_options() {
        let enemy = Enemy {
            name: "slime".to_string(),
            hp: NonZeroUsize::new(10).unwrap(),
            def: 5,
        };
        let options = Options::new(Some(8), Some(12));
        let bs = breakpoints(&enemy, &options);
        assert_eq!(bs, vec![8, 9, 10]);
    }
}
