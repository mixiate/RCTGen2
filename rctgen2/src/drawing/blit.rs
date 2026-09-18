use openrct2::colour::Colour;
use rct::screen_coords::ScreenCoords;
use rct::world_coords::Coords;
use renderer::image::{Image, IndexedImage};

fn clip_image_to_buffer(
    position: &ScreenCoords,
    dest_width: usize,
    dest_height: usize,
    dest_offset: glam::IVec2,
    src_width: u16,
    src_height: u16,
    src_offset: glam::IVec2,
) -> (i32, i32, i32, i32, i32, i32) {
    let dest_x = dest_offset.x + position.x + src_offset.x;
    let dest_y = dest_offset.y + position.y + src_offset.y;

    let src_x = -std::cmp::min(dest_x, 0);
    let src_y = -std::cmp::min(dest_y, 0);
    let dest_x = std::cmp::max(dest_x, 0);
    let dest_y = std::cmp::max(dest_y, 0);

    let mut width = i32::from(src_width) - src_x;
    let mut height = i32::from(src_height) - src_y;
    width -= std::cmp::max(dest_x + width - dest_width as i32, 0);
    height -= std::cmp::max(dest_y + height - dest_height as i32, 0);

    (dest_x, dest_y, src_x, src_y, width, height)
}

pub fn draw_image(buffer: &mut Image, image: &Image, coords: &Coords) {
    let (dest_x, dest_y, src_x, src_y, width, height) = clip_image_to_buffer(
        &ScreenCoords::from(coords),
        buffer.width(),
        buffer.height(),
        buffer.offset,
        image.width() as u16,
        image.height() as u16,
        image.offset,
    );
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel((src_x + x) as usize, (src_y + y) as usize);
            if pixel[3] != 0 {
                buffer.set_pixel((dest_x + x) as usize, (dest_y + y) as usize, pixel);
            }
        }
    }
}

pub fn draw_indexed_image(
    buffer: &mut Image,
    image: &IndexedImage,
    coords: &Coords,
    colour_1: Colour,
    colour_2: Colour,
) {
    let (dest_x, dest_y, src_x, src_y, width, height) = clip_image_to_buffer(
        &ScreenCoords::from(coords),
        buffer.width(),
        buffer.height(),
        buffer.offset,
        image.width(),
        image.height(),
        image.offset,
    );
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel((src_x + x) as usize, (src_y + y) as usize);
            let pixel = if (243..=254).contains(&pixel) {
                openrct2::colour::COLOUR_RAMPS[colour_1 as usize][usize::from(pixel) - 243]
            } else if (202..=213).contains(&pixel) {
                openrct2::colour::COLOUR_RAMPS[colour_2 as usize][usize::from(pixel) - 202]
            } else {
                pixel
            };
            if pixel != 0 {
                let [r, g, b] = renderer::palette::PALETTE[usize::from(pixel)];
                buffer.set_pixel((dest_x + x) as usize, (dest_y + y) as usize, [r, g, b, 255]);
            }
        }
    }
}
