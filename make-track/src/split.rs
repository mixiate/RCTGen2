use crate::mask;

fn split_sprite(
    view: &mask::View,
    sprite: &mask::Sprite,
    mut image: renderer::image::IndexedImage,
    y_offset: i32,
) -> renderer::image::IndexedImage {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let mask_x = image.offset.x + i32::from(x);
            let mask_y = image.offset.y + i32::from(y) + y_offset;

            if !view.sample_primary(mask_x, mask_y, sprite.index) {
                image.set_pixel(x.into(), y.into(), 0);
            }
        }
    }
    image
}

fn split_sprite_depth(
    view: &mask::View,
    sprite: &mask::Sprite,
    mut image: renderer::image::IndexedImage,
    y_offset: i32,
    track_depth: &renderer::DepthBuffer,
    mask_depth: &renderer::DepthBuffer,
    operation: mask::Operation,
) -> renderer::image::IndexedImage {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let mask_x = image.offset.x + i32::from(x);
            let mask_y = image.offset.y + i32::from(y);

            let track_depth = track_depth.get_depth(x.into(), y.into());
            let mask_depth = {
                let x = mask_x - mask_depth.offset.x;
                let y = mask_y - mask_depth.offset.y;
                if x >= 0
                    && x < mask_depth.width().try_into().unwrap()
                    && y >= 0
                    && y < mask_depth.height().try_into().unwrap()
                {
                    mask_depth.get_depth(usize::try_from(x).unwrap(), usize::try_from(y).unwrap())
                } else {
                    f32::INFINITY
                }
            };

            let masked = match operation {
                mask::Operation::Intersect => {
                    !view.sample_primary(mask_x, mask_y + y_offset, sprite.index) || track_depth < mask_depth
                }
                mask::Operation::Difference => {
                    !view.sample_secondary(mask_x, mask_y + y_offset, sprite.index) || track_depth >= mask_depth
                }
                mask::Operation::TransferNext => {
                    !(view.sample_primary(mask_x, mask_y + y_offset, sprite.index)
                        || view.sample_primary(mask_x, mask_y + y_offset, sprite.index + 1) && track_depth > mask_depth)
                }
            };

            if masked {
                image.set_pixel(x.into(), y.into(), 0);
            }
        }
    }
    image
}

fn get_coordinates(tile: mask::TileType, tiles: &[[i16; 3]]) -> &[i16; 3] {
    match tile {
        mask::TileType::Index(index) => tiles.get(index),
        mask::TileType::Last => tiles.last(),
    }
    .unwrap_or(&[0; 3])
}

fn calculate_tile_image_offset(coordinates: &[i16; 3], offset: &[i16; 3], rotation: usize) -> glam::IVec2 {
    let [x, y, z] = *coordinates;
    let (x, y) = match rotation {
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => (x, y),
    };
    let x = x + offset[0];
    let y = y + offset[1];
    let z = z + offset[2];

    let offset_x = x - y;
    let offset_y = (-(x + y) / 2) + z;
    glam::IVec2::new(offset_x.into(), offset_y.into())
}

pub fn split_image(
    image: &renderer::image::IndexedImage,
    view: &mask::View,
    tiles: &[[i16; 3]],
    rotation: usize,
    y_offset: i32,
) -> Vec<renderer::image::IndexedImage> {
    view.sprites
        .iter()
        .map(|sprite| {
            let mut split_image = split_sprite(view, sprite, image.clone(), y_offset);
            let coordinates = get_coordinates(sprite.tile, tiles);
            split_image.offset += calculate_tile_image_offset(coordinates, &sprite.offset, rotation);
            split_image.crop();
            split_image
        })
        .collect()
}

pub fn split_image_depth(
    image: &renderer::image::IndexedImage,
    view: &mask::View,
    tiles: &[[i16; 3]],
    rotation: usize,
    y_offset: i32,
    track_depth: &renderer::DepthBuffer,
    mask_depth: &renderer::DepthBuffer,
) -> Vec<renderer::image::IndexedImage> {
    view.sprites
        .iter()
        .map(|sprite| {
            let mut split_image = if let Some(operation) = sprite.operation {
                split_sprite_depth(
                    view,
                    sprite,
                    image.clone(),
                    y_offset,
                    track_depth,
                    mask_depth,
                    operation,
                )
            } else {
                split_sprite(view, sprite, image.clone(), y_offset)
            };
            let coordinates = get_coordinates(sprite.tile, tiles);
            split_image.offset += calculate_tile_image_offset(coordinates, &sprite.offset, rotation);
            split_image.crop();
            split_image
        })
        .collect()
}
