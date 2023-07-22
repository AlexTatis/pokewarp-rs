use crate::{
    pokedex,
    utils::{Gender, ItemEntry, Move, Stats},
};
use byteorder::{ByteOrder, LittleEndian};
use rug::Integer;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PK5 {
    pub id: u16,
    pub species: String,
    pub item_id: u16,
    pub exp: u32,
    pub ability: String,
    pub moves: Vec<Move>,
    pub ivs: Stats,
    pub evs: Stats,
    pub nature: String,
    pub nickname: String,
    pub happiness: u8,
    pub gender: Gender,
    pub level: u16,
    pub is_egg: bool,
    pub item: ItemEntry,
    pub pokeball: ItemEntry
}

const SHUFFLING_PATTERNS: &[&str; 24] = &[
    "ABCD", "ABDC", "ACBD", "ADBC", "ACDB", "ADCB", "BACD", "BADC", "CABD", "DABC", "CADB", "DACB",
    "BCAD", "BDAC", "CBAD", "DBAC", "CDAB", "DCAB", "BCDA", "BDCA", "CBDA", "DBCA", "CDBA", "DCBA",
];

impl PK5 {
    fn prng(seed: Integer) -> Integer {
        seed * 0x41C64E6D + 0x6073
    }

    fn populate_nickname(raw: &[u8]) -> String {
        let mut result = String::from("");

        raw.into_iter().for_each(|x| {
            if *x != 0xff && *x != 0x0 {
                result.push(char::from(*x))
            }
        });

        result
    }

    pub fn empty() -> Self {
        Self {
            id: 0,
            species: String::from("MissingNo."),
            item_id: 0,
            exp: 0,
            ability: String::from(""),
            moves: vec![],
            ivs: Stats {
                hp: 0,
                atk: 0,
                def: 0,
                spa: 0,
                spd: 0,
                spe: 0,
            },
            evs: Stats {
                hp: 0,
                atk: 0,
                def: 0,
                spa: 0,
                spd: 0,
                spe: 0,
            },
            nature: String::from(""),
            nickname: String::from("MissingNo."),
            happiness: 0,
            gender: Gender::M,
            level: 0,
            is_egg: false,
            item: ItemEntry {
                num: 0,
                name: String::from(""),
                alias: String::from(""),
                sprite: String::from(""),
            },
            pokeball: ItemEntry {
                num: 0,
                name: String::from(""),
                alias: String::from(""),
                sprite: String::from(""),
            },
        }
    }

    pub fn new(
        raw_data: &[u8],
        pokedex_pokemons: &pokedex::Pokemons,
        pokedex_abilities: &pokedex::Abilities,
        pokedex_moves: &pokedex::Moves,
        pokedex_natures: &pokedex::Natures,
        pokedex_items: &pokedex::Items,
    ) -> Self {
        let pid = LittleEndian::read_u32(&raw_data[..0x4]);
        let checksum = LittleEndian::read_u16(&raw_data[0x6..0x8]);
        let shuffle_index = (((pid & 0x3E000) >> 0xD) % 24) as usize;

        let mut decrypted: Vec<u8> = vec![];
        let mut final_data: Vec<u8> = vec![];
        let mut buffer = [0; 2];

        // Populate final_data with unencrypted section
        raw_data[..0x8].iter().for_each(|e| final_data.push(*e));

        // PRNG decryption
        let mut prng_result = Self::prng(checksum.into());
        for byte_pair in raw_data[0x8..0x88].chunks(2) {
            LittleEndian::write_u16(
                &mut buffer,
                LittleEndian::read_u16(byte_pair)
                    ^ Integer::from(&prng_result >> 16).to_u16_wrapping(),
            );

            //println!("{:#x?}", &prng_result);
            buffer.iter().for_each(|e| decrypted.push(*e));
            prng_result = Self::prng(prng_result);
        }

        // Block unshuffling
        let blocks: Vec<&[u8]> = decrypted.chunks(32).collect();

        for block_letter in SHUFFLING_PATTERNS[shuffle_index].chars() {
            match block_letter {
                'A' => {
                    blocks[0].iter().for_each(|e| final_data.push(*e));
                }
                'B' => {
                    blocks[1].iter().for_each(|e| final_data.push(*e));
                }
                'C' => {
                    blocks[2].iter().for_each(|e| final_data.push(*e));
                }
                'D' => {
                    blocks[3].iter().for_each(|e| final_data.push(*e));
                }
                _ => {
                    panic!("Shuffling pattern contains an unexpected letter")
                }
            }
        }

        // Check if the Pokemon is null

        if LittleEndian::read_u16(&final_data[0x8..0x10]) == 0 {
            Self::empty()
        } else {
            PK5 {
                id: LittleEndian::read_u16(&final_data[0x8..0x10]),
                species: pokedex_pokemons
                    .get(LittleEndian::read_u16(&final_data[0x8..0x10]))
                    .expect(format!("{}", LittleEndian::read_u16(&final_data[0x8..0x10])).as_str())
                    .name,
                item_id: LittleEndian::read_u16(&final_data[0xA..0xC]),
                exp: LittleEndian::read_u32(&final_data[0x10..0x14]),
                ability: pokedex_abilities.get(final_data[0x15] as u16).unwrap().name,
                moves: vec![
                    Move::from_entry(
                        pokedex_moves
                            .get(LittleEndian::read_u16(&final_data[0x28..0x2A]))
                            .expect(
                                format!("{}", LittleEndian::read_u16(&final_data[0x28..0x2A]))
                                    .as_str(),
                            ),
                        final_data[0x30],
                    ),
                    Move::from_entry(
                        pokedex_moves
                            .get(LittleEndian::read_u16(&final_data[0x2A..0x2C]))
                            .expect(
                                format!("{}", LittleEndian::read_u16(&final_data[0x2A..0x2C]))
                                    .as_str(),
                            ),
                        final_data[0x31],
                    ),
                    Move::from_entry(
                        pokedex_moves
                            .get(LittleEndian::read_u16(&final_data[0x2C..0x2E]))
                            .expect(
                                format!("{}", LittleEndian::read_u16(&final_data[0x8..0x10]))
                                    .as_str(),
                            ),
                        final_data[0x32],
                    ),
                    Move::from_entry(
                        pokedex_moves
                            .get(LittleEndian::read_u16(&final_data[0x2E..0x30]))
                            .unwrap(),
                        final_data[0x33],
                    ),
                ],
                ivs: Stats {
                    hp: (LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 00 & 0x1F) as u8,
                    atk: (LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 05 & 0x1F) as u8,
                    def: (LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 10 & 0x1F) as u8,
                    spe: (LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 15 & 0x1F) as u8,
                    spa: (LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 20 & 0x1F) as u8,
                    spd: (LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 25 & 0x1F) as u8,
                },
                evs: Stats {
                    hp: final_data[0x18],
                    atk: final_data[0x19],
                    def: final_data[0x1A],
                    spe: final_data[0x1B],
                    spa: final_data[0x1C],
                    spd: final_data[0x1D],
                },
                nature: pokedex_natures.get(final_data[0x41].into()).unwrap().name,
                nickname: Self::populate_nickname(&final_data[0x48..0x5D]),
                happiness: final_data[0x14],
                gender: Gender::from_byte(final_data[0x40]),
                level: 100,
                is_egg: ((LittleEndian::read_u32(&final_data[0x38..0x3C]) >> 30) & 0x1) == 1,
                item: pokedex_items.get(LittleEndian::read_u16(&final_data[0xA..0xC])).unwrap(),
                pokeball: pokedex_items.get(final_data[0x83] as u16).unwrap()
            }
        }
    }
}
