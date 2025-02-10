// src/game/stats.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub agility: i32,
    // You can add more stats here.
}

impl Stats {
    // Helper methods for applying item effects, calculating damage, etc.
    pub fn new() -> Self {
        Self {
            hp: 0,
            attack: 0,
            defense: 0,
            agility: 0,
        }
    }

    pub fn apply_modifier(&mut self, modifier: Stats) {
        self.hp += modifier.hp;
        self.attack += modifier.attack;
        self.defense += modifier.defense;
        self.agility += modifier.agility;
    }

    // Get stats string for displaying in the UI.
    pub fn get_stats_string(&self) -> String {
        format!(
            "\t\tHP: {}\n\t\tAttack: {}\n\t\tDefense: {}\n\t\tAgility: {}",
            self.hp, self.attack, self.defense, self.agility
        )
    }
}
