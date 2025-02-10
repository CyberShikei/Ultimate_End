mod cli;
mod game;

use crate::game::persistence::GameState;
use std::io::{self, Write};

// CONSTANTS
const SAVE_FILE: &str = "savegame.json";
const ENTITIES_FILE: &str = "assets/entities.json";
const ITEMS_FILE: &str = "assets/items.json";
const SKILLS_FILE: &str = "assets/skills.json";

fn help() {
    println!("Commands:");
    println!("    1. start: Start game.");
    println!("    2. cc: Create a new character.");
    println!("    3. lc: Load an existing character.");
    println!("    4. sc: Show all characters.");
    println!("    5. help: Display this help message.");
    println!("    6. exit: Exit the game.");
}

fn split_command_args(input: &str) -> (String, Vec<String>) {
    let split_input = input.split_whitespace();
    let command = split_input.clone().next().unwrap_or("").to_lowercase();
    let args = split_input.skip(1).map(|s| s.to_string()).collect();
    (command, args)
}

fn start_game(game_state: &mut GameState, save_file: &str) {
    // Main game loop
    loop {
        // Display prompt
        let line_leader = ">> ";
        let input = ask_user_for_input(&format!("{} ", line_leader));
        let (command, args) = split_command_args(&input);

        // Check for exit conditions
        if command.eq_ignore_ascii_case("exit") || command.eq_ignore_ascii_case("quit") {
            println!("Exiting game. Goodbye!");
            break;
        }

        // Process the command via game logic
        if let Err(e) = game::process_command(game_state, command.as_str(), args) {
            println!("Error: {}", e);
        }

        // Optionally, save the game state after processing the command
        if let Err(e) = game_state.save_to_file(save_file) {
            eprintln!("Failed to save game state: {}", e);
        }

        // Check for game over conditions
        if !game_state.is_player_alive() {
            println!("You died!");
            break;
        }
    }
}

fn load_game_state(save_file: &str) -> GameState {
    match GameState::load_from_file(save_file) {
        Ok(state) => {
            println!("Loaded game state from file.");
            state
        }
        Err(e) => {
            eprintln!("Failed to load game state: {}", e);
            let state = create_new_game_state();
            state
        }
    }
}

fn create_new_game_state() -> GameState {
    let mut state = GameState::new();
    if let Err(e) = state.reload(ENTITIES_FILE, ITEMS_FILE, SKILLS_FILE) {
        eprintln!("Failed to load game data: {}", e);
    } else {
        println!("Loaded game data.");
    }
    state
}

fn ask_user_for_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn ask_user_create_player(game_state: &mut GameState) {
    // Ask user for player name
    let name = ask_user_for_input("Enter character name: ");
    let id = game_state.players.len() as u32 + 1;
    let mut player = game::entity::Entity::new(id, &name);
    let default_player = game_state.get_default_player();

    player.stats = default_player.stats.clone();
    player.skills = default_player.skills.clone();
    player.inventory = default_player.inventory.clone();
    player.equipment = default_player.equipment.clone();

    game_state.create_player(player);
    game_state.set_player(game_state.players.len() - 1);
}

fn ask_user_select_player(game_state: &mut GameState) {
    let players = game_state.get_players_string();
    println!("Select a character to load: ");
    println!("{}", players);

    let player_id = ask_user_for_input("Enter character ID: ");
    // index - 1
    let index = player_id.parse::<usize>().unwrap() - 1;
    game_state.set_player(index);
}

fn welcome_screen() {
    println!("");
    println!("+++++++++++++++++++++");
    println!("Welcome to Ultima End");
    println!("+++++++++++++++++++++");
    println!("");

    let mut exiting_game = false;

    let save_file = SAVE_FILE;
    let mut game_state = load_game_state(save_file);
    let mut is_character_loaded = false;

    while !exiting_game {
        let line_leader = "> ";
        let input = ask_user_for_input(&format!("{} ", line_leader));
        let (command, args) = split_command_args(&input);

        match command.as_str() {
            "1" | "start" => {
                if is_character_loaded {
                    println!("Starting a new game.");
                    //let &mut run_game_state = &mut game_state;
                    start_game(&mut game_state, save_file);
                } else {
                    println!("No character loaded. Please load a character or create a new one.");
                }
            }
            "2" | "cc" => {
                println!("Creating a new character.");
                ask_user_create_player(&mut game_state);
                is_character_loaded = true;
            }
            "3" | "lc" => {
                println!("Loading an existing character.");
                ask_user_select_player(&mut game_state);
                is_character_loaded = true;
            }
            "4" | "sc" => {
                println!("Showing all characters.");
                println!("{}", game_state.get_players_string());
            }
            "5" | "help" => {
                help();
            }
            "6" | "exit" => {
                println!("Thanks for playing Ultima End.");
                exiting_game = true;
            }
            _ => {
                println!("Invalid command. Type 'help' for a list of commands.");
            }
        }
    }
}

fn main() {
    // // Connect to the database
    // let db_pool = match connect_to_db() {
    //     Ok(pool) => {
    //         println!("Connected to the database.");
    //         pool
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to connect to the database: {}", e);
    //         return;
    //     }
    // };

    // Parse CLI arguments
    let matches = cli::build_cli().get_matches();
    let debug_mode = matches.is_present("debug");
    if debug_mode {
        println!("Debug mode enabled.");
    }

    welcome_screen();
}
