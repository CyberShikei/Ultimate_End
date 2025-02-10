// src/game/entity.rs
use crate::game::{item::Item, skills::Skill, stats::Stats};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub name: String,
    pub stats: Stats,
    pub inventory: Vec<Item>,
    pub equipment: Vec<Item>,
    pub skills: Vec<Skill>,
}

impl Entity {
    // Methods for entity actions (attack, take damage, etc.) go here.
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            stats: Stats::new(),
            inventory: Vec::new(),
            equipment: Vec::new(),
            skills: Vec::new(),
        }
    }

    pub fn damage_roll(&self, skill: &Skill) -> u32 {
        let mut rng = rand::thread_rng();
        let dmg_roll: u32 = rng.gen_range(0..skill.power);
        let atck: u32 = self.stats.attack as u32;
        let damage = dmg_roll + atck;
        damage
    }

    // Apply item stat modifiers to an entity
    pub fn _apply_item(&mut self, item: &Item) {
        self.stats.hp += item.stat_modifier.hp;
        self.stats.attack += item.stat_modifier.attack;
        self.stats.defense += item.stat_modifier.defense;
        self.stats.agility += item.stat_modifier.agility;
    }

    fn apply_equipment(&mut self) {
        for item in &self.equipment {
            self.stats.apply_modifier(item.stat_modifier);
            // self.stats.hp += item.stat_modifier.hp;
            // self.stats.attack += item.stat_modifier.attack;
            // self.stats.defense += item.stat_modifier.defense;
            // self.stats.agility += item.stat_modifier.agility;
        }
    }

    fn un_apply_equipment(&mut self) {
        for item in &self.equipment {
            self.stats.hp -= item.stat_modifier.hp;
            self.stats.attack -= item.stat_modifier.attack;
            self.stats.defense -= item.stat_modifier.defense;
            self.stats.agility -= item.stat_modifier.agility;
        }
    }

    pub fn equip_item(&mut self, item: Item) {
        self.un_apply_equipment();
        if self.is_item_in_inventory(&item) {
            if self.is_item_equipped(&item) {
                println!("Item already equipped.");
            } else if self.is_equipment_slot_taken(&item) {
                println!("Item type already equipped.");
            } else {
                let eq_item = item.clone();
                self.equipment.push(eq_item);
                self.remove_item_from_inventory(&item);
                println!("Equipped item: {}", item.name);
            }
        } else {
            println!("Item not in inventory.");
        }
        self.apply_equipment();
    }

    fn remove_item_from_inventory(&mut self, item: &Item) {
        self.inventory.retain(|x| x != item);
    }

    pub fn unequip_item(&mut self, item: Item) {
        if self.equipment.contains(&item) {
            self.equipment.retain(|x| x != &item);
            self.stats.hp -= item.stat_modifier.hp;
            self.stats.attack -= item.stat_modifier.attack;
            self.stats.defense -= item.stat_modifier.defense;
            self.stats.agility -= item.stat_modifier.agility;
            println!("Unequipped item: {}", item.name);
            self.add_item_to_inventory(item);
        }
    }

    // Get entity string for displaying in the UI.
    pub fn get_entity_string(&self) -> String {
        format!(
            "Name: {}\n\tStats:\n{}",
            self.name,
            self.stats.get_stats_string()
        )
    }

    pub fn get_inventory_string(&self) -> String {
        let mut inventory_string = String::new();
        for item in &self.inventory {
            inventory_string.push_str(&format!("ID: {}, Name: {}", item.id, item.name));
        }
        inventory_string
    }

    pub fn get_equipment_string(&self) -> String {
        let mut equipment_string = String::new();
        for item in &self.equipment {
            equipment_string.push_str(&format!("ID: {}, Name: {}", item.id, item.name));
        }
        equipment_string
    }

    pub fn get_skills_string(&self) -> String {
        let mut skills_string = String::new();
        let mut i = 0;
        for skill in &self.skills {
            i += 1;
            skills_string.push_str(&format!("ID: {}, Name: {}", i, skill.name));
        }
        skills_string
    }

    pub fn get_skill(&self, index: usize) -> &Skill {
        let mut i = 0;
        for skill in &self.skills {
            if i == index {
                return skill;
            }
            i += 1;
        }
        panic!("Skill not found.");
    }

    pub fn get_equipment(&self, item_id: usize) -> &Item {
        for item in &self.equipment {
            if item.id == item_id as u32 {
                return item;
            }
        }
        panic!("Item not found in equipment.");
    }

    pub fn get_item(&self, item_id: usize) -> &Item {
        for item in &self.inventory {
            if item.id == item_id as u32 {
                return item;
            }
        }
        panic!("Item not found in inventory.");
    }

    pub fn add_item_to_inventory(&mut self, item: Item) {
        self.inventory.push(item);
    }

    fn is_item_in_inventory(&self, item: &Item) -> bool {
        self.inventory.contains(item)
    }

    fn is_item_equipped(&self, item: &Item) -> bool {
        self.equipment.contains(&item)
    }

    fn is_equipment_slot_taken(&self, item: &Item) -> bool {
        for i in &self.equipment {
            if i.item_type == item.item_type {
                return true;
            }
        }
        false
    }

    fn _is_inventory(&self) -> bool {
        !self.inventory.is_empty()
    }
}
