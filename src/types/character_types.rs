use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Vision {
    Pyro,
    Hydro,
    Dendro,
    Electro,
    Anemo,
    Cryo,
    Geo
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Weapon {
    Sword,
    Claymore,
    Bow,
    Catalyst,
    Polearm
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Nation {
    Mondstadt,
    Liyue,
    Inazuma,
    Sumeru,
    Fontaine,
    Natlan,
    Snezhnaya
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stat {
    lvl: String,
    hp: f32,
    atk: f32,
    def: f32,
    crit_rate: f32,
    crit_dmg: f32
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    lvl: String,
    hp: String,
    atk: String,
    def: String,
    crit_rate: String,
    crit_dmg: String
}

#[derive(Serialize, Deserialize)]
pub struct CharacterRaw {
    id: String,
    name: String,
    vision: Vision,
    weapon: Weapon,
    nation: Nation,
    rarity: u8,
    stats: Vec<Stat>,
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    id: String,
    name: String,
    vision: Vision,
    weapon: Weapon,
    nation: Nation,
    rarity: u8,
    stats: Vec<Stat>,
    icon_url: String
}

impl Character {
    pub fn from_raw(character_raw: &CharacterRaw, icon: String) -> Self {
        Character {
            id: character_raw.id.clone(),
            name: character_raw.name.clone(),
            vision: character_raw.vision.clone(),
            weapon: character_raw.weapon.clone(),
            nation: character_raw.nation.clone(),
            rarity: character_raw.rarity,
            stats: character_raw.stats.clone(),
            icon_url: icon
        }
    }
}