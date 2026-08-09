use crate::drawing;
use crate::drawing::blit;
use crate::sprites;
use make_track::track_desc::{MetalSupport, TrackSectionMetalSupports};
use openrct2::supports::{MetalSupportType, SupportPosition};
use renderer::image::Image;
use strum::EnumCount as _;

static POSITION_COORD_OFFSETS: [[i16; 3]; SupportPosition::COUNT] = [
    [4, 4, 0],
    [28, 4, 0],
    [4, 28, 0],
    [28, 28, 0],
    [16, 16, 0],
    [16, 4, 0],
    [4, 16, 0],
    [28, 16, 0],
    [16, 28, 0],
];

#[derive(Clone, Copy, strum::EnumCount)]
enum SupportGraphicType {
    Tubes,
    Fork,
    ForkAlt,
    Boxed,
    Stick,
    StickAlt,
    ThickCentred,
    Thick,
    ThickAlt,
    ThickAltCentred,
    Truss,
    TubesInverted,
    BoxedCoated,
}

fn rotate_support_type(support_type: MetalSupportType, rotation: usize) -> SupportGraphicType {
    use SupportGraphicType as T;
    use strum::EnumCount as _;

    static ROTATION_MAP: [[SupportGraphicType; 4]; MetalSupportType::COUNT] = [
        [T::Tubes, T::Tubes, T::Tubes, T::Tubes],                     // Tubes
        [T::Fork, T::ForkAlt, T::Fork, T::ForkAlt],                   // Fork
        [T::Boxed, T::Boxed, T::Boxed, T::Boxed],                     // Boxed
        [T::Stick, T::StickAlt, T::Stick, T::StickAlt],               // Stick
        [T::Thick, T::ThickAlt, T::ThickCentred, T::ThickAltCentred], // Thick
        [T::Truss, T::Truss, T::Truss, T::Truss],                     // Truss
        [T::TubesInverted, T::TubesInverted, T::TubesInverted, T::TubesInverted], // TubesInverted
        [T::BoxedCoated, T::BoxedCoated, T::BoxedCoated, T::BoxedCoated], // BoxedCoated
    ];

    ROTATION_MAP[support_type as usize][rotation % 4]
}

struct SpriteIndices {
    base: Option<u32>,
    beam: u32,
    beam_capped: u32,
}

#[rustfmt::skip]
static SUPPORT_TYPE_SPRITE_INDICES: [SpriteIndices; SupportGraphicType::COUNT] = [
    SpriteIndices { base: Some(3243), beam: 3209, beam_capped: 3226 }, // Tubes
    SpriteIndices { base: Some(3279), beam: 3262, beam_capped: 3262 }, // Fork
    SpriteIndices { base: Some(3298), beam: 3262, beam_capped: 3262 }, // ForkAlt
    SpriteIndices { base: Some(3334), beam: 3317, beam_capped: 3317 }, // Boxed
    SpriteIndices { base: None, beam: 3658, beam_capped: 3658 },       // Stick
    SpriteIndices { base: None, beam: 3658, beam_capped: 3658 },       // StickAlt
    SpriteIndices { base: None, beam: 3141, beam_capped: 3141 },       // ThickCentred
    SpriteIndices { base: None, beam: 3158, beam_capped: 3158 },       // Thick
    SpriteIndices { base: None, beam: 3175, beam_capped: 3175 },       // ThickAlt
    SpriteIndices { base: None, beam: 3192, beam_capped: 3192 },       // ThickAltCentred
    SpriteIndices { base: None, beam: 3124, beam_capped: 3124 },       // Truss
    SpriteIndices { base: Some(3243), beam: 3209, beam_capped: 3226 }, // TubesInverted
    SpriteIndices { base: Some(3334), beam: 3353, beam_capped: 3353 }, // BoxedCoated
];

fn draw_support_segments(
    buffer: &mut Image,
    coords: &mut [i16; 3],
    colour: openrct2::colour::Colour,
    mut height_remaining: i16,
    sprite_index: u32,
    sprites: &mut sprites::Sprites,
) {
    const MAX_SEGMENT_HEIGHT: i16 = 16;
    while height_remaining > 0 {
        let segment_height = std::cmp::min(height_remaining, MAX_SEGMENT_HEIGHT);
        let sprite_index = sprite_index + segment_height as u32 - 1;

        if let Some(sprite) = sprites.get(sprite_index) {
            blit::draw_indexed_image(buffer, sprite, coords, colour, colour);
        }

        height_remaining -= segment_height;
        coords[2] += segment_height;
    }
}

fn draw_support(
    buffer: &mut Image,
    coords: &[i16; 3],
    support: &MetalSupport,
    rotation: usize,
    colour: openrct2::colour::Colour,
    support_type: MetalSupportType,
    sprites: &mut sprites::Sprites,
) {
    const SUPPORT_START_HEIGHT: i16 = -32;
    let mut height_remaining = coords[2] + i16::from(support.height) - SUPPORT_START_HEIGHT;

    let mut coords = drawing::rotate_coords(coords, rotation);
    let position = support.position.rotate(rotation);
    coords = drawing::add_coords(&coords, &POSITION_COORD_OFFSETS[position as usize]);
    coords[2] = SUPPORT_START_HEIGHT;

    let support_type = rotate_support_type(support_type, (rotation + usize::from(support.rotation)) % 4);
    let sprite_indices = &SUPPORT_TYPE_SPRITE_INDICES[support_type as usize];

    if let Some(base_index) = sprite_indices.base {
        if let Some(sprite) = sprites.get(base_index) {
            blit::draw_indexed_image(buffer, sprite, &coords, colour, colour);
        }
        height_remaining -= 6;
        coords[2] += 6;
    }

    draw_support_segments(
        buffer,
        &mut coords,
        colour,
        height_remaining,
        sprite_indices.beam,
        sprites,
    );

    height_remaining = support.extra_heights[rotation % 4];
    let sprite_index = if height_remaining < 0 {
        height_remaining = height_remaining.abs();
        coords[2] -= 1;
        sprite_indices.beam_capped
    } else {
        sprite_indices.beam
    };
    draw_support_segments(buffer, &mut coords, colour, height_remaining, sprite_index, sprites);
}

pub fn draw_supports(
    buffer: &mut Image,
    track_section: &make_track::track_sections::TrackSection,
    supports: &TrackSectionMetalSupports,
    rotation: usize,
    colour: openrct2::colour::Colour,
    support_type: MetalSupportType,
    sprites: &mut sprites::Sprites,
) {
    for (tile_coords, support) in track_section.tiles.iter().zip(supports.iter()) {
        if let Some(support) = support {
            draw_support(buffer, tile_coords, support, rotation, colour, support_type, sprites);
        }
    }
}
