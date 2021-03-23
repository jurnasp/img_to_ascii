use std::collections::HashMap;

pub trait CharacterSet {
    fn convert_usize_to_char(&self, char_byte: u8) -> String {
        let i = (char_byte / self.get_step()).checked_sub(1).unwrap_or(0);

        self.get_character(i)
    }
    fn get_step(&self) -> u8 {
        self.div_up(self.get_limit(), self.get_character_set_size())
    }
    fn div_up(&self, a: u8, b: u8) -> u8 {
        (a - 1) / b + 1
    }
    fn get_character_set_size(&self) -> u8;
    fn get_limit(&self) -> u8 {
        255
    }
    fn get_character(&self, index: u8) -> String;
}

pub fn construct_character_set_from_string(characters: String) -> HashMap<usize, String> {
    characters
        .chars()
        .enumerate()
        .map(|(k, c)| (k, c.to_string()))
        .collect::<HashMap<usize, String>>()
}
