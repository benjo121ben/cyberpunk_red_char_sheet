use std::cmp::{Ordering, min, max};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use super::{journal::Journal, gear::*};

pub enum GearType {
    Drugs,
    Gear,
    Hardware,
    Programs,
    Fashion
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name: String,
    
    pub alias: String,

    pub current_armor_body: Option<usize>,

    pub current_armor_head: Option<usize>,

    #[serde(default)]
    pub armors: Vec<Armor>,

    #[serde(default)]
    pub weapons: Vec<Weapon>,

    #[serde(default)]
    pub ammo: IndexMap<String, i32>,

    #[serde(default)]
    pub cyberware: Vec<Cyberware>,

    pub humanity:i32,

    pub hp_current: i32, 

    pub ip: i32,

    pub money: i32,
    
    pub stats: CharStats,
    
    #[serde(default)]
    pub journals: Vec<Journal>,
    
    #[serde(default)]
    pub skills: IndexMap<String, Skill>,
    
    #[serde(default)]
    pub gear: IndexMap<String, i32>,

    #[serde(default)]
    pub cyberdeck_hardware: IndexMap<String, i32>,

    #[serde(default)]
    pub drugs: IndexMap<String, i32>,

    #[serde(default)]
    pub fashion: IndexMap<String, i32>,

    #[serde(default)]
    pub programs: IndexMap<String, i32>,
    
    #[serde(default)]
    pub flags: IndexMap<String, bool>,

    #[serde(default)]
    pub head_crit_injuries: Vec<usize>,

    #[serde(default)]
    pub body_crit_injuries: Vec<usize>,
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
    pub luck_current: i32,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Penalty {
    pub selectors: Vec<String>,
    pub penalty: i32,
    pub show_higlight_color: bool
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
        let class_journal = Journal {
            name: "class".to_string(),
            text: "".to_string()
        };
        let mut char = Character {
            name: String::from("Test"),
            alias: String::from("Alias"),
            current_armor_head: None,
            current_armor_body: None,
            armors: vec![],
            ammo: IndexMap::new(),
            weapons: vec![],
            cyberware: vec![],
            humanity: 0,
            hp_current: 0,
            ip: 0,
            money: 0,
            stats: CharStats { intelligence: 0, reflex: 0, dexterity: 0, technique: 0, cool: 0, willpower: 0, luck: 0, luck_current: 0, movement: 0, body: 0, empathy: 0 },
            journals: vec![class_journal, Journal::default()],
            skills: IndexMap::new(),
            gear: IndexMap::new(),
            drugs: IndexMap::new(),
            fashion: IndexMap::new(),
            cyberdeck_hardware: IndexMap::new(),
            programs: IndexMap::new(),
            flags: IndexMap::new(),
            head_crit_injuries: vec![],
            body_crit_injuries: vec![],
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
            (true,  "Martial Arts (Akido)"),
            (true,  "Martial Arts (Karate)"),
            (true,  "Martial Arts (Judo)"),
            (true,  "Martial Arts (Taekwondo)"),
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

        let add_skills_from_list = move |char: &mut Character, skill_list: Vec<(bool, &str)>, stat: &str| {
            for (diff, key) in skill_list {
                let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
                char.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 0, difficult_train: diff, stat: stat.to_string() });
            }
        };

        add_skills_from_list(&mut char, cool_skills, "cool");
        add_skills_from_list(&mut char, dex_skills, "dex");
        add_skills_from_list(&mut char, emp_skills, "emp");
        add_skills_from_list(&mut char, int_skills, "int");
        add_skills_from_list(&mut char, ref_skills, "ref");
        add_skills_from_list(&mut char, tech_skills, "tech");
        add_skills_from_list(&mut char, will_skills, "will");

