use crate::drawing;
use crate::drawing::blit;
use crate::track_editor::adjacent_track;
use crate::track_editor::render::TrackImage;
use make_track::track_desc;
use make_track::track_desc::TrackSectionSprites;
use rct::world_coords::Coords;
use renderer::image::Image;

pub struct Options {
    pub indexed: bool,
    pub adjacent_track: bool,
    pub original_track: bool,
    pub supports: bool,
    pub colours: [openrct2::colour::Colour; 3],
}

impl Default for Options {
    fn default() -> Self {
        Options {
            indexed: true,
            adjacent_track: false,
            original_track: false,
            supports: false,
            colours: [
                openrct2::colour::Colour::LightBlue,
                openrct2::colour::Colour::BrightPink,
                openrct2::colour::Colour::Yellow,
            ],
        }
    }
}

#[derive(Clone, Copy)]
enum DrawOrder {
    Before,
    After,
}

fn compare_coords(coords: &Coords, draw_order: DrawOrder) -> bool {
    match draw_order {
        DrawOrder::Before => coords.x + coords.y < 0 || (coords.x + coords.y == 0 && coords.z <= 0),
        DrawOrder::After => coords.x + coords.y > 0 || (coords.x + coords.y == 0 && coords.z >= 0),
    }
}

#[expect(clippy::too_many_arguments)]
fn draw_original_track_section(
    buffer: &mut Image,
    options: &Options,
    track_section: &make_track::track_sections::TrackSection,
    coords: Coords,
    rotation: usize,
    track_rotation: usize,
    sprites: &rct::csg::Archive,
    track_sprites: &TrackSectionSprites,
    draw_order: Option<DrawOrder>,
) {
    for (tile_coords, track_sprites) in track_section.tiles.iter().zip(track_sprites.iter()) {
        let tile_coords = Coords::from(tile_coords).rotate(track_rotation);
        let coords = (coords + tile_coords).rotate(rotation);
        for sprite in &track_sprites[(rotation + track_rotation) % 4] {
            let coords = coords + Coords::from(sprite.offset);
            if let Some(draw_order) = draw_order
                && !compare_coords(&coords, draw_order)
            {
                continue;
            }
            blit::draw_csg_sprite(
                buffer,
                sprites,
                sprite.index,
                &coords,
                options.colours[0],
                options.colours[1],
            );
        }
    }
}

fn draw_with_adjacent_sprites(
    track_image: &TrackImage,
    z_offset: i16,
    options: &Options,
    adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
    rct2_sprites: &rct::csg::Archive,
    track_desc_sprites: &indexmap::IndexMap<String, TrackSectionSprites>,
    buffer: &mut Image,
) {
    for adjacent_track_section in adjacent_track_sections {
        if let Some(track_sprites) = track_desc_sprites.get(adjacent_track_section.track_section.name) {
            draw_original_track_section(
                buffer,
                options,
                adjacent_track_section.track_section,
                adjacent_track_section.coords.into(),
                track_image.rotation,
                adjacent_track_section.rotation.into(),
                rct2_sprites,
                track_sprites,
                Some(DrawOrder::Before),
            );
        }
    }

    if options.original_track
        && let Some(track_sprites) = track_desc_sprites.get(track_image.track_section.name)
    {
        draw_original_track_section(
            buffer,
            options,
            track_image.track_section,
            Coords::ZERO,
            track_image.rotation,
            0,
            rct2_sprites,
            track_sprites,
            None,
        );
    } else if options.indexed {
        blit::draw_indexed_image(
            buffer,
            &track_image.images.indexed,
            &Coords::new(0, 0, z_offset.into()),
            options.colours[0],
            options.colours[1],
        );
    } else {
        blit::draw_image(
            buffer,
            &track_image.images.unindexed,
            &Coords::new(0, 0, z_offset.into()),
        );
    }
    for adjacent_track_section in adjacent_track_sections {
        if let Some(track_sprites) = track_desc_sprites.get(adjacent_track_section.track_section.name) {
            draw_original_track_section(
                buffer,
                options,
                adjacent_track_section.track_section,
                adjacent_track_section.coords.into(),
                track_image.rotation,
                adjacent_track_section.rotation.into(),
                rct2_sprites,
                track_sprites,
                Some(DrawOrder::After),
            );
        }
    }
}

pub fn draw_track(
    track: &track_desc::Track,
    metal_supports: Option<&track_desc::MetalSupports>,
    track_image: &TrackImage,
    options: &Options,
    adjacent_track_sections: Option<&adjacent_track::AdjacentTrackSections>,
    rct2_sprites: Option<&rct::csg::Archive>,
    buffer: &mut Image,
) {
    let z_offset = (track.z_offset - 16) as i16;
    if options.supports
        && let Some(rct2_sprites) = rct2_sprites
        && let Some(metal_supports) = metal_supports
        && let Some(supports) = metal_supports.sections.get(track_image.track_section.name)
    {
        drawing::supports_metal::draw_supports(
            buffer,
            track_image.track_section,
            supports,
            track_image.rotation,
            options.colours[2],
            metal_supports.support_type,
            rct2_sprites,
        );
    }

    if options.adjacent_track
        && let Some(adjacent_track_sections) = adjacent_track_sections
        && let Some(rct2_sprites) = rct2_sprites
    {
        draw_with_adjacent_sprites(
            track_image,
            z_offset,
            options,
            adjacent_track_sections,
            rct2_sprites,
            &track.original_sprites,
            buffer,
        );
    } else if options.original_track
        && let Some(rct2_sprites) = rct2_sprites
        && let Some(track_sprites) = track.original_sprites.get(track_image.track_section.name)
    {
        draw_original_track_section(
            buffer,
            options,
            track_image.track_section,
            Coords::ZERO,
            track_image.rotation,
            0,
            rct2_sprites,
            track_sprites,
            None,
        );
    } else if options.indexed {
        blit::draw_indexed_image(
            buffer,
            &track_image.images.indexed,
            &Coords::new(0, 0, z_offset.into()),
            options.colours[0],
            options.colours[1],
        );
    } else {
        blit::draw_image(
            buffer,
            &track_image.images.unindexed,
            &Coords::new(0, 0, z_offset.into()),
        );
    }
}
