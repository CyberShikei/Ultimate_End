// src/game/item.rs
use crate::game::stats::Stats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Weapon,
    Armour,
    Consumable,
    // Other types as needed
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    /// The effect this item has on stats.
    /// For equipment, this might be a bonus added to the base stats.
    /// For consumables, it might be applied once.
    pub stat_modifier: Stats,
}

impl Item {
    pub fn _new(
        id: u32,
        name: &str,
        description: &str,
        item_type: ItemType,
        stat_modifier: Stats,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            item_type,
            stat_modifier,
        }
    }

    // Getters
    pub fn _id(&self) -> u32 {
        self.id
    }

    pub fn _name(&self) -> &str {
        &self.name
    }

    pub fn _description(&self) -> &str {
        &self.description
    }

    pub fn _item_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn _stat_modifier(&self) -> &Stats {
        &self.stat_modifier
    }

    pub fn _get_item_string(&self) -> String {
        format!(
            "Name: {}\nDescription: {}\nType: {:?}\nStats:\n{}",
            self.name,
            self.description,
            self.item_type,
            self.stat_modifier.get_stats_string()
        )
    }

    pub fn is_consumable(&self) -> bool {
        self.item_type == ItemType::Consumable
    }
}
