use crate::adjacent_track;
use crate::adjacent_track::TrackSectionWithSprites;
use crate::render::TrackImage;
use crate::sprites;
use make_track::track_desc;
use make_track::track_desc::TrackSectionSprites;
use renderer::image::{Image, IndexedImage};

pub struct Options {
    pub indexed: bool,
    pub adjacent_track: bool,
    pub original_track: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            indexed: true,
            adjacent_track: false,
            original_track: false,
        }
    }
}

fn add_coords(a: &[i16; 3], b: &[i16; 3]) -> [i16; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn rotate_coords(coordinates: &[i16; 3], rotation: usize) -> [i16; 3] {
    let [x, y, z] = *coordinates;
    match rotation {
        1 => [y, -x, z],
        2 => [-x, -y, z],
        3 => [-y, x, z],
        _ => [x, y, z],
    }
}

fn coords_to_screen_space(coordinates: &[i16; 3]) -> [i32; 2] {
    let [x, y, z] = coordinates;
    let offset_x = x - y;
    let offset_y = (-(x + y) / 2) + z;
    [(-offset_x).into(), (-offset_y).into()]
}

fn clip_image_to_buffer(
    dest_width: usize,
    dest_height: usize,
    dest_offset: glam::IVec2,
    src_width: u16,
    src_height: u16,
    src_offset: glam::IVec2,
    coords: &[i16; 3],
) -> (i32, i32, i32, i32, i32, i32) {
    let position = coords_to_screen_space(coords);
    let dest_x = dest_offset.x + position[0] + src_offset.x;
    let dest_y = dest_offset.y + position[1] + src_offset.y;

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

fn draw_image(buffer: &mut Image, image: &Image, coords: &[i16; 3]) {
    let (dest_x, dest_y, src_x, src_y, width, height) = clip_image_to_buffer(
        buffer.width(),
        buffer.height(),
        buffer.offset,
        image.width() as u16,
        image.height() as u16,
        image.offset,
        coords,
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

fn draw_indexed_image(buffer: &mut Image, image: &IndexedImage, coords: &[i16; 3]) {
    let (dest_x, dest_y, src_x, src_y, width, height) = clip_image_to_buffer(
        buffer.width(),
        buffer.height(),
        buffer.offset,
        image.width(),
        image.height(),
        image.offset,
        coords,
    );
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel((src_x + x) as usize, (src_y + y) as usize);
            if pixel != 0 {
                let [r, g, b] = renderer::palette::PALETTE[usize::from(pixel)];
                buffer.set_pixel((dest_x + x) as usize, (dest_y + y) as usize, [r, g, b, 255]);
            }
        }
    }
}

#[derive(Clone, Copy)]
enum DrawOrder {
    Before,
    After,
}

fn compare_coords(coords: &[i16; 3], draw_order: DrawOrder) -> bool {
    match draw_order {
        DrawOrder::Before => coords[0] + coords[1] < 0 || (coords[0] + coords[1] == 0 && coords[2] <= 0),
        DrawOrder::After => coords[0] + coords[1] > 0 || (coords[0] + coords[1] == 0 && coords[2] >= 0),
    }
}

fn draw_adjacent_track_section(
    track_image: &TrackImage,
    sprites: &mut sprites::Sprites,
    adjacent_sections: &[TrackSectionWithSprites],
    draw_order: DrawOrder,
    buffer: &mut Image,
) {
    for TrackSectionWithSprites {
        track_section,
        coords,
        rotation,
        sprites: track_sprites,
    } in adjacent_sections
    {
        let sprite_rotation = (track_image.rotation + usize::from(*rotation)) % 4;

        for (tile_coords, track_sprites) in track_section.tiles.iter().zip(track_sprites.iter()) {
            let tile_coords = rotate_coords(tile_coords, (*rotation).into());
            let coords = rotate_coords(&add_coords(coords, &tile_coords), track_image.rotation);
            for sprite in &track_sprites[sprite_rotation] {
                let coords = add_coords(&coords, &sprite.offset);
                if !compare_coords(&coords, draw_order) {
                    continue;
                }
                if let Some(sprite) = sprites.get(sprite.index) {
                    draw_indexed_image(buffer, sprite, &coords);
                }
            }
        }
    }
}

fn draw_original_track_section(
    track_section: &make_track::track_sections::TrackSection,
    rotation: usize,
    sprites: &mut sprites::Sprites,
    track_sprites: &TrackSectionSprites,
    buffer: &mut Image,
) {
    for (tile_coords, track_sprites) in track_section.tiles.iter().zip(track_sprites.iter()) {
        let coords = rotate_coords(tile_coords, rotation);
        for sprite in &track_sprites[rotation] {
            let coords = add_coords(&coords, &sprite.offset);
            if let Some(sprite) = sprites.get(sprite.index) {
                draw_indexed_image(buffer, sprite, &coords);
            }
        }
    }
}

fn draw_with_adjacent_sprites(
    track_image: &TrackImage,
    options: &Options,
    adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
    sprites: &mut sprites::Sprites,
    track_desc_sprites: &indexmap::IndexMap<String, TrackSectionSprites>,
    buffer: &mut Image,
) {
    let adjacent_sections = adjacent_track::list_track_sections(
        track_image.track_section.name,
        adjacent_track_sections,
        track_desc_sprites,
    );

    draw_adjacent_track_section(track_image, sprites, &adjacent_sections, DrawOrder::Before, buffer);
    if options.original_track
        && let Some(track_sprites) = track_desc_sprites.get(track_image.track_section.name)
    {
        draw_original_track_section(
            track_image.track_section,
            track_image.rotation,
            sprites,
            track_sprites,
            buffer,
        );
    } else if options.indexed {
        draw_indexed_image(buffer, &track_image.images.indexed, &[0; 3]);
    } else {
        draw_image(buffer, &track_image.images.unindexed, &[0; 3]);
    }
    draw_adjacent_track_section(track_image, sprites, &adjacent_sections, DrawOrder::After, buffer);
}

pub fn draw(
    track_desc: &track_desc::Desc,
    track_image: &TrackImage,
    options: &Options,
    adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
    sprites: Option<&mut sprites::Sprites>,
    buffer: &mut Image,
) {
    if options.adjacent_track
        && let Some(sprites) = sprites
    {
        draw_with_adjacent_sprites(
            track_image,
            options,
            adjacent_track_sections,
            sprites,
            &track_desc.original_sprites,
            buffer,
        );
    } else if options.original_track
        && let Some(sprites) = sprites
        && let Some(track_sprites) = track_desc.original_sprites.get(track_image.track_section.name)
    {
        draw_original_track_section(
            track_image.track_section,
            track_image.rotation,
            sprites,
            track_sprites,
            buffer,
        );
    } else if options.indexed {
        draw_indexed_image(buffer, &track_image.images.indexed, &[0; 3]);
    } else {
        draw_image(buffer, &track_image.images.unindexed, &[0; 3]);
    }
}
