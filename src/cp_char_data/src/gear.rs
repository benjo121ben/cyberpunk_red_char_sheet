use serde::Deserialize;
use serde::Serialize;

pub trait Shoppable {
    fn get_name(&self) -> &String;
    fn get_description(&self) -> &String;
    fn get_price(&self) -> i32;
    fn get_type(&self) -> &String;
    fn get_file(&self) -> &String;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeType {
    Pistol,
    Shotgun,
    Assault,
    Sniper,
    SMG,
    Bow,
    Grenade,
    Rocket,
    Melee,
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmmoMagType {
    Standard,
    Extended,
    Drum
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShoppableVisualData {
    pub name: String,
    pub description: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_data: String
}

impl ShoppableVisualData {
    pub fn from(item: &impl Shoppable) -> Self {
        Self { 
            name: item.get_name().clone(),
            description: item.get_description().clone(),
            price: item.get_price(),
            type_data: item.get_type().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemQuality {
    Average,
    Excellent,
    Poor
}

impl Default for ItemQuality {
    fn default() -> Self {
        ItemQuality::Average
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearData {
    pub ammunition: Vec<Ammunition>,
    pub armor: Vec<Armor>,
    pub attachments: Vec<Attachment>,
    #[serde(rename = "cyberdeck-hardware")]
    pub cyberdeck_hardware: Vec<ItemData>,
    pub cyberware: Vec<Cyberware>,
    pub drugs: Vec<ItemData>,
    pub fashion: Vec<ShoppableVisualData>,
    pub items: Vec<ItemData>,
    pub programs: Vec<Program>,
    pub weapons: Vec<Weapon>
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub price: i32,
    pub description: String,
    pub file: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Armor {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,

    //armor data
    pub sp: i32,
    #[serde(default)]
    pub sp_current: i32,
    pub penalty: i32,
    #[serde(default="is_false")]
    pub head: bool,
    pub bonus: Option<i32>,
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


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    #[serde(default)]
    pub personalized_name: String,
    pub description: String,
    pub file: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub weapon_data: WeaponData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponData {
    pub ammo: Option<WeaponAmmoData>,
    pub burst: bool,
    pub damage: String,
    pub fullauto: bool,
    pub rof: i32,
    pub skill: String,
    #[serde(default)]
    pub attachments: Vec<String>,
    #[serde(default)]
    pub quality: ItemQuality,
    pub weapontype: RangeType
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponAmmoData {
    pub max: Vec<i32>,
    #[serde(default)]
    pub mag_type: AmmoMagType,
    pub value: i32,
    pub current_ammo_type: Option<String>,
    pub compatible_calibers: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ammunition {
    pub description: String,
    pub file: String,
    pub name: String,
    pub price: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub caliber: String,
    pub only_one: bool
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub shorthand: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub file: String,
    pub price: i64,
    pub slot_size: i32,
    pub description: String,
    pub selector: Selector,
    pub slot_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selector {
    pub only_shoulder_arms: Option<bool>,
    pub exclude_type: Option<RangeType>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub price: i32,
    pub description: String,
    pub file: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub atk: i32,
    pub def: i32,
    pub rez: i32
}

//needed to set default bools
pub fn is_false() -> bool { false }

impl Default for AmmoMagType {
    fn default() -> Self {
        Self::Standard
    }
}


impl Shoppable for Armor {
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

    fn get_file(&self) -> &String {
        return &self.file;
    }
}

impl Shoppable for Weapon {
    fn get_name(&self) -> &String {
        if self.personalized_name.to_string() != "" {
            &self.personalized_name
        } else {
            &self.name
        }
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

    fn get_file(&self) -> &String {
        return &self.file;
    }
}

impl Shoppable for Ammunition {
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

    fn get_file(&self) -> &String {
        return &self.file;
    }
}

impl Shoppable for Cyberware {
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

    fn get_file(&self) -> &String {
        return &self.file;
    }
}

impl Shoppable for ItemData {
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

    fn get_file(&self) -> &String {
        return &self.file;
    }
}

impl Shoppable for Program {
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

    fn get_file(&self) -> &String {
        return &self.file;
    }
}

impl Armor {
    pub fn get_max_sp(&self) -> i32 {
        return self.sp + self.bonus.or(Some(0)).unwrap()
    }
}

impl Weapon {
    pub fn get_free_attachment_slots(&self, gear_data: &GearData) -> i32 {
        let mut count: i32 = 3;
        for attachment in self.weapon_data.attachments.iter() {
            count -= gear_data
                .attachments
                .iter()
                .find(|gear_att| gear_att.shorthand == *attachment)
                .expect("Expecting attachment to exist")
                .slot_size
        }
        count
    }
}

impl WeaponAmmoData {
    pub fn shoot(&mut self) {
        self.value = std::cmp::max(0, self.value - 1);
    }

    pub fn get_max_ammo(&self) -> i32 {
        let max_index = match self.mag_type {
            AmmoMagType::Standard => 0,
            AmmoMagType::Extended => 1,
            AmmoMagType::Drum => 2,
        };
        *self.max.get(max_index).unwrap()
    }

    pub fn undo_shoot(&mut self) {
        self.value = std::cmp::min(self.get_max_ammo(), self.value + 1);
    }
}

pub fn get_map_key(obj: &impl Shoppable) -> String {
    obj.get_name().to_lowercase().replace(" - ", "_").replace(" ", "_")
}

pub fn get_map_key_from_name(name: &str) -> String {
    name.to_lowercase().replace(" - ", "_").replace(" ", "_")
}
