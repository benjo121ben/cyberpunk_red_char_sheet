use serde::Deserialize;
use serde::Serialize;

pub trait ShopItem {
    fn get_name(&self) -> &String;
    fn get_description(&self) -> &String;
    fn get_price(&self) -> i32;
    fn get_type(&self) -> &String;
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopItemVisualData {
    pub name: String,
    pub description: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_data: String
}

impl ShopItemVisualData {
    pub fn from(item: &impl ShopItem) -> Self {
        Self { 
            name: item.get_name().clone(),
            description: item.get_description().clone(),
            price: item.get_price(),
            type_data: item.get_type().clone(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearData {
    pub ammunition: Vec<Ammunition>,
    pub armor: Vec<Armor>,
    #[serde(rename = "cyberdeck-hardware")]
    pub cyberdeck_hardware: Vec<CyberdeckHardware>,
    pub cyberware: Vec<Cyberware>,
    pub drugs: Vec<Drug>,
    pub fashion: Vec<ShopItemVisualData>,
    pub items: Vec<Item>,
    pub programs: Vec<Program>,
    pub weapons: Vec<Weapon>
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Armor {
    pub armor_data: ArmorData,
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmorData {
    pub sp: i32,
    #[serde(default)]
    pub sp_current: i32,
    pub penalty: i32,
    #[serde(default="is_false")]
    pub head: bool
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CyberdeckHardware {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Drug {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponData {
    pub ammo: Option<WeaponAmmoData>,
    pub bonus: Option<i32>,
    pub burst: bool,
    pub damage: String,
    pub fullauto: bool,
    pub rof: i32,
    pub skill: String,
    pub weapontype: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponAmmoData {
    pub max: Option<i32>,
    pub value: i32,
    pub ammo_type: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ammunition {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

//needed to set default bools
pub fn is_false() -> bool { false }


impl ShopItem for Armor {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for Weapon {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for Ammunition {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for Cyberware {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for CyberdeckHardware {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for Drug {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for Item {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl ShopItem for Program {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_price(&self) -> i32 {
        return self.price;
    }

    fn get_type(&self) -> &String {
        return &self.type_field;
    }
}

impl WeaponAmmoData {
    pub fn shoot(&mut self) {
        self.value = std::cmp::max(0, self.value - 1);
    }

    pub fn reload(&mut self) {
        self.max.and_then(|max_val| {
            self.value = max_val;
            Some(max_val)
        });
    }
}