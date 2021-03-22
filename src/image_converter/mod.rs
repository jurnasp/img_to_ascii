use character_set_trait::CharacterSet;
use image::GrayImage;
pub mod character_set_trait;

pub struct ImageConverter<T> {
    character_set: T,
    x_position: u32,
    image_width: u32,
}

impl<T: CharacterSet> ImageConverter<T> {
    pub fn new(character_set: T) -> Self {
        ImageConverter {
            character_set,
            x_position: 0,
            image_width: 0,
        }
    }

    pub fn convert(&mut self, img: &mut GrayImage) -> String {
        let mut result = String::from("");
        self.image_width = img.width();

        let mut index = 0;
        for pixel in img.pixels_mut() {
            let pixel_value = pixel.0.to_vec()[0];
            self.x_position = index % self.image_width;
            let character = self.character_set.convert_usize_to_char(pixel_value);

            result += &(self.format_character(character));
            index += 1;
        }

        result
    }

    fn format_character(&self, character: String) -> String {
        let mut result = character + " ";
        if self.x_position == self.image_width - 1 {
            result += &"\n";
        }
        result
    }
}
