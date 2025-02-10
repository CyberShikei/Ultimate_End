// src/game/combat.rs
use crate::game::{entity::Entity, skills::Skill};

pub fn combat_round(attacker: &mut Entity, defender: &mut Entity) {
    // Example combat resolution:
    let skill = attacker.get_skill(0);
    let damage = attacker.damage_roll(&skill);
    defender.stats.hp -= damage as i32;
    println!(
        "{} attacks {} for {} damage!",
        attacker.name, defender.name, damage
    );
}

// You can expand this module with more complex mechanics like turn order, critical hits, etc.
pub fn attack_entity(attacker: &mut Entity, defender: &mut Entity, skill: &Skill) {
    let damage = attacker.damage_roll(skill);
    defender.stats.hp -= damage as i32;
    println!(
        "{} attacks {} for {} damage!",
        attacker.name, defender.name, damage
    );
}
