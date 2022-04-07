use crate::function::color_index_to_rgb;

pub struct PixelChanges {
    pub x: usize,
    pub y: usize,
    pub new_color: u8,
}

impl PixelChanges {
    pub fn new(x_coord: usize, y_coord: usize, new_color: u8) -> PixelChanges {
        PixelChanges {
            x: x_coord,
            y: y_coord,
            new_color: new_color,
        }
    }
}

#[derive(Clone)]
pub struct Img {
    pub img_array: Vec<u8>,
    pub img_x_size: usize,
    pub img_y_size: usize,
}

impl Img {
    pub fn default(x: usize, y: usize) -> Img {
        let len_usize: usize = (x * y).try_into().unwrap();
        return Img {
            img_array: vec![31; len_usize],
            img_x_size: x,
            img_y_size: y,
        };
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, new_color: u8) {
        self.img_array[y * self.img_x_size + x % self.img_x_size] = new_color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.img_array[y * self.img_x_size + x % self.img_x_size]
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, new_color: u8) -> bool {
        if self.get_pixel(x, y) != new_color {
            self.set_pixel(x, y, new_color);
            return true;
        } else {
            return false;
        }
    }

    pub fn check_pixel(&self, x: usize, y: usize, color: u8) -> bool {
        self.get_pixel(x, y) == color
    }

    pub fn to_rgb8_vec(&self) -> Vec<u8> {
        let mut raw_img = Vec::<u8>::with_capacity(self.img_x_size * self.img_y_size * 3);

        for px in &self.img_array {
            let (r, g, b) = color_index_to_rgb((*px).into());

            raw_img.push(r);
            raw_img.push(g);
            raw_img.push(b);
        }

        raw_img
    }
}
