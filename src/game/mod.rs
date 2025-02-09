pub mod combat;
pub mod entity;
pub mod item;
pub mod persistence;
pub mod stats;

/// Processes a command string by updating the game state accordingly.

pub fn process_command(state: &mut persistence::GameState, command: &str) -> Result<(), String> {
    match command.to_lowercase().as_str() {
        "attack" => {
            if state.entities.len() < 2 {
                return Err("Not enough entities to engage in combat.".into());
            }

            let player = &mut state.players[state.player_index];
            let enemy = &mut state.enemies[state.enemy_index];

            // Execute a combat round.
            combat::combat_round(player, enemy);
            combat::combat_round(enemy, player);

            if state.enemies[state.enemy_index].stats.hp <= 0 {
                println!("Enemy defeated!");
            }

            if state.is_player_alive() == false {
                println!("You died!");
                return Ok(());
            }

            Ok(())
        }
        "run" => {
            if state.entities.len() < 2 {
                return Err("Not enough entities to run away.".into());
            }
            combat::combat_round(
                &mut state.enemies[state.enemy_index],
                &mut state.players[state.player_index],
            );
            if state.players[state.player_index].stats.hp <= 0 {
                println!("You died!");
                return Ok(());
            }

            println!("You ran away!");

            println!("A New Enemy Approaches!");

            let new_enemy_index = rand::random::<usize>() % state.enemies.len();
            state.set_enemy(new_enemy_index);

            println!("New Enemy: {:?}", state.enemies[state.enemy_index].name);

            Ok(())
        }
        "status" => {
            if state.entities.len() < 2 {
                return Err("Not enough entities to display status.".into());
            }
            let player = &state.players[state.player_index];
            let enemy = &state.enemies[state.enemy_index];

            println!("{}: {:?}", player.name, player.stats.get_stats_string());
            println!("{}: {:?}", enemy.name, enemy.stats.get_stats_string());
            Ok(())
        }
        "show enemies" => {
            for enemy in &state.enemies {
                println!("{:?}", enemy);
            }
            Ok(())
        }
        "use_sword" => {
            let player = &mut state.players[state.player_index];
            let sword = state.items[0].clone();
            player.apply_item(&sword);
            Ok(())
        }
        "help" => {
            println!("Available commands: attack, run, status, show enemies, help, exit");
            Ok(())
        }
        _ => Err("Unknown command. Type 'help' for a list of commands.".into()),
    }
}
