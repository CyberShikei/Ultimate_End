// src/game/persistence.rs
use crate::game::entity::Entity;
use crate::game::item::Item;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub items: Vec<Item>,

    pub players: Vec<Entity>,
    pub enemies: Vec<Entity>,

    pub player_index: usize,
    pub enemy_index: usize,
    // Add additional fields if needed (e.g., current level, settings, etc.)
}

#[derive(Serialize, Deserialize)]
struct EntitiesWrapper {
    entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize)]
struct ItemsWrapper {
    items: Vec<Item>,
}

impl GameState {
    /// Create a new, empty game state.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            items: Vec::new(),
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
        let wrapper: EntitiesWrapper = serde_json::from_str(&data).map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Deserialization error: {}", e),
            )
        })?;
        self.entities = wrapper.entities;
        for entity in self.entities.clone() {
            if entity.id < 100 {
                self.players.push(entity);
            } else {
                self.enemies.push(entity);
            }
        }

        Ok(())
    }

    pub fn load_items(&mut self, path: &str) -> io::Result<()> {
        let data = fs::read_to_string(path)?;
        let wrapper: ItemsWrapper = serde_json::from_str(&data).map_err(|e| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Deserialization error: {}", e),
            )
        })?;
        self.items = wrapper.items;
        Ok(())
    }

    /// Set Player Index
    pub fn set_player(&mut self, index: usize) {
        self.player_index = index;
    }

    /// Get Mutable Player
    pub fn get_player(&mut self) -> &mut Entity {
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
    pub fn get_enemy(&mut self) -> &mut Entity {
        &mut self.enemies[self.enemy_index]
    }

    /// Is Enemy Alive
    pub fn is_enemy_alive(&self) -> bool {
        self.enemies[self.enemy_index].stats.hp > 0
    }
}
