fn split_sprite(
    view: &crate::mask::View,
    sprite: &crate::mask::Sprite,
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

fn split_sprite_intersect(
    view: &crate::mask::View,
    sprite: &crate::mask::Sprite,
    mut image: renderer::image::IndexedImage,
    y_offset: i32,
    track_depth: &renderer::DepthBuffer,
    mask_depth: &renderer::DepthBuffer,
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

            if !view.sample_primary(mask_x, mask_y + y_offset, sprite.index) || track_depth < mask_depth {
                image.set_pixel(x.into(), y.into(), 0);
            }
        }
    }
    image
}

fn split_sprite_difference(
    view: &crate::mask::View,
    sprite: &crate::mask::Sprite,
    mut image: renderer::image::IndexedImage,
    y_offset: i32,
    track_depth: &renderer::DepthBuffer,
    mask_depth: &renderer::DepthBuffer,
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

            if !view.sample_secondary(mask_x, mask_y + y_offset, sprite.index) || track_depth >= mask_depth {
                image.set_pixel(x.into(), y.into(), 0);
            }
        }
    }
    image
}

fn split_sprite_transfer_next(
    view: &crate::mask::View,
    sprite: &crate::mask::Sprite,
    mut image: renderer::image::IndexedImage,
    y_offset: i32,
    track_depth: &renderer::DepthBuffer,
    mask_depth: &renderer::DepthBuffer,
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

            if !(view.sample_primary(mask_x, mask_y + y_offset, sprite.index)
                || view.sample_primary(mask_x, mask_y + y_offset, sprite.index + 1) && track_depth > mask_depth)
            {
                image.set_pixel(x.into(), y.into(), 0);
            }
        }
    }
    image
}

pub fn split_image(
    image: &renderer::image::IndexedImage,
    view: &crate::mask::View,
    y_offset: i32,
) -> Vec<renderer::image::IndexedImage> {
    view.sprites
        .iter()
        .map(|sprite| {
            let mut split_image = split_sprite(view, sprite, image.clone(), y_offset);
            split_image.offset += sprite.offset;
            split_image.crop();
            split_image
        })
        .collect()
}

pub fn split_image_depth(
    image: &renderer::image::IndexedImage,
    view: &crate::mask::View,
    y_offset: i32,
    track_depth: &renderer::DepthBuffer,
    mask_depth: &renderer::DepthBuffer,
) -> Vec<renderer::image::IndexedImage> {
    view.sprites
        .iter()
        .map(|sprite| {
            let mut split_image = match sprite.operation {
                None => split_sprite(view, sprite, image.clone(), y_offset),
                Some(crate::mask::Operation::Difference) => {
                    split_sprite_difference(view, sprite, image.clone(), y_offset, track_depth, mask_depth)
                }
                Some(crate::mask::Operation::Intersect) => {
                    split_sprite_intersect(view, sprite, image.clone(), y_offset, track_depth, mask_depth)
                }
                Some(crate::mask::Operation::TransferNext) => {
                    split_sprite_transfer_next(view, sprite, image.clone(), y_offset, track_depth, mask_depth)
                }
            };

            split_image.offset += sprite.offset;
            split_image.crop();

            split_image
        })
        .collect()
}
