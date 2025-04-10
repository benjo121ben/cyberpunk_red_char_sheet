use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearData {
    pub ammunition: Vec<Ammo>,
    pub armor: Vec<Armor>,
    #[serde(rename = "cyberdeck-hardware")]
    pub cyberdeck_hardware: Vec<CyberdeckHardware>,
    pub cyberware: Vec<Cyberware>,
    pub drugs: Vec<Drug>,
    pub items: Vec<Item>,
    #[serde(rename = "programs-attackers")]
    pub programs_attackers: Vec<ProgramsAttacker>,
    #[serde(rename = "programs-boosters")]
    pub programs_boosters: Vec<ProgramsBooster>,
    #[serde(rename = "programs-defender")]
    pub programs_defender: Vec<ProgramsDefender>,
    pub weapons: Vec<Weapon>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Armor {
    pub armor_data: ArmorData,
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
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
pub struct CyberdeckHardware {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cyberware {
    pub description: String,
    pub file: String,
    pub internal: bool,
    pub name: String,
    pub price: i32,
    pub psychosis: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drug {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramsAttacker {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramsBooster {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramsDefender {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    #[serde(default)]
    pub personalized_name: String,
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub weapon_data: WeaponData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponData {
    pub ammo: Ammo,
    pub burst: bool,
    pub damage: String,
    pub fullauto: bool,
    pub rof: i32,
    pub skill: String,
    pub weapontype: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ammo {
    pub max: Option<i32>,
    pub min: i32,
    pub value: i32,
}
