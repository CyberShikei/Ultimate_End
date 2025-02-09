// src/game/item.rs
use crate::game::stats::Stats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    // Other types as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
