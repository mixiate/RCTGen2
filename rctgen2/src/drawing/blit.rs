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

fn blit_pixel(buffer: &mut Image, x: usize, y: usize, pixel: u8, colour_1: Colour, colour_2: Colour) {
    let pixel = if (243..=254).contains(&pixel) {
        openrct2::colour::COLOUR_RAMPS[colour_1 as usize][usize::from(pixel) - 243]
    } else if (202..=213).contains(&pixel) {
        openrct2::colour::COLOUR_RAMPS[colour_2 as usize][usize::from(pixel) - 202]
    } else {
        pixel
    };
    if pixel != 0 {
        let [r, g, b] = renderer::palette::PALETTE[usize::from(pixel)];
        buffer.set_pixel(x, y, [r, g, b, 255]);
    }
}

#[expect(clippy::too_many_arguments)]
fn draw_sprite(
    buffer: &mut Image,
    width: u16,
    height: u16,
    offset: glam::IVec2,
    pixels: &[u8],
    coords: &Coords,
    colour_1: Colour,
    colour_2: Colour,
) {
    let (dest_x, dest_y, src_x, src_y, clipped_width, clipped_height) = clip_image_to_buffer(
        &ScreenCoords::from(coords),
        buffer.width(),
        buffer.height(),
        buffer.offset,
        width,
        height,
        offset,
    );
    'outer: for y in 0..clipped_height {
        for x in 0..clipped_width {
            let index = (src_x + x) as usize + ((src_y + y) as usize * usize::from(width));
            let Some(pixel) = pixels.get(index) else {
                break 'outer;
            };
            let x = (dest_x + x) as usize;
            let y = (dest_y + y) as usize;
            blit_pixel(buffer, x, y, *pixel, colour_1, colour_2);
        }
    }
}

fn draw_compressed_sprite(
    buffer: &mut Image,
    data: &rct::csg::CompressedSpriteData,
    coords: &Coords,
    colour_1: Colour,
    colour_2: Colour,
) {
    let (dest_x, dest_y, src_x, src_y, width, height) = clip_image_to_buffer(
        &ScreenCoords::from(coords),
        buffer.width(),
        buffer.height(),
        buffer.offset,
        data.entry.width,
        data.entry.height,
        glam::IVec2::new(i32::from(data.entry.offset_x), i32::from(data.entry.offset_y)),
    );
    'outer: for y in src_y..(src_y + height) {
        let Some(data) = data.get_row_data(y as usize) else {
            break;
        };
        let mut index = 0;
        loop {
            let (pixel_count, end) = {
                let Some(byte) = data.get(index) else {
                    break 'outer;
                };
                (byte & 0b0111_1111, (byte & 0b1000_0000) != 0)
            };
            index += 1;

            let Some(x) = data.get(index).map(|x| i32::from(*x)) else {
                break 'outer;
            };
            index += 1;

            for x in x..(x + i32::from(pixel_count)) {
                if x >= src_x && x < src_x + width {
                    let Some(pixel) = data.get(index) else {
                        break 'outer;
                    };
                    let x = (dest_x + x - src_x) as usize;
                    let y = (dest_y + y - src_y) as usize;
                    blit_pixel(buffer, x, y, *pixel, colour_1, colour_2);
                }

                index += 1;
            }

            if end {
                break;
            }
        }
    }
}

pub fn draw_csg_sprite(
    buffer: &mut Image,
    archive: &rct::csg::Archive,
    index: u32,
    coords: &Coords,
    colour_1: Colour,
    colour_2: Colour,
) {
    if let Some(entry) = archive.entries().get(index as usize) {
        match &archive.get_entry_data(entry) {
            Some(rct::csg::EntryData::Uncompressed(pixels)) => {
                let offset = glam::IVec2::new(i32::from(entry.offset_x), i32::from(entry.offset_y));
                draw_sprite(
                    buffer,
                    entry.width,
                    entry.height,
                    offset,
                    pixels,
                    coords,
                    colour_1,
                    colour_2,
                );
            }
            Some(rct::csg::EntryData::Compressed(data)) => {
                draw_compressed_sprite(buffer, data, coords, colour_1, colour_2);
            }
            None => {}
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
    draw_sprite(
        buffer,
        image.width(),
        image.height(),
        image.offset,
        image.as_raw(),
        coords,
        colour_1,
        colour_2,
    );
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
