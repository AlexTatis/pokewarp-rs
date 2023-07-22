use crate::{pokedex::{self}, pokemon::pk5::PK5};

pub struct Save<'a> {
    save_file: &'a [u8],
    pokemons: &'a pokedex::Pokemons,
    abilities: &'a pokedex::Abilities,
    moves: &'a pokedex::Moves,
    natures: &'a pokedex::Natures,
    items: &'a pokedex::Items,
}

impl<'a> Save<'a> {
    pub fn new(
        save_file: &'a [u8],
        pokemons: &'a pokedex::Pokemons,
        abilities: &'a pokedex::Abilities,
        moves: &'a pokedex::Moves,
        natures: &'a pokedex::Natures,
        items: &'a pokedex::Items,
    ) -> Self {
        Save { save_file, pokemons, abilities, moves, natures, items }
    }

    pub fn get_raw_party(&self) -> &[u8] {
        &self.save_file[0x18E08..0x19333 + 1]
    }

    pub fn get_party(&self) -> [PK5; 6] {
        let party: [PK5; 6] = [
            PK5::new(&self.save_file[0x18E08 + 220 * 0..0x18E08 + 220 * 1], self.pokemons, self.abilities, self.moves, self.natures, self.items),
            PK5::new(&self.save_file[0x18E08 + 220 * 1..0x18E08 + 220 * 2], self.pokemons, self.abilities, self.moves, self.natures, self.items),
            PK5::new(&self.save_file[0x18E08 + 220 * 2..0x18E08 + 220 * 3], self.pokemons, self.abilities, self.moves, self.natures, self.items),
            PK5::new(&self.save_file[0x18E08 + 220 * 3..0x18E08 + 220 * 4], self.pokemons, self.abilities, self.moves, self.natures, self.items),
            PK5::new(&self.save_file[0x18E08 + 220 * 4..0x18E08 + 220 * 5], self.pokemons, self.abilities, self.moves, self.natures, self.items),
            PK5::new(&self.save_file[0x18E08 + 220 * 5..0x18E08 + 220 * 6], self.pokemons, self.abilities, self.moves, self.natures, self.items),
        ];

        party
    }
}
