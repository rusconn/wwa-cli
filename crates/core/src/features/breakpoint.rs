use std::collections::BTreeMap;

use rustc_hash::FxHashMap;

use crate::Enemy;

#[derive(Debug, Default)]
pub struct Options {
    min: Option<usize>,
    max: Option<usize>,
}

impl Options {
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }
}

pub type Breakpoint = usize;

/// Extension trait for [[`Enemy`]] to compute a breakpoint map.
pub trait EnemiesBreakpointExt {
    /// Computes attack value breakpoints for multiple enemies.
    ///
    /// # Example
    ///
    /// ```
    /// use std::{
    ///     collections::BTreeMap,
    ///     num::NonZeroUsize,
    /// };
    ///
    /// use wwa::{Enemy, BreakpointOptions, EnemiesBreakpointExt};
    ///
    /// let enemies = [
    ///     Enemy {
    ///         name: "goblin".to_owned(),
    ///         hp: NonZeroUsize::new(30).unwrap(),
    ///         atk: 10,
    ///         def: 1,
    ///     },
    ///     Enemy {
    ///         name: "wolf".to_owned(),
    ///         hp: NonZeroUsize::new(25).unwrap(),
    ///         atk: 12,
    ///         def: 3,
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     enemies.breakpoints(&BreakpointOptions::new(Some(11), Some(20))),
    ///     BTreeMap::from_iter([
    ///         (11, vec![&enemies[0]]),
    ///         (12, vec![&enemies[1]]),
    ///         (16, vec![&enemies[0], &enemies[1]]), // enemies preserve the original order of the `enemies` slice
    ///     ]),
    /// );
    ///
    /// assert_eq!(
    ///     enemies.breakpoints(&BreakpointOptions::new(Some(20), Some(11))),
    ///     BTreeMap::new(), // empty if min > max
    /// );
    /// ```
    fn breakpoints(&self, options: &Options) -> BTreeMap<Breakpoint, Vec<&Enemy>>;
}

impl EnemiesBreakpointExt for [Enemy] {
    fn breakpoints(&self, options: &Options) -> BTreeMap<Breakpoint, Vec<&Enemy>> {
        let mut map = FxHashMap::<Breakpoint, Vec<&Enemy>>::default();

        for enemy in self {
            for breakpoint in enemy.breakpoints(options) {
                map.entry(breakpoint).or_default().push(enemy);
            }
        }

        map.into_iter().collect()
    }
}

impl Enemy {
    /// Computes attack value breakpoints for this enemy.
    ///
    /// # Example
    ///
    /// ```
    /// use std::{
    ///     collections::BTreeMap,
    ///     num::NonZeroUsize,
    /// };
    ///
    /// use wwa::{Enemy, BreakpointOptions};
    ///
    /// let enemy = Enemy {
    ///     name: "goblin".to_owned(),
    ///     hp: NonZeroUsize::new(30).unwrap(),
    ///     atk: 10,
    ///     def: 1,
    /// };
    ///
    /// assert_eq!(
    ///     enemy.breakpoints(&BreakpointOptions::new(Some(11), Some(20))),
    ///     vec![11, 16], // ascending order
    /// );
    ///
    /// assert_eq!(
    ///     enemy.breakpoints(&BreakpointOptions::new(Some(20), Some(11))),
    ///     vec![], // empty if min > max
    /// );
    /// ```
    pub fn breakpoints(&self, options: &Options) -> Vec<Breakpoint> {
        let hp = self.hp.get();
        let def = self.def;

        let min = options.min.unwrap_or(1);
        let max = options.max.unwrap_or(hp + def); // TODO: handle overflow

        // hp  def  n    atk
        //  8   3   8 ->  4
        //          7 ->  5
        //          6 ->  5
        //          5 ->  5
        //          4 ->  5
        //          3 ->  6
        //          2 ->  7,8,9,10
        //          1 -> 11,12,13,...
        let mut breakpoints = Vec::new();
        let mut n = if min <= def {
            hp
        } else {
            hp.div_ceil(min - def)
        };
        while n > 0 {
            let n_times_min_atk = hp.div_ceil(n) + def;
            let same_atk_min_n = hp.div_ceil(n_times_min_atk - def);
            n = same_atk_min_n - 1;
            if n_times_min_atk < min {
                continue;
            }
            if max < n_times_min_atk {
                break;
            }
            breakpoints.push(n_times_min_atk);
        }

        breakpoints
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    fn make_enemy(hp: usize, atk: usize, def: usize) -> Enemy {
        Enemy {
            name: "test".to_string(),
            hp: NonZeroUsize::new(hp).unwrap(),
            atk,
            def,
        }
    }

    #[test]
    fn test_breakpoints_without_options() {
        let enemy = make_enemy(10, 10, 5);
        let options = Options::default();
        let breakpoints = enemy.breakpoints(&options);
        assert_eq!(breakpoints, vec![6, 7, 8, 9, 10, 15]);
    }

    #[test]
    fn test_breakpoints_edge() {
        let enemy = make_enemy(8, 10, 3);
        let options = Options::new(Some(7), None);
        let breakpoints = enemy.breakpoints(&options);
        assert_eq!(breakpoints, vec![7, 11]);

        let options = Options::new(Some(8), None);
        let breakpoints = enemy.breakpoints(&options);
        assert_eq!(breakpoints, vec![11]);
    }
}
