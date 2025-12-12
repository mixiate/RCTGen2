fn pack_rects_fixed(
    images: &[crate::image::IndexedImage],
    width: usize,
    height: usize,
    coords: &mut [glam::IVec2],
) -> bool {
    let mut rect_packer = rect_packer::DensePacker::new(width.try_into().unwrap(), height.try_into().unwrap());
    for (image, coord) in images.iter().zip(coords.iter_mut()) {
        if let Some(rect) = rect_packer.pack(
            image.width().try_into().unwrap(),
            image.height().try_into().unwrap(),
            false,
        ) {
            coord.x = rect.x;
            coord.y = rect.y;
        } else {
            return false;
        }
    }
    true
}

struct PackedRects {
    width: usize,
    height: usize,
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
        let rect_x = usize::try_from(coord.x).unwrap();
        let rect_y = usize::try_from(coord.y).unwrap();
        for y in 0..image.height() {
            for x in 0..image.width() {
                atlas_image.set_pixel(rect_x + x, rect_y + y, image.get_pixel(x, y));
            }
        }
    }

    Atlas {
        image: atlas_image,
        coords,
    }
}
