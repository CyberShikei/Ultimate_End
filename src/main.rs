mod cli;
mod game;

use crate::game::{db::connect_to_db, persistence::GameState};
use std::io::{self, Write};

fn main() {
    // Connect to the database
    let db_pool = match connect_to_db() {
        Ok(pool) => {
            println!("Connected to the database.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return;
        }
    };

    // Parse CLI arguments
    let matches = cli::build_cli().get_matches();
    let debug_mode = matches.is_present("debug");
    if debug_mode {
        println!("Debug mode enabled.");
    }

    let save_file = "savegame.json";
    let enities_file = "assets/entities.json";
    let items_file = "assets/items.json";
    let skills_file = "assets/skills.json";

    // Attempt to load an existing game state. If not found, create a new state.
    let mut game_state = match GameState::load_from_file(save_file) {
        Ok(state) => {
            println!("Loaded game state from '{}'.", save_file);
            state
        }
        Err(_) => {
            println!("No saved game found. Starting a new game.");
            // Create a new game reloading all enities.
            let mut state = GameState::new();
            if let Err(e) = state.load_entities(enities_file) {
                eprintln!("Failed to load entities: {}", e);
            }
            if let Err(e) = state.load_items(items_file) {
                eprintln!("Failed to load items: {}", e);
            }
            if let Err(e) = state.load_skills(skills_file) {
                eprintln!("Failed to load skills: {}", e);
            }
            state
        }
    };

    // Main game loop
    loop {
        // Display prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read input: {}", err);
            continue;
        }
        let command = input.trim();

        // Check for exit conditions
        if command.eq_ignore_ascii_case("exit") || command.eq_ignore_ascii_case("quit") {
            println!("Exiting game. Goodbye!");
            break;
        }

        // Process the command via game logic
        if let Err(e) = game::process_command(&mut game_state, command) {
            println!("Error: {}", e);
        }

        // Optionally, save the game state after processing the command
        if let Err(e) = game_state.save_to_file(save_file) {
            eprintln!("Failed to save game state: {}", e);
        }
    }
}
