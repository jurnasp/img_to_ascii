use std::collections::HashMap;

pub trait CharacterSet {
    fn convert_usize_to_char(&self, char_byte: u8) -> String {
        let i = (char_byte / self.get_step()).checked_sub(1).unwrap_or(0);

        self.get_character(i)
    }
    fn get_limit(&self) -> u8 {
        255
    }
    fn get_step(&self) -> u8;
    fn get_character(&self, index: u8) -> String;
}
