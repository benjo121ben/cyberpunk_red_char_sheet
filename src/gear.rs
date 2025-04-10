use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearData {
    pub armor: Vec<Armor>,
    #[serde(rename = "cyberdeck-hardware")]
    pub cyberdeck_hardware: Vec<CyberdeckHardware>,
    pub cyberware: Vec<Value>,
    pub drugs: Vec<Drug>,
    pub items: Vec<Item>,
    #[serde(rename = "programs-attackers")]
    pub programs_attackers: Vec<Attacker>,
    #[serde(rename = "programs-boosters")]
    pub programs_boosters: Vec<Booster>,
    #[serde(rename = "programs-defender")]
    pub programs_defender: Vec<ProgramsDefender>,
    pub weapons: Vec<Weapon>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Armor {
    pub data: Data,
    pub armor_data: ArmorData,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorData {
    pub sp: i32,
    #[serde(default)]
    pub sp_current: i32,
    pub penalty: i32
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub description: String,
    pub price: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CyberdeckHardware {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drug {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attacker {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Booster {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramsDefender {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    pub data: WeaponData,
    pub file: String,
    pub name: String,
    #[serde(default)]
    pub personalized_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponData {
    pub ammo: Ammo,
    pub burst: bool,
    pub damage: String,
    pub description: String,
    pub fullauto: bool,
    pub legality: i32,
    pub price: i32,
    pub rarity: i32,
    pub rof: i32,
    pub skill: String,
    pub weapontype: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cyberware {
    pub data: Data,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ammo {
    pub max: Option<i32>,
    pub min: i32,
    pub value: i32,
}
