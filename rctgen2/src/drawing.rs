use crate::adjacent_track;
use crate::adjacent_track::TrackSectionWithSprites;
use crate::render::{Texture, TrackTexture};
use crate::sprites;
use eframe::egui;
use make_track::track_desc;
use make_track::track_desc::TrackSectionSprites;

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

fn coords_to_screen_space(coordinates: &[i16; 3]) -> [i16; 2] {
    let [x, y, z] = coordinates;
    let offset_x = x - y;
    let offset_y = (-(x + y) / 2) + z;
    [-offset_x, -offset_y]
}

fn draw_sprite(ui: &mut egui::Ui, texture: &Texture, coords: &[i16; 3]) {
    let texture_size = texture.handle.size_vec2();
    let image = egui::Image::from_texture((texture.handle.id(), texture_size));

    let image_rect = {
        let position = coords_to_screen_space(coords);
        let mut image_pos = ui.max_rect().center();
        image_pos += egui::Vec2::new(position[0].into(), position[1].into());
        image_pos += egui::Vec2::new(texture.offset.x as f32, texture.offset.y as f32);
        egui::Rect::from_min_size(image_pos, texture_size)
    };

    ui.place(image_rect, image);
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
    main_sprite: &TrackTexture,
    original_sprites: &mut sprites::Sprites,
    adjacent_sections: &[TrackSectionWithSprites],
    draw_order: DrawOrder,
    ui: &mut egui::Ui,
) {
    for TrackSectionWithSprites {
        track_section,
        coords,
        rotation,
        sprites,
    } in adjacent_sections
    {
        let sprite_rotation = (main_sprite.rotation + usize::from(*rotation)) % 4;

        for (tile_coords, sprites) in track_section.tiles.iter().zip(sprites[sprite_rotation].iter()) {
            let tile_coords = rotate_coords(tile_coords, (*rotation).into());
            let coords = rotate_coords(&add_coords(coords, &tile_coords), main_sprite.rotation);
            for sprite in sprites {
                let coords = add_coords(&coords, &sprite.offset);
                if !compare_coords(&coords, draw_order) {
                    continue;
                }
                if let Some(texture) = original_sprites.get_sprite(sprite.index, ui.ctx()) {
                    draw_sprite(ui, texture, &coords);
                }
            }
        }
    }
}

fn draw_original_track_section(
    track_section: &make_track::track_sections::TrackSection,
    rotation: usize,
    original_sprites: &mut sprites::Sprites,
    track_desc_sprites: &TrackSectionSprites,
    ui: &mut egui::Ui,
) {
    for (tile_coords, sprites) in track_section.tiles.iter().zip(track_desc_sprites[rotation].iter()) {
        let coords = rotate_coords(tile_coords, rotation);
        for sprite in sprites {
            let coords = add_coords(&coords, &sprite.offset);
            if let Some(texture) = original_sprites.get_sprite(sprite.index, ui.ctx()) {
                draw_sprite(ui, texture, &coords);
            }
        }
    }
}

fn draw_with_adjacent_sprites(
    main_sprite: &TrackTexture,
    adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
    original_sprites: &mut sprites::Sprites,
    track_desc_sprites: &indexmap::IndexMap<String, TrackSectionSprites>,
    show_original_piece: bool,
    ui: &mut egui::Ui,
) {
    let adjacent_sections = adjacent_track::list_track_sections(
        main_sprite.track_section.name,
        adjacent_track_sections,
        track_desc_sprites,
    );

    draw_adjacent_track_section(main_sprite, original_sprites, &adjacent_sections, DrawOrder::Before, ui);
    if show_original_piece && let Some(sprites) = track_desc_sprites.get(main_sprite.track_section.name) {
        draw_original_track_section(
            main_sprite.track_section,
            main_sprite.rotation,
            original_sprites,
            sprites,
            ui,
        );
    } else {
        draw_sprite(ui, &main_sprite.texture, &[0; 3]);
    }
    draw_adjacent_track_section(main_sprite, original_sprites, &adjacent_sections, DrawOrder::After, ui);
}

pub fn draw(
    track_desc: &track_desc::Desc,
    main_sprite: &TrackTexture,
    show_adjacent_sprites: bool,
    show_original_piece: bool,
    adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
    original_sprites: Option<&mut sprites::Sprites>,
    ui: &mut egui::Ui,
) {
    if show_adjacent_sprites && let Some(original_sprites) = original_sprites {
        draw_with_adjacent_sprites(
            main_sprite,
            adjacent_track_sections,
            original_sprites,
            &track_desc.original_sprites,
            show_original_piece,
            ui,
        );
    } else if show_original_piece
        && let Some(original_sprites) = original_sprites
        && let Some(sprites) = track_desc.original_sprites.get(main_sprite.track_section.name)
    {
        draw_original_track_section(
            main_sprite.track_section,
            main_sprite.rotation,
            original_sprites,
            sprites,
            ui,
        );
    } else {
        draw_sprite(ui, &main_sprite.texture, &[0; 3]);
    }
}
