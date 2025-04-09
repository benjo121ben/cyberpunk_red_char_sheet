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
#[serde(rename_all = "camelCase")]
pub struct Armor {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub description: String,
    pub legality: i64,
    pub price: i64,
    pub rarity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CyberdeckHardware {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drug {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attacker {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Booster {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramsDefender {
    pub data: Data,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub data: WeaponData,
    pub effects: Vec<Value>,
    pub file: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponData {
    pub ammo: Ammo,
    pub burst: bool,
    pub damage: String,
    pub description: String,
    pub fullauto: bool,
    pub legality: i64,
    pub price: i64,
    pub rarity: i64,
    pub rof: i64,
    pub skill: String,
    pub weapontype: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ammo {
    pub max: Option<i64>,
    pub min: i64,
    pub value: i64,
}