        return char;
    }

    pub fn get_stat(self: &Self, stat_name: &str) -> (i32, bool){
        let stat_name_lower = stat_name.to_lowercase();

        let stat_nr = match stat_name_lower.as_str() {
            "int" => self.stats.intelligence,
            "ref" => self.stats.reflex,
            "dex" => self.stats.dexterity,
            "tech" => self.stats.technique,
            "cool" => self.stats.cool,
            "will" => self.stats.willpower,
            "luck" => self.stats.luck,
            "move" => self.stats.movement,
            "body" => self.stats.body,
            "emp" => self.humanity / 10,
            _ => {panic!("This stat does not exist {stat_name_lower}");}
        };

        let armor_penalty = match stat_name_lower.as_str() {
            "ref" | "dex" | "move" => self.get_current_armor_penalty(),
            _ => 0
        };

        let wounded_penalty = match stat_name_lower.as_str() {
            "luck"  => 0,
            "move" => { 
                if self.hp_current <= 0 { 
                    if stat_nr <= 6 { stat_nr - 1 } else {6} 
                }
                else {0}
            }
            _ => self.get_wounded_action_penalty()
        };

        let final_penatly = wounded_penalty + armor_penalty;
        return (stat_nr - final_penatly, final_penatly != 0);
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
        return 10 + 5 * ((self.stats.body as f32 + self.stats.willpower as f32) / 2.0).ceil() as i32
    }

    pub fn get_current_body_armor(&self) -> Option<&Armor> {
        if self.current_armor_body.is_none(){
            return None;
        }
        
        self.armors.get(self.current_armor_body.unwrap())
    }

    pub fn get_current_body_armor_mut(&mut self) -> Option<&mut Armor> {
        if self.current_armor_body.is_none(){
            return None;
        }
        
        self.armors.get_mut(self.current_armor_body.unwrap())
    }

    pub fn get_current_head_armor(&self) -> Option<&Armor> {
        if self.current_armor_head.is_none(){
            return None;
        }
        self.armors.get(self.current_armor_head.unwrap())
    }

    pub fn get_current_head_armor_mut(&mut self) -> Option<&mut Armor> {
        if self.current_armor_head.is_none(){
            return None;
        }
        self.armors.get_mut(self.current_armor_head.unwrap())
    }

    pub fn get_current_armor_penalty(&self) -> i32{
        let head_armor_penalty = self.get_current_head_armor()
            .map(|armor|armor.penalty)
            .or(Some(0))
            .unwrap();
        let body_armor_penalty = self.get_current_body_armor()
            .map(|armor|armor.penalty)
            .or(Some(0))
            .unwrap();
        std::cmp::max(head_armor_penalty, body_armor_penalty)
    }

    pub fn add_gear(&mut self, gear_type: GearType,name: String) {
        let changed_name = name.to_lowercase().replace(" ", "_");
        let relevant_map = match gear_type {
            GearType::Drugs => &mut self.drugs,
            GearType::Gear => &mut self.gear,
            GearType::Hardware => &mut self.cyberdeck_hardware,
            GearType::Programs => &mut self.programs,
            GearType::Fashion => &mut self.fashion,
        };
        
        if relevant_map.get_mut(&changed_name).and_then(|val| Some(*val += 1)).is_none() {
            relevant_map.insert(changed_name, 1);
        }
    }

    pub fn change_health_without_armor(&mut self, amount: i32) {
        self.hp_current = min(max(self.hp_current + amount, 0), self.calc_max_health());
    }

    pub fn change_health_with_armor(&mut self, head_damage:bool, melee_damage: bool, amount: i32) {
        if amount >= 0 {
            self.hp_current = min(self.hp_current + amount, self.calc_max_health());
            return;
        }

        let armor = if head_damage {
            self.get_current_head_armor()
        } else {
            self.get_current_body_armor()
        };

        let current_sp = armor
            .and_then(|a| Some(
                if melee_damage {
                    (a.sp_current as f32 / 2.0).ceil() as i32
                }
                else {
                    a.sp_current
                }
            ))
            .or(Some(0))
            .unwrap()
        ;

        let mut overshoot_damage = amount + current_sp;

        if overshoot_damage >= 0 {
            return;
        }

        if head_damage {
            overshoot_damage *= 2;
        }

        self.hp_current = max(0, self.hp_current + overshoot_damage);

        let mut_armor = if head_damage {self.get_current_head_armor_mut()} else {self.get_current_body_armor_mut()};
        mut_armor.and_then(|a| {
            a.sp_current -= 1;
            Some(a)
        });
    }

    pub fn get_wounded_action_penalty(&self) -> i32 {
        let half_hp = (self.calc_max_health() as f32 / 2.0).ceil() as i32;
        if self.hp_current < 1 {
            return 4;
        }
        else if self.hp_current < half_hp {
            return 2;
        }
        else {
            return 0;
        }
    } 

    pub fn got_cyber(&self, name: &str) -> bool {
        self.cyberware.iter().find(|cyber| get_map_key(*cyber).as_str() == get_map_key_from_name(name)).is_some()
    }

    pub fn can_use_smar_weapons(&self) -> bool{
        let got_neural_link = self.got_cyber("Neural Link");
        let got_grip = self.got_cyber("Subdermal Grip");
        let got_plugs = self.got_cyber("Interface Plugs");

        got_neural_link && (got_grip || got_plugs)
    }

    pub fn get_max_humanity(&self) -> i32 {
        let abs_max: i32 = self.stats.empathy * 10;
        let penalty: i32 = self.cyberware.iter()
            .filter(|cyber| cyber.psychosis > 0)
            .map(|cyber| {
                if cyber.get_file().as_str() == "borgware" { 4 } else { 2 }
            }).sum();
        abs_max - penalty
    }
}