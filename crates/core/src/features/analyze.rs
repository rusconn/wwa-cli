use crate::Enemy;

#[derive(Debug)]
pub struct Player {
    atk: usize,
    def: usize,
}

#[derive(Debug, PartialEq)]
pub enum Analysis {
    Blocked {
        atk_inc: usize,
    },
    Damage {
        atk_inc: usize,
        atk_reduction: usize,
        def_reduction: usize,
    },
    NoDamage,
}

impl Player {
    pub fn new(atk: usize, def: usize) -> Self {
        Self { atk, def }
    }

    /// Computes damage-change analysis for a single enemy.
    ///
    /// # Example
    ///
    /// ```
    /// use std::num::NonZeroUsize;
    ///
    /// use wwa::{Enemy, Player};
    ///
    /// let enemy = Enemy {
    ///     name: "goblin".to_owned(),
    ///     hp: NonZeroUsize::new(30).unwrap(),
    ///     atk: 10,
    ///     def: 1,
    /// };
    ///
    /// assert_eq!(
    ///     Player::new(11, 5).analyze(&enemy),
    ///     wwa::Analysis::Damage {
    ///         atk_inc: 5,
    ///         atk_reduction: 5,
    ///         def_reduction: 2,
    ///     },
    /// );
    /// ```
    pub fn analyze(&self, enemy: &Enemy) -> Analysis {
        let hp = enemy.hp.get();
        let dmg_dealt = self.atk.saturating_sub(enemy.def);

        if dmg_dealt == 0 {
            return Analysis::Blocked {
                atk_inc: enemy.def + 1 - self.atk,
            };
        }

        let hits = hp.div_ceil(dmg_dealt);
        let dmg_taken = enemy.atk.saturating_sub(self.def);

        if hits == 1 || dmg_taken == 0 {
            return Analysis::NoDamage;
        }

        let new_dealt = hp.div_ceil(hits - 1);
        let breakpoint_atk = new_dealt + enemy.def;
        let new_hits = hp.div_ceil(new_dealt);

        Analysis::Damage {
            atk_inc: breakpoint_atk - self.atk,
            atk_reduction: (hits - new_hits) * dmg_taken,
            def_reduction: hits - 1,
        }
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
    fn test_one_hit_no_damage() {
        let enemy = make_enemy(30, 10, 1);
        let player = Player { atk: 50, def: 5 };
        let analysis = player.analyze(&enemy);
        assert_eq!(analysis, Analysis::NoDamage);
    }

    #[test]
    fn test_no_incoming_damage() {
        let enemy = make_enemy(30, 10, 1);
        let player = Player { atk: 11, def: 10 };
        let analysis = player.analyze(&enemy);
        assert_eq!(analysis, Analysis::NoDamage);
    }

    #[test]
    fn test_attack_does_not_go_through() {
        let enemy = make_enemy(30, 10, 1);
        let player = Player { atk: 1, def: 5 };
        let analysis = player.analyze(&enemy);
        assert_eq!(analysis, Analysis::Blocked { atk_inc: 1 });
    }

    #[test]
    fn test_multiple_hits_reduction() {
        let enemy = make_enemy(6, 10, 1);
        let player = Player { atk: 2, def: 3 };
        let analysis = player.analyze(&enemy);
        assert_eq!(
            analysis,
            Analysis::Damage {
                atk_inc: 1,
                atk_reduction: 21,
                def_reduction: 5,
            },
        );
    }
}
