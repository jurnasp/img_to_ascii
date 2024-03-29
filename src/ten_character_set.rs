use crate::image_converter::character_set_trait::{
    construct_character_set_from_string, CharacterSet,
};
use std::collections::HashMap;

pub struct TenCharacterSet {
    characters: HashMap<usize, String>,
}

impl TenCharacterSet {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TenCharacterSet {
            characters: construct_character_set_from_string("@%#*+=-:. ".to_string()),
        }
    }
}

impl CharacterSet for TenCharacterSet {
    fn get_character_set_size(&self) -> u8 {
        self.characters.len() as u8
    }

    fn get_character(&self, index: u8) -> String {
        self.characters[&(index as usize)].clone()
    }
}
