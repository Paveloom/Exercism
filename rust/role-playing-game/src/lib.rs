// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    #[must_use]
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Player {
                health: 100,
                mana: if self.level < 10 { None } else { Some(100) },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.unwrap_or_default() < mana_cost {
            if self.level < 10 {
                if self.health < mana_cost {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
            }
            0
        } else {
            if let Some(mana) = self.mana {
                self.mana = Some(mana - mana_cost);
            }
            mana_cost * 2
        }
    }
}
