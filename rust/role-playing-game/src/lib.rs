// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::cmp;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: self.mana.map(|_| 100).filter(|_| self.level > 9),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana > mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            Some(mana) => 0,
            None => {
                self.health = cmp::max(self.health as i32 - mana_cost as i32, 0) as u32;
                0
            }
        }
    }
}
