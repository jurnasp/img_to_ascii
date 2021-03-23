use crate::image_converter::character_set_trait::{
    construct_character_set_from_string, CharacterSet,
};
use std::collections::HashMap;

pub struct MaxCharacterSet {
    characters: HashMap<usize, String>,
}

impl MaxCharacterSet {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MaxCharacterSet {
            characters: construct_character_set_from_string(
                "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "
                    .to_string(),
            ),
        }
    }
}

impl CharacterSet for MaxCharacterSet {
    fn get_character_set_size(&self) -> u8 {
        self.characters.len() as u8
    }

    fn get_character(&self, index: u8) -> String {
        self.characters[&(index as usize)].clone()
    }
}
