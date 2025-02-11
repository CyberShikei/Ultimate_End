// src/game/persistence.rs
use crate::game::entity::Entity;
use crate::game::{item::Item, skills::Skill, stats::Stats};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub pc_ents: Vec<Entity>,
    pub npc_ents: Vec<Entity>,
    pub items: Vec<Item>,
    pub skills: Vec<Skill>,

    pub players: Vec<Entity>,
    pub enemies: Vec<Entity>,

    pub player_index: usize,
    pub enemy_index: usize,
    // Add additional fields if needed (e.g., current level, settings, etc.)
}

#[derive(Serialize, Deserialize)]
struct RawEntity {
    id: u32,
    name: String,
    stats: Stats,
    inventory: Vec<u32>,
    equipment: Vec<u32>,
    skills: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct EntitiesWrapper {
    entities: Vec<RawEntity>,
}

#[derive(Serialize, Deserialize)]
struct ItemsWrapper {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
struct SkillsWrapper {
    skills: Vec<Skill>,
}

const SPAWN_LIMIT: usize = 10;
const ITEM_DROP_RATE: f32 = 0.5;
impl GameState {
    /// Create a new, empty game state.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            pc_ents: Vec::new(),
            npc_ents: Vec::new(),
            items: Vec::new(),
            skills: Vec::new(),
            players: Vec::new(),
            enemies: Vec::new(),
            player_index: 0,
            enemy_index: 0,
        }
    }

