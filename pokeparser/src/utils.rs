use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PokedexEntry {
    pub num: u16,
    pub name: String,
    pub types: Vec<String>,
    pub base_stats: Stats,
    pub abilities: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbilityEntry {
    pub num: u16,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MoveEntry {
    pub num: u16,
    pub name: String,
    pub alias: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemEntry {
    pub num: u16,
    pub name: String,
    pub alias: String,
    pub sprite: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NatureEntry {
    pub name: String,
    pub slug: String,
}


#[derive(Clone, Debug, Serialize)]
pub struct Move {
    pub num: u16,
    pub name: String,
    pub alias: String,
    pub r#type: String,
    pub pp: u8,
}

impl Move {
    pub fn from_entry(entry: MoveEntry, pp: u8) -> Self {
        Self {
            num: entry.num,
            name: entry.name,
            alias: entry.alias,
            r#type: entry.r#type,
            pp,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Gender {
    M,
    F,
    Genderless
}

impl Gender {
    pub fn from_byte(byte: u8) -> Self {
        if (byte >> 1) & 0x1 != 0 {
            return Self::F
        }

        if (byte >> 2) & 0x1 != 0 {
            return Self::Genderless
        }

        Self::M
    }
}

pub enum Types {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    Unknown,
    Shadow,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}
