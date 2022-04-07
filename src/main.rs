use std::{env::args, fs, vec};

use image::{io::Reader as imageReader, GenericImageView};
use rayon::prelude::*;

use function::*;
use r#const::*;
use r#struct::*;

mod r#const;
mod function;
mod r#struct;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize

    let img_path: Vec<String> = args().collect();
    let img_path = &img_path[1];

    let img_paths = fs::read_dir(img_path)?;
    let mut src_img = Img::default(2000, 2000);

    let mut img_changes = Vec::<Vec<PixelChanges>>::new();

    for (i, path) in img_paths.enumerate() {
        println!("Processing image {}", i);
        let mut current_img_changes = Vec::<PixelChanges>::new();
        let img = imageReader::open(path?.path())?.decode()?;

        let processed: Vec<(u32, u32, u8)> = img
            .pixels()
            .par_bridge()
            .filter_map(|px| if px.2[3] != 0 { Some(px) } else { None })
            .map(|(x, y, px)| (x, y, get_color_index(px)))
            .collect();

        for (x, y, new_color_index) in processed {
            if src_img.update_pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                new_color_index,
            ) {
                current_img_changes.push(PixelChanges::new(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    new_color_index,
                ));
            }
        }

        img_changes.push(current_img_changes)
    }

    let img_buffer = src_img.to_rgb8_vec();

    let mut raw_bytes: Vec<u8> = vec![];

    raw_bytes.extend(u16::try_from(src_img.img_x_size).unwrap().to_be_bytes()); // Size X
    raw_bytes.extend(u16::try_from(src_img.img_y_size).unwrap().to_be_bytes()); // Size Y

    raw_bytes.push(COLOR_PALETTE.len().try_into().unwrap()); // Palette amount

    // Palettes
    let palette_arr = COLOR_PALETTE.iter().map(|x| [x.0, x.1, x.2]);
    for color in palette_arr {
        raw_bytes.extend(color);
    }

    raw_bytes.push(31); // Default color index. 31 = white

    for px_change_frame in img_changes {
        for px_change in px_change_frame {
            raw_bytes.extend(u16::try_from(px_change.x).unwrap().to_be_bytes()); // X change coords
            raw_bytes.extend(u16::try_from(px_change.y).unwrap().to_be_bytes()); // Y change coords
            raw_bytes.push(px_change.new_color); // New color
        }

        raw_bytes.extend([2, 4, 3, 4]); // Space for each frame
    }

    fs::write("changes.cbpf", raw_bytes).unwrap();

    image::save_buffer("final.png", &img_buffer, 2000, 2000, image::ColorType::Rgb8).unwrap();

    Ok(())
}
