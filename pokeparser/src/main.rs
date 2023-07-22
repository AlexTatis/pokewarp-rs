use std::fs;
use pokeparser::{pokedex, saves};
use serde_json::json;

fn main() {

    let pokemons = pokedex::Pokemons::new("./data/pokedex.json");
    let abilities = pokedex::Abilities::new("./data/abilities.json");
    let moves = pokedex::Moves::new("./data/moves.json");
    let natures = pokedex::Natures::new("./data/natures.json");
    let items = pokedex::Items::new("./data/items.json");
    
    let save_file = fs::read("Black 1.sav").expect("Could not open save file");
    let gen5_save = saves::gen5::Save::new(save_file.as_slice(), &pokemons, &abilities, &moves, &natures, &items);
    let pkms = gen5_save.get_party();

    println!("{}", json!(pkms).to_string());
    

}


