// src/game/entity.rs
use crate::game::{item::Item, stats::Stats};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub name: String,
    pub stats: Stats,
    pub inventory: Vec<Item>,
    // You could later extend with equipment or abilities:
    // pub equipment: Equipment,
}

impl Entity {
    // Methods for entity actions (attack, take damage, etc.) go here.
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            stats: Stats::new(),
            inventory: Vec::new(),
        }
    }

    // Apply item stat modifiers to an entity
    pub fn apply_item(&mut self, item: &Item) {
        self.stats.hp += item.stat_modifier.hp;
        self.stats.attack += item.stat_modifier.attack;
        self.stats.defense += item.stat_modifier.defense;
        self.stats.agility += item.stat_modifier.agility;
    }

    // Get entity string for displaying in the UI.
    pub fn get_entity_string(&self) -> String {
        format!(
            "Name: {}\nStats:\n{}",
            self.name,
            self.stats.get_stats_string()
        )
    }

    pub fn get_inventory_string(&self) -> String {
        let mut inventory_string = String::new();
        for item in &self.inventory {
            inventory_string.push_str(&format!("Item: {}\n", item.name));
        }
        inventory_string
    }

    pub fn add_item_to_inventory(&mut self, item: Item) {
        self.inventory.push(item);
    }
}
