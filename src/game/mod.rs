pub mod combat;
pub mod entity;
pub mod item;
pub mod persistence;
pub mod skills;
pub mod stats;

/// Processes a command string by updating the game state accordingly.

pub fn process_command(
    state: &mut persistence::GameState,
    command: &str,
    args: Vec<String>,
) -> Result<(), String> {
    match command.to_lowercase().as_str() {
        "attack" | "a" => {
            if state.entities.len() < 2 {
                return Err("Not enough entities to engage in combat.".into());
            }

            let player = &mut state.players[state.player_index];
            let enemy = &mut state.enemies[state.enemy_index];
            let mut skill_id = 1;

            if args.len() < 1 {
                // Print player skills and ask for input
                println!("Player Skills: {:?}", player.get_skills_string());
                println!("Enter skill id to use:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                skill_id = input.trim().parse::<usize>().unwrap();
            // else if args[0] is a number
            } else {
                skill_id = args[0].parse::<usize>().unwrap();
            }

            let skill = &mut player.get_skill(skill_id - 1).clone();

            // execute a combat round
            combat::attack_entity(player, enemy, skill);
            combat::combat_round(enemy, player);

            if !state.is_enemy_alive() {
                println!("Enemy defeated!");
                // remove enemy from the state
                state.remove_enemy(state.enemy_index);
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
            if !state.is_player_alive() {
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

            println!("[PLAYER]\n{}", player.get_entity_string());
            println!("[ENEMY]\n{}", enemy.get_entity_string());
            Ok(())
        }
        "show enemies" => {
            for enemy in &state.enemies {
                println!("{:?}", enemy);
            }
            Ok(())
        }
        "show inventory" => {
            let player = &state.players[state.player_index];
            for item in &player.inventory {
                println!("{:?}", item);
            }
            Ok(())
        }
        "equip" => {
            let player = &mut state.players[state.player_index];
            println!("Inventory: {}", player.get_inventory_string());
            println!("Enter item id to equip:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let item_id = input.trim().parse::<usize>().unwrap();

            let item = player.get_item(item_id).clone();
            player.equip_item(item);
            Ok(())
        }
        "unequip" => {
            let player = &mut state.players[state.player_index];
            println!("Equipment: {:?}", player.get_equipment_string());
            println!("Enter item id to unequip:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let item_id = input.trim().parse::<usize>().unwrap();

            let item = player.get_equipment(item_id).clone();
            player.unequip_item(item);
            Ok(())
        }
        "pickup_sword" => {
            let player = &mut state.players[state.player_index];
            let sword = state.items[0].clone();
            player.add_item_to_inventory(sword);
            // player.equip_item(player.inventory[0]);
            Ok(())
        }
        "help" => {
            println!(
                "Available commands: attack, run, (un)equip, status, show enemies, show inventory, help, exit"
            );
            Ok(())
        }
        _ => Err("Unknown command. Type 'help' for a list of commands.".into()),
    }
}
