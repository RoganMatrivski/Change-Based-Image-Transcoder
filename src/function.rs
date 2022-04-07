use crate::r#const::COLOR_PALETTE;

pub fn get_color_index(color: image::Rgba<u8>) -> u8 {
    let r_target = color.0[0];
    let g_target = color.0[1];
    let b_target = color.0[2];
    // let a_target = color.0[3];

    COLOR_PALETTE
        .iter()
        .position(|&(r, g, b)| r == r_target && g == g_target && b == b_target)
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn color_index_to_rgb(color_index: usize) -> (u8, u8, u8) {
    COLOR_PALETTE[color_index]
}
