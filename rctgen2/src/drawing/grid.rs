use crate::drawing::blit;
use rct::world_coords::Coords;

pub fn new_tile_grid_image() -> renderer::image::IndexedImage {
    let tile_grid_image = include_bytes!("../../resources/tile_grid.png");
    let mut tile_grid_image =
        renderer::image::IndexedImage::read(std::io::Cursor::new(tile_grid_image), &renderer::palette::PALETTE_FLAT)
            .unwrap();
    tile_grid_image.offset = glam::IVec2::new(-32, 0);
    tile_grid_image
}

pub fn draw_grid(
    image: &mut renderer::image::Image,
    tile_grid_image: &renderer::image::IndexedImage,
    dimensions: i16,
    z: i16,
    rotation: usize,
    highlighted_tiles: &[[i16; 3]],
) {
    for x in 0..dimensions {
        for y in 0..dimensions {
            let x = (x - (dimensions / 2)) * 32;
            let y = (y - (dimensions / 2)) * 32;
            let colour = if highlighted_tiles.iter().any(|tile| tile[0] == x && tile[1] == y) {
                openrct2::colour::Colour::BrightRed
            } else {
                openrct2::colour::Colour::White
            };
            let coords = Coords::new(x.into(), y.into(), z.into()).rotate(rotation);
            blit::draw_indexed_image(image, tile_grid_image, &coords, colour, colour);
        }
    }
}
