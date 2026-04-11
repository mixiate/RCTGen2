#[derive(Debug, Clone, Copy)]
pub enum ChainType {
    Flat,
    Gentle,
    FlatDiag,
}

const CHAIN_PALETTE_INDEX_START: u8 = 1;

pub fn apply_chain(image: &mut renderer::image::IndexedImage, chain_type: ChainType, view: usize) {
    let chain_image = &match chain_type {
        ChainType::Flat => &FLAT_CHAIN_IMAGES,
        ChainType::Gentle => &GENTLE_CHAIN_IMAGES,
        ChainType::FlatDiag => &FLAT_DIAG_CHAIN_IMAGES,
    }[view];

    for y in 0..image.height() {
        for x in 0..image.width() {
            if image.get_pixel(x.into(), y.into()) == CHAIN_PALETTE_INDEX_START {
                let chain_x = i32::from(x);
                let chain_y = i32::from(y);
                let chain_x = (chain_x + image.offset.x - chain_image.x_offset).rem_euclid(chain_image.width);
                let chain_y = (chain_y + image.offset.y - chain_image.y_offset).rem_euclid(chain_image.height);

                let chain_index = usize::try_from(chain_x + chain_y * chain_image.width).unwrap();

                image.set_pixel(x.into(), y.into(), chain_image.pixels[chain_index]);
            }
        }
    }
}

struct ChainImage {
    pixels: [u8; 18],
    width: i32,
    height: i32,
    x_offset: i32,
    y_offset: i32,
}

const FLAT_CHAIN_IMAGES: [ChainImage; 4] = [
    ChainImage {
        pixels: [1, 2, 3, 1, 2, 3, 3, 1, 2, 3, 1, 2, 2, 3, 1, 2, 3, 1],
        width: 3,
        height: 6,
        x_offset: 0,
        y_offset: -2,
    },
    ChainImage {
        pixels: [1, 2, 3, 1, 2, 3, 2, 3, 1, 2, 3, 1, 3, 1, 2, 3, 1, 2],
        width: 3,
        height: 6,
        x_offset: 0,
        y_offset: -1,
    },
    ChainImage {
        pixels: [1, 3, 2, 1, 3, 2, 2, 1, 3, 2, 1, 3, 3, 2, 1, 3, 2, 1],
        width: 3,
        height: 6,
        x_offset: 0,
        y_offset: 0,
    },
    ChainImage {
        pixels: [1, 3, 2, 1, 3, 2, 3, 2, 1, 3, 2, 1, 2, 1, 3, 2, 1, 3],
        width: 3,
        height: 6,
        x_offset: -1,
        y_offset: 0,
    },
];

const GENTLE_CHAIN_IMAGES: [ChainImage; 4] = [
    ChainImage {
        pixels: [1, 1, 2, 2, 3, 3, 3, 3, 1, 1, 2, 2, 2, 2, 3, 3, 1, 1],
        width: 6,
        height: 3,
        x_offset: -3,
        y_offset: -1,
    },
    ChainImage {
        pixels: [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 3,
        height: 1,
        x_offset: 1,
        y_offset: 0,
    },
    ChainImage {
        pixels: [3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 3,
        height: 1,
        x_offset: 1,
        y_offset: 0,
    },
    ChainImage {
        pixels: [2, 2, 1, 1, 3, 3, 1, 1, 3, 3, 2, 2, 3, 3, 2, 2, 1, 1],
        width: 6,
        height: 3,
        x_offset: 0,
        y_offset: -1,
    },
];

const FLAT_DIAG_CHAIN_IMAGES: [ChainImage; 4] = [
    ChainImage {
        pixels: [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 3,
        height: 1,
        x_offset: -2,
        y_offset: 0,
    },
    ChainImage {
        pixels: [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 1,
        height: 3,
        x_offset: 0,
        y_offset: -2,
    },
    ChainImage {
        pixels: [3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 3,
        height: 1,
        x_offset: -1,
        y_offset: 0,
    },
    ChainImage {
        pixels: [3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 1,
        height: 3,
        x_offset: 0,
        y_offset: -1,
    },
];
