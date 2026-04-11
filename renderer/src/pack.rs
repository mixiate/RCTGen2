fn pack_rects_fixed(
    images: &[crate::image::IndexedImage],
    width: u16,
    height: u16,
    coords: &mut [glam::IVec2],
) -> bool {
    let mut rect_packer = rect_packer::DensePacker::new(width.into(), height.into());
    for (image, coord) in images.iter().zip(coords.iter_mut()) {
        if let Some(rect) = rect_packer.pack(image.width().into(), image.height().into(), false) {
            coord.x = rect.x;
            coord.y = rect.y;
        } else {
            return false;
        }
    }
    true
}

struct PackedRects {
    width: u16,
    height: u16,
    coords: Vec<glam::IVec2>,
}

fn pack_rects(images: &[crate::image::IndexedImage]) -> PackedRects {
    let mut size = 256;
    let mut coords = vec![glam::IVec2::new(0, 0); images.len()];

    while !pack_rects_fixed(images, size, size, &mut coords) {
        size *= 2;
    }
    let size = size;

    // Use binary search to find smallest square that can fit the images
    let mut lower_size = size / 2;
    let mut upper_size = size;
    while upper_size - lower_size > 2 {
        let mid_size = (upper_size + lower_size) / 2;
        if pack_rects_fixed(images, mid_size, mid_size, &mut coords) {
            upper_size = mid_size;
        } else {
            lower_size = mid_size;
        }
    }

    // Use binary search to determine how much the height can be reduced
    let mut upper_height = upper_size;
    let mut lower_height = 0;
    while upper_height - lower_height > 2 {
        let mid_height = (upper_height + lower_height) / 2;
        if pack_rects_fixed(images, upper_size, mid_height, &mut coords) {
            upper_height = mid_height;
        } else {
            lower_height = mid_height;
        }
    }

    // Use binary search to determine how much the width can be reduced
    let mut upper_width = upper_size;
    let mut lower_width = 0;
    while upper_width - lower_width > 2 {
        let mid_width = (upper_width + lower_width) / 2;
        if pack_rects_fixed(images, mid_width, upper_height, &mut coords) {
            upper_width = mid_width;
        } else {
            lower_width = mid_width;
        }
    }

    let (width, height) = if upper_width < upper_height {
        (upper_width, upper_size)
    } else {
        (upper_size, upper_height)
    };

    if !pack_rects_fixed(images, width, height, &mut coords) {
        panic!();
    }

    PackedRects { width, height, coords }
}

pub struct Atlas {
    pub image: crate::image::IndexedImage,
    pub coords: Vec<glam::IVec2>,
}

pub fn create_atlas(images: &[crate::image::IndexedImage]) -> Atlas {
    let PackedRects { width, height, coords } = pack_rects(images);

    let mut atlas_image = crate::image::IndexedImage::new(width, height);
    for (image, coord) in images.iter().zip(coords.iter()) {
        atlas_image.blit(image, coord.x.try_into().unwrap(), coord.y.try_into().unwrap());
    }

    Atlas {
        image: atlas_image,
        coords,
    }
}

pub fn create_grid(images: &[crate::image::IndexedImage], columns: i32) -> Atlas {
    let (x_min, y_min, x_max, y_max) = {
        let mut x_min = 0;
        let mut y_min = 0;
        let mut x_max = 0;
        let mut y_max = 0;

        for image in images {
            let x_neg = image.offset.x;
            let x_pos = i32::from(image.width()) + image.offset.x;
            if x_neg < x_min {
                x_min = x_neg;
            }
            if x_pos > x_max {
                x_max = x_pos;
            }
            let y_neg = image.offset.y;
            let y_pos = i32::from(image.height()) + image.offset.y;
            if y_neg < y_min {
                y_min = y_neg;
            }
            if y_pos > y_max {
                y_max = y_pos;
            }
        }
        (x_min, y_min, x_max, y_max)
    };

    let column_width = x_max - x_min;
    let row_height = y_max - y_min;

    let image_count = i32::try_from(images.len()).unwrap();
    let rows = image_count / columns;
    let rows = if image_count % columns != 0 { rows + 1 } else { rows };

    let width = u16::try_from(column_width * columns).unwrap();
    let height = u16::try_from(row_height * rows).unwrap();

    let mut grid_image = crate::image::IndexedImage::new(width, height);
    let mut coords = vec![glam::IVec2::new(0, 0); images.len()];
    for ((i, image), coord) in images.iter().enumerate().zip(coords.iter_mut()) {
        let i = i32::try_from(i).unwrap();
        let row = i / columns;
        let column = i - (row * columns);
        coord.x = (column_width * column) - x_min + image.offset.x;
        coord.y = (row_height * row) - y_min + image.offset.y;

        grid_image.blit(image, coord.x.try_into().unwrap(), coord.y.try_into().unwrap());
    }

    Atlas {
        image: grid_image,
        coords,
    }
}
