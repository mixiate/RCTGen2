use crate::adjacent_track;
use crate::adjacent_track::TrackSectionWithSprites;
use crate::drawing;
use crate::drawing::Options;
use crate::drawing::blit;
use crate::render::TrackImage;
use crate::sprites;
use make_track::track_desc;
use make_track::track_desc::TrackSectionSprites;
use renderer::image::Image;

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
    options: &Options,
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
            let tile_coords = drawing::rotate_coords(tile_coords, (*rotation).into());
            let coords = drawing::rotate_coords(&drawing::add_coords(coords, &tile_coords), track_image.rotation);
            for sprite in &track_sprites[sprite_rotation] {
                let coords = drawing::add_coords(&coords, &sprite.offset);
                if !compare_coords(&coords, draw_order) {
                    continue;
                }
                if let Some(sprite) = sprites.get(sprite.index) {
                    blit::draw_indexed_image(buffer, sprite, &coords, options.colour_1, options.colour_2);
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
    options: &Options,
    buffer: &mut Image,
) {
    for (tile_coords, track_sprites) in track_section.tiles.iter().zip(track_sprites.iter()) {
        let coords = drawing::rotate_coords(tile_coords, rotation);
        for sprite in &track_sprites[rotation] {
            let coords = drawing::add_coords(&coords, &sprite.offset);
            if let Some(sprite) = sprites.get(sprite.index) {
                blit::draw_indexed_image(buffer, sprite, &coords, options.colour_1, options.colour_2);
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

    draw_adjacent_track_section(
        track_image,
        options,
        sprites,
        &adjacent_sections,
        DrawOrder::Before,
        buffer,
    );
    if options.original_track
        && let Some(track_sprites) = track_desc_sprites.get(track_image.track_section.name)
    {
        draw_original_track_section(
            track_image.track_section,
            track_image.rotation,
            sprites,
            track_sprites,
            options,
            buffer,
        );
    } else if options.indexed {
        blit::draw_indexed_image(
            buffer,
            &track_image.images.indexed,
            &[0; 3],
            options.colour_1,
            options.colour_2,
        );
    } else {
        blit::draw_image(buffer, &track_image.images.unindexed, &[0; 3]);
    }
    draw_adjacent_track_section(
        track_image,
        options,
        sprites,
        &adjacent_sections,
        DrawOrder::After,
        buffer,
    );
}

pub fn draw(
    track_desc: &track_desc::Desc,
    track_image: &TrackImage,
    options: &Options,
    adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
    mut sprites: Option<&mut sprites::Sprites>,
    buffer: &mut Image,
) {
    if options.supports
        && let Some(sprites) = sprites.as_mut()
        && let Some(metal_supports) = &track_desc.metal_supports
        && let Some(supports) = metal_supports.sections.get(track_image.track_section.name)
    {
        drawing::supports_metal::draw_supports(
            buffer,
            track_image.track_section,
            supports,
            track_image.rotation,
            options.colour_3,
            metal_supports.support_type,
            sprites,
        );
    }

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
            options,
            buffer,
        );
    } else if options.indexed {
        blit::draw_indexed_image(
            buffer,
            &track_image.images.indexed,
            &[0; 3],
            options.colour_1,
            options.colour_2,
        );
    } else {
        blit::draw_image(buffer, &track_image.images.unindexed, &[0; 3]);
    }
}
