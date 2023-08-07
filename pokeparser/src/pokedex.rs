use std::{fs::File, io::BufReader, collections::HashMap};
use crate::utils::{PokedexEntry, AbilityEntry, MoveEntry, NatureEntry, ItemEntry};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No pokemon found with the given id")]
    PokemonError,
    #[error("No ability found with the given id")]
    AbilityError,
    #[error("No move found with the given id")]
    MoveError,
    #[error("No nature found with the given id")]
    NatureError,
    #[error("No item found with the given id")]
    ItemError
} 

#[derive(Debug)]
pub struct Pokemons (HashMap<String, PokedexEntry>);

impl Pokemons {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap_or_else(|_| panic!("Could not open {}", path)));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Result<PokedexEntry, Error> {
        self.0.values().find(|e| e.num == num).ok_or(Error::PokemonError).cloned()
    }
}


#[derive(Debug)]
pub struct Abilities (HashMap<String, AbilityEntry>);

impl Abilities {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap_or_else(|_| panic!("Could not open {}", path)));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Result<AbilityEntry, Error> {
        self.0.values().find(|e| e.num == num).ok_or(Error::AbilityError).cloned()
    }
}


#[derive(Debug)]
pub struct Moves (HashMap<String, MoveEntry>);

impl Moves {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap_or_else(|_| panic!("Could not open {}", path)));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Result<MoveEntry, Error> {
        self.0.values().find(|e| e.num == num).ok_or(Error::MoveError).cloned()
    }
}


#[derive(Debug)]
pub struct Natures (Vec<NatureEntry>);

impl Natures {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap_or_else(|_| panic!("Could not open {}", path)));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: usize) -> Result<NatureEntry, Error> {
        self.0.get(num).ok_or(Error::NatureError).cloned()
    }
}


#[derive(Debug)]
pub struct Items (HashMap<String, ItemEntry>);

impl Items {
    pub fn new(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).unwrap_or_else(|_| panic!("Could not open {}", path)));
        Self(serde_json::from_reader(reader).expect("Could not parse pokedex.json"))
    }

    pub fn get(&self, num: u16) -> Result<ItemEntry, Error> {
        self.0.values().find(|e| e.num == num).ok_or(Error::ItemError).cloned()
    }
}