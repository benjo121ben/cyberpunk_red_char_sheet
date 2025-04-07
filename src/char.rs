
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use super::journal::Journal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name: String,

    pub hp_current: i32, 
    
    pub stats: CharStats,
    
    #[serde(default)]
    pub journals: Vec<Journal>,
    
    #[serde(default)]
    pub skills: Vec<Skill>,
    
    #[serde(default)]
    pub gear_list: Vec<Gear>,
    
    #[serde(default)]
    pub flags: HashMap<String, bool>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharStats {
    pub intelligence: i32,
    pub reflex: i32,
    pub dexterity: i32,
    pub technique: i32,
    pub cool: i32,
    pub willpower: i32,
    pub luck: i32,
    pub movement: i32,
    pub empathy: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub nr: i32,
    pub difficult_train: bool,
    pub stat: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Gear {
    pub name: String,
}

impl Character {
    pub fn zero() -> Character {
        let mut char = Character {
            name: String::from("Test"),
            hp_current: 0,
            stats: CharStats { intelligence: 0, reflex: 0, dexterity: 0, technique: 0, cool: 0, willpower: 0, luck: 0, movement: 0, empathy: 0 },
            journals: vec![Journal::default()],
            skills: vec![],
            gear_list: vec![],
            flags: HashMap::new(),
        };

        let cool_skills: Vec<(bool, &str)> = vec![
            (false, "Acting"),
            (false, "Bribery"),
            (false, "Interrogation"),
            (false, "Personal Grooming"),
            (false, "Persuasion"),
            (false, "Streetwise"),
            (false, "Trading"),
            (false, "Wardrobe & Style"),
        ];
        let dex_skills = vec![
            (false, "Athletics"),
            (false, "Brawling"),
            (false, "Contortionist"),
            (false, "Dance"),
            (false, "Evasion"),
            (true,  "Martial Arts"),
            (false, "Melee Weapon"),
            (false, "Stealth"),
        ];
        let emp_skills = vec![
            (false, "Conversation"),
            (false, "Human Perception")
        ];
        let int_skills = vec![
            (false, "Accounting"),
            (false, "Animal Handling"),
            (false, "Bureaucracy"),
            (false, "Business"),
            (false, "Composition"),
            (false, "Conceal & Reveal Object"),
            (false, "Criminology"),
            (false, "Cryptography"),
            (false, "Deduction"),
            (false, "Education"),
            (false, "Gamble"),
            (false, "Library Search"),
            (false, "Lip Reading"),
            (false, "Local Expert (Your Home)"),
            (false, "Perception"),
            (false, "Science"),
            (false, "Tactics"),
            (false, "Wilderness Survival"),
        ];

        let ref_skills = vec![
            (false, "Archery"),
            (true, "Autofire"),
            (false, "Drive Land Vehicle"),
            (false, "Handgun"),
            (true, "Heavy Weapons"),
            (true, "Pilot Air Vehicle"),
            (false, "Pilot Sea Vehicle"),
            (false, "Ride"),
            (false, "Shoulder Arms"),
        ];

        let tech_skills = vec![
            (false, "Air Vehicle Tech"),
            (false, "Basic Tech"),
            (false, "Cybertech"),
            (true, "Demoltions"),
            (false, "Electronics & Security Tech"),
            (false, "First Aid"),
            (false, "Forgery"),
            (false, "Land Vehicle Tech"),
            (false, "Paint, Draw, Sculpt"),
            (true, "Paramedic"),
            (false, "Photography/Film"),
            (false, "Pick Lock"),
            (false, "Pick Pocket"),
            (false, "Play Instrument"),
            (false, "Sea Vehicle Tech"),
            (false, "Weapons Tech"),
            
        ];

        let will_skills = vec![
            (false, "Concentration"),
            (false, "Endurance"),
            (false, "Resist Torture & Drugs"),
        ];

        for (diff, key) in cool_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "cool".to_string() });
        }

        for (diff, key) in dex_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "dex".to_string() });
        }

        for (diff, key) in emp_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "emp".to_string() });
        }

        for (diff, key) in int_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "int".to_string() });
        }

        for (diff, key) in ref_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "ref".to_string() });
        }

        for (diff, key) in tech_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "tech".to_string() });
        }

        for (diff, key) in will_skills {
            char.skills.push(Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "will".to_string() });
        }

        return char;
    }
}

/* impl Character {
    fn get_attribute_and_lore_flag_from_skill_name(skill_name: &str, p_type: &ProficiencyType) -> String{
        return String::from(match p_type {
            ProficiencyType::Save => {
                match skill_name {
                    "Fortitude" => "con",
                    "Reflex" => "dex",
                    "Will" => "wis",
                    _ => {panic!("This save does not exist {skill_name}");}
                }
            },
            ProficiencyType::Skill => {
                match skill_name {
                    "Acrobatics" => "dex",
                    "Arcana" => "int",
                    "Athletics" => "str",
                    "Crafting" => "int",
                    "Deception" => "cha",
                    "Diplomacy" => "cha",
                    "Intimidation" => "cha",
                    "Medicine" => "wis",
                    "Nature" => "wis",
                    "Occultism" => "int",
                    "Performance" => "cha",
                    "Religion" => "wis",
                    "Society" => "int",
                    "Stealth" => "dex",
                    "Survival" => "wis",
                    "Thievery" => "dex",
                    _ => {panic!("This skill does not exist {skill_name}");}
                }
            },
            ProficiencyType::Lore => "int",
            ProficiencyType::Armor => "dex",
            ProficiencyType::Weapon => "str",
            ProficiencyType::Spell => "key",
            ProficiencyType::ClassDC => "key",
            ProficiencyType::Perception => "wis",
        });
        
    }

    pub fn get_prof_obj_from_name(self: &Self, skill_name: &str) -> Option<CalculatedStat>{
        return self.proficiencies
        .iter()
        .find(|prof| prof.name==skill_name).cloned();
    }

    pub fn get_prof_indx_from_name(self: &Self, skill_name: &str) -> Option<usize>{
        for (indx, skill) in self.proficiencies.iter().enumerate() {
            if skill.name == skill_name {
                return Some(indx);
            }
        }
        return None;
    }

    pub fn calculate_ac(self: & Self, bp_map: &HashMap<String, StatBonusPenalties>) -> (i32, i32) {
        let calc_stat = self.get_prof_obj_from_name("Medium").expect("Character must have a medium proficiency");
        let dex_cap = 1;
        let item_bonus = 4;
        let auto_bonus_prog_bonus = self.abp_data.def_pot;
        let dex_bonus = std::cmp::min(self.attributes.get_stat_val("dex").expect("Defense expects a dex attribute to be set"), dex_cap);
        let prof_bonus = calc_stat.proficiency.get_bonus(self.level);
        let selectors = vec!["dex".to_string(), "ac".to_string()];
        let armor_bonus_penalties = combine_selected_bonus_penalties(&bp_map, &selectors).calculate_total();
        let armor_total = 10 + dex_bonus + prof_bonus + item_bonus + armor_bonus_penalties + auto_bonus_prog_bonus;

        return (armor_total, armor_bonus_penalties)
    }

    pub fn level_up_down(self: &mut Self, increase: i32) {
        self.level += increase;
        self.hp_info.calculate_max_hp(self.level, self.attributes.get_stat_val("con").expect("There should be a con stat"));
        self.animal.hp_info.calculate_max_hp(self.level, self.animal.attributes.get_stat_val("con").expect("There should be a con stat"));
    } 
} */