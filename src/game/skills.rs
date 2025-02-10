use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillType {
    Passive,
    Active,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillTarget {
    SelfTarget,
    SingleTarget,
    MultiTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillClass {
    Physical,
    Magical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
    pub skill_target: SkillTarget,
    pub skill_class: SkillClass,
    pub power: u32,
    pub cost: u32,
}

impl Skill {
    pub fn new(
        id: u32,
        name: &str,
        description: &str,
        skill_type: SkillType,
        skill_target: SkillTarget,
        skill_class: SkillClass,
        power: u32,
        cost: u32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            skill_type,
            skill_target,
            skill_class,
            power,
            cost,
        }
    }

    pub fn get_skill_string(&self) -> String {
        format!(
            "Skill: {}\nDescription: {}\nType: {:?}\nTarget: {:?}\nClass: {:?}\nPower: {}\nCost: {}\n",
            self.name, self.description, self.skill_type, self.skill_target, self.skill_class, self.power, self.cost
        )
    }
}
