// src/game/item.rs
use crate::game::stats::Stats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    // Other types as needed

    // comparison paramenter
    // ItemType::Weapon > ItemType::Armor
    // ItemType::Armor > ItemType::Consumable
    // ItemType::Consumable > ItemType::Weapon
    // ItemType::Weapon == ItemType::Weapon
    // ItemType::Armor == ItemType::Armor
    // ItemType::Consumable == ItemType::Consumable
    // ItemType::Weapon != ItemType::Armor
    // ItemType::Weapon != ItemType::Consumable
    // ItemType::Armor != ItemType::Consumable
    // ItemType::Armor != ItemType::Weapon
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
    pub fn new(
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

    // Implement PartialEq for Item to compare items by id.
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn stat_modifier(&self) -> &Stats {
        &self.stat_modifier
    }

    pub fn get_item_string(&self) -> String {
        format!(
            "Name: {}\nDescription: {}\nType: {:?}\nStats:\n{}",
            self.name,
            self.description,
            self.item_type,
            self.stat_modifier.get_stats_string()
        )
    }
}
