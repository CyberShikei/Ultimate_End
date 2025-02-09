// src/game/combat.rs
use crate::game::entity::Entity;

pub fn combat_round(attacker: &mut Entity, defender: &mut Entity) {
    // Example combat resolution:
    let damage = (attacker.stats.attack - defender.stats.defense).max(0);
    defender.stats.hp -= damage;
    println!(
        "{} attacks {} for {} damage!",
        attacker.name, defender.name, damage
    );
}

// You can expand this module with more complex mechanics like turn order, critical hits, etc.
