use std::{fs::File, io::BufReader, collections::HashMap};
use crate::utils::{PokedexEntry, AbilityEntry, MoveEntry, NatureEntry, ItemEntry};


#[derive(Debug)]
pub struct Pokemons (HashMap<String, PokedexEntry>);

impl Pokemons {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect(format!("Could not open {}", path).as_str()));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Option<PokedexEntry> {
        self.0.values().find(|e| e.num == num).cloned()
    }
}


#[derive(Debug)]
pub struct Abilities (HashMap<String, AbilityEntry>);

impl Abilities {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect(format!("Could not open {}", path).as_str()));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Option<AbilityEntry> {
        self.0.values().find(|e| e.num == num).cloned()
    }
}


#[derive(Debug)]
pub struct Moves (HashMap<String, MoveEntry>);

impl Moves {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect(format!("Could not open {}", path).as_str()));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Option<MoveEntry> {
        self.0.values().find(|e| e.num == num).cloned()
    }
}


#[derive(Debug)]
pub struct Natures (Vec<NatureEntry>);

impl Natures {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect(format!("Could not open {}", path).as_str()));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: usize) -> Option<NatureEntry> {
        self.0.get(num).cloned()
    }
}


#[derive(Debug)]
pub struct Items (HashMap<String, ItemEntry>);

impl Items {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect(format!("Could not open {}", path).as_str()));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Option<ItemEntry> {
        self.0.values().find(|e| e.num == num).cloned()
    }
}