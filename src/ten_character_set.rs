use crate::image_converter::character_set_trait::CharacterSet;
use std::collections::HashMap;

pub struct TenCharacterSet {
    characters: HashMap<usize, String>,
}

impl TenCharacterSet {
    pub fn new() -> Self {
        TenCharacterSet {
            characters: construct_character_set_from_string("@%#*+=-:. ".to_string()),
        }
    }
}

impl CharacterSet for TenCharacterSet {
    fn get_step(&self) -> u8 {
        self.get_limit() / self.characters.len() as u8
    }

    fn get_character(&self, index: u8) -> String {
        self.characters[&(index as usize)].clone()
    }
}
fn construct_character_set_from_string(characters: String) -> HashMap<usize, String> {
    dbg!(characters
        .chars()
        .enumerate()
        .map(|(k, c)| (k, c.to_string()))
        .collect::<HashMap<usize, String>>());
    characters
        .chars()
        .enumerate()
        .map(|(k, c)| (k, c.to_string()))
        .collect::<HashMap<usize, String>>()
}
