
use std::{cmp::Ordering};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use super::{journal::Journal, gear::*};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name: String,

    #[serde(default)]
    pub armors: Vec<Armor>,

    #[serde(default)]
    pub weapons: Vec<Weapon>,

    #[serde(default)]
    pub cyberware: Vec<Cyberware>,

    pub humanity:i32,

    pub hp_current: i32, 
    
    pub stats: CharStats,
    
    #[serde(default)]
    pub journals: Vec<Journal>,
    
    #[serde(default)]
    pub skills: IndexMap<String, Skill>,
    
    #[serde(default)]
    pub gear_list: IndexMap<String, i32>,
    
    #[serde(default)]
    pub flags: IndexMap<String, bool>
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
    pub body: i32,
    pub empathy: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub nr: i32,
    pub difficult_train: bool,
    pub stat: String
}

impl Skill {
    pub fn cmp_stat_and_name(&self, other: &Self) -> Ordering{
        self.stat
            .cmp(&other.stat)
            .then(self.name.cmp(&other.name))
    }

    pub fn cmp_name(&self, other: &Self) -> Ordering{
        self.name.cmp(&other.name)
    }
}

impl Character {
    pub fn zero() -> Character {
        let mut char = Character {
            name: String::from("Test"),
            armors: vec![],
            weapons: vec![],
            cyberware: vec![],
            humanity: 0,
            hp_current: 0,
            stats: CharStats { intelligence: 0, reflex: 0, dexterity: 0, technique: 0, cool: 0, willpower: 0, luck: 0, movement: 0, body: 0, empathy: 0 },
            journals: vec![Journal::default()],
            skills: IndexMap::new(),
            gear_list: IndexMap::new(),
            flags: IndexMap::new(),
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
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "cool".to_string() });
        }

        for (diff, key) in dex_skills {
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "dex".to_string() });
        }

        for (diff, key) in emp_skills {
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "emp".to_string() });
        }

        for (diff, key) in int_skills {
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "int".to_string() });
        }

        for (diff, key) in ref_skills {
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "ref".to_string() });
        }

        for (diff, key) in tech_skills {
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "tech".to_string() });
        }

        for (diff, key) in will_skills {
            let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
            char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: "will".to_string() });
        }

        return char;
    }

    pub fn get_stat(self: &Self, stat_name: &str) -> i32 {
        match stat_name.to_lowercase().as_str() {
            "int" => return self.stats.intelligence,
            "ref" => return self.stats.reflex,
            "dex" => return self.stats.dexterity,
            "tech" => return self.stats.technique,
            "cool" => return self.stats.cool,
            "will" => return self.stats.willpower,
            "luck" => return self.stats.luck,
            "move" => return self.stats.movement,
            "body" => return self.stats.body,
            "emp" => return self.humanity / 10,
            _ => {panic!("This stat does not exist {stat_name}");}
        }
    }

    pub fn has_active_flag(self: &Self, key: &str) -> bool{
        *self.flags.get(key).or(Some(&false)).unwrap()
    }

    pub fn flip_flag(self: &mut Self, key: &str) {
        let new_val = !self.flags.get(key).or(Some(&false)).unwrap();
        self.flags.insert(
            key.to_string(), 
            new_val
        );
    }

    pub fn calc_max_health(self: &Self) -> i32 {
        return 10 + 5 * ((self.get_stat("body") as f32 + self.get_stat("will") as f32) / 2.0).ceil() as i32
    }
}