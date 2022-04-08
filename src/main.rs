use std::{env::args, fs, io::Write, vec};

use image::{io::Reader as imageReader, GenericImageView};
use rayon::prelude::*;

use function::*;
use r#const::*;
use r#struct::*;

mod r#const;
mod function;
mod r#struct;

fn setup() -> Result<std::io::BufWriter<std::fs::File>, Box<dyn std::error::Error>> {
    let file_dst = std::fs::File::create("changes.cbif")?;
    let file_buf = std::io::BufWriter::new(file_dst);
    Ok(file_buf)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize

    let mut file_buf = setup()?;

    let img_path: Vec<String> = args().collect();
    let img_path = &img_path[1];

    let img_paths = fs::read_dir(img_path)?;
    let mut src_img = Img::default(2000, 2000);

    let mut img_changes = Vec::<Vec<PixelChanges>>::new();

    for (i, path) in img_paths.enumerate() {
        println!("Processing image {}", i);
        let mut current_img_changes = Vec::<PixelChanges>::new();
        let img = image::load(
            std::io::BufReader::new(std::fs::File::open(path?.path())?),
            image::ImageFormat::Png,
        )?;

        let processed: Vec<(u32, u32, u8)> = img
            .pixels()
            .par_bridge()
            .filter_map(|px| if px.2[3] != 0 { Some(px) } else { None })
            .map(|(x, y, px)| (x, y, get_color_index(px)))
            .collect();

        for (x, y, new_color_index) in processed {
            if src_img.update_pixel(x.try_into()?, y.try_into()?, new_color_index) {
                current_img_changes.push(PixelChanges::new(
                    x.try_into()?,
                    y.try_into()?,
                    new_color_index,
                ));
            }
        }

        img_changes.push(current_img_changes)
    }

    let img_buffer = src_img.to_rgb8_vec();

    file_buf.write_all(&u16::try_from(src_img.img_x_size)?.to_be_bytes())?; // Size X
    file_buf.write_all(&u16::try_from(src_img.img_y_size)?.to_be_bytes())?; // Size Y

    file_buf.write_all(&[u8::try_from(COLOR_PALETTE.len())?])?; // Palette amount

    // Palettes
    let palette_arr = COLOR_PALETTE.iter().map(|x| [x.0, x.1, x.2]);
    for color in palette_arr {
        file_buf.write_all(&color)?;
    }

    file_buf.write_all(&[31])?; // Default color index. 31 = white

    for px_change_frame in &img_changes {
        for px_change in px_change_frame {
            file_buf.write_all(&u16::try_from(px_change.x)?.to_be_bytes())?; // X change coords
            file_buf.write_all(&u16::try_from(px_change.y)?.to_be_bytes())?; // Y change coords
            file_buf.write_all(&[px_change.new_color])?; // New color
        }

        file_buf.write_all(&[2, 4, 3, 4])?; // Space for each frame
    }

    image::save_buffer("final.png", &img_buffer, 2000, 2000, image::ColorType::Rgb8).unwrap();

    Ok(())
}