    /// Load the game state from a JSON file at the given path.
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        // Read the file to a string
        let data = fs::read_to_string(path)?;
        // Deserialize the JSON string into a GameState struct
        let state: GameState = serde_json::from_str(&data).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Deserialization error: {}", e),
            )
        })?;
        Ok(state)
    }

    /// Save the current game state to a JSON file at the given path.
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        // Serialize the GameState struct to a pretty JSON string
        let data = serde_json::to_string_pretty(self).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e))
        })?;
        // Write the JSON string to a file
        fs::write(path, data)
    }

    /// Load Enities
    pub fn load_entities(&mut self, path: &str) -> io::Result<()> {
        let data = fs::read_to_string(path)?;
        // entity skills, inventory and equipment are saved as ids or names
        // we need to convert them to actual objects
        // we can do this by iterating over the entities and checking that the ids
        // exist in the GameState Items and Skills
        // if they do, we can replace the id with the actual object
        // if they don't, we must handle it gracefully
        // NEW WAY
        // parse data to replace ids with actual objects
        let wrapper: EntitiesWrapper = serde_json::from_str(&data).map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Deserialization error loading entities: {}", e),
            )
        })?;
        // iterate over entities and replace ids with actual objects
        for entity in wrapper.entities {
            let mut new_entity = Entity {
                id: entity.id,
                name: entity.name,
                stats: entity.stats,
                inventory: Vec::new(),
                equipment: Vec::new(),
                skills: Vec::new(),
            };
            for i in 0..entity.skills.len() {
                let skill_id = entity.skills[i];
                let skill = self.get_skill_by_id(skill_id);
                if let Some(skill) = skill {
                    new_entity.skills.push(skill);
                } else {
                    println!(
                        "Skill with id {} not found for entity {}",
                        skill_id, new_entity.name
                    );
                }
            }
            for i in 0..entity.inventory.len() {
                let item_id = entity.inventory[i];
                let item = self.get_item_by_id(item_id);
                if let Some(item) = item {
                    new_entity.inventory.push(item);
                } else {
                    println!(
                        "Item with id {} not found for entity {}",
                        item_id, new_entity.name
                    );
                }
            }
            for i in 0..entity.equipment.len() {
                let item_id = entity.equipment[i];
                let item = self.get_item_by_id(item_id);
                if let Some(item) = item {
                    let c_item = item.clone();
                    new_entity.inventory.push(item);
                    new_entity.equip_item(c_item);
                } else {
                    println!(
                        "Item with id {} not found for entity {}",
                        item_id, new_entity.name
                    );
                }
            }
            self.entities.push(new_entity);
        }

        // OLD WAY
        // let wrapper: EntitiesWrapper = serde_json::from_str(&data).map_err(|e| {
        //     io::Error::new(
        //         ErrorKind::InvalidData,
        //         format!("Deserialization error: {}", e),
        //     )
        // })?;
        // self.entities = wrapper.entities;

        for entity in self.entities.clone() {
            if entity.id < 1000 && entity.id >= 100 {
                self.pc_ents.push(entity);
            } else if entity.id >= 1000 {
                self.npc_ents.push(entity);
            }
        }

        Ok(())
    }

    pub fn load_items(&mut self, path: &str) -> io::Result<()> {
        let data = fs::read_to_string(path)?;
        let wrapper: ItemsWrapper = serde_json::from_str(&data).map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Deserialization error loading items: {}", e),
            )
        })?;
        self.items = wrapper.items;
        Ok(())
    }

    pub fn load_skills(&mut self, path: &str) -> io::Result<()> {
        let data = fs::read_to_string(path)?;
        let wrapper: SkillsWrapper = serde_json::from_str(&data).map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Deserialization error loading skills: {}", e),
            )
        })?;
        self.skills = wrapper.skills;
        Ok(())
    }

    pub fn reload(
        &mut self,
        entities_path: &str,
        items_path: &str,
        skills_path: &str,
    ) -> io::Result<()> {
        println!("Reloading game data...");
        self.load_skills(skills_path)?;
        self.load_items(items_path)?;
        self.load_entities(entities_path)?;

        self.populate_enemies();
        Ok(())
    }

    fn _get_entity_by_id(&self, id: u32) -> Option<Entity> {
        for entity in self.entities.clone() {
            if entity.id == id {
                return Some(entity);
            }
        }
        None
    }

    pub fn create_player(&mut self, entity: Entity) {
        self.players.push(entity);
    }

    pub fn create_enemy(&mut self, entity: Entity) {
        self.enemies.push(entity);
    }

    pub fn spawn_enemy(&mut self) {
        // let enemy = self.enemies[self.enemy_index].clone();
        // let mut new_enemy = enemy.clone();
        // self.enemies.push(new_enemy);
        let spawn_limit = SPAWN_LIMIT;
        let item_drop_rate = ITEM_DROP_RATE;

        if self.enemies.len() < spawn_limit {
            let rand_index = rand::random::<usize>() % self.npc_ents.len();
            let enemy = self.npc_ents[rand_index].clone();
            let mut new_enemy = enemy.clone();
            let gets_item = rand::random::<f32>() < item_drop_rate;
            if gets_item {
                let item = self.items[rand::random::<usize>() % self.items.len()].clone();
                if item.is_consumable() {
                    let c_item = item.clone();
                    new_enemy.inventory.push(item);
                    new_enemy.equip_item(c_item);
                } else {
                    new_enemy.inventory.push(item);
                }
            }
            self.create_enemy(new_enemy);
        }
    }

    pub fn populate_enemies(&mut self) {
        while self.enemies.len() < SPAWN_LIMIT {
            self.spawn_enemy();
        }
    }

    pub fn remove_enemy(&mut self, index: usize) {
        self.enemies.remove(index);
    }

    /// Set Player Index
    pub fn set_player(&mut self, index: usize) {
        self.player_index = index;
    }

    /// Get Mutable Player
    pub fn _get_player(&mut self) -> &mut Entity {
        &mut self.players[self.player_index]
    }

    /// Is Player Alive
    pub fn is_player_alive(&self) -> bool {
        self.players[self.player_index].stats.hp > 0
    }

    /// Set Enemy Index
    pub fn set_enemy(&mut self, index: usize) {
        self.enemy_index = index;
    }

    /// Get Mutable Enemy
    pub fn _get_enemy(&mut self) -> &mut Entity {
        &mut self.enemies[self.enemy_index]
    }

    /// Is Enemy Alive
    pub fn is_enemy_alive(&self) -> bool {
        self.enemies[self.enemy_index].stats.hp > 0
    }

    pub fn _is_item(&self, id: u32) -> bool {
        for item in self.items.clone() {
            if item.id == id {
                return true;
            }
        }
        false
    }

    pub fn _is_skill(&self, id: u32) -> bool {
        for skill in self.skills.clone() {
            if skill.id == id {
                return true;
            }
        }
        false
    }

    pub fn get_skill_by_id(&self, id: u32) -> Option<Skill> {
        for skill in self.skills.clone() {
            if skill.id == id {
                return Some(skill);
            }
        }
        None
    }

    pub fn get_item_by_id(&self, id: u32) -> Option<Item> {
        for item in self.items.clone() {
            if item.id == id {
                return Some(item);
            }
        }
        None
    }

    pub fn get_default_player(&self) -> Entity {
        self.entities[0].clone()
    }

    pub fn _get_skills_string(&self) -> String {
        let mut skills = String::new();
        let mut i = 1;
        for skill in self.skills.clone() {
            skills.push_str(&format!("{}. {}\n", i, skill._get_skill_string()));
            i += 1;
        }
        skills
    }

    pub fn get_players_string(&self) -> String {
        let mut players = String::new();
        let mut i = 1;
        for player in self.players.clone() {
            players.push_str(&format!("{}. {}\n", i, player.get_entity_string()));
            i += 1;
        }
        players
    }
}
