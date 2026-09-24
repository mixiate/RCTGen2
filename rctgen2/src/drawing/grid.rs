use crate::drawing::blit;
use rct::world_coords::Coords;

pub struct GridImages {
    grid: renderer::image::IndexedImage,
    highlighted: renderer::image::IndexedImage,
}

impl GridImages {
    pub fn new() -> Self {
        let grid = include_bytes!("../../resources/tile_grid.png");
        let mut grid =
            renderer::image::IndexedImage::read(std::io::Cursor::new(grid), &renderer::palette::PALETTE_FLAT).unwrap();
        grid.offset = glam::IVec2::new(-32, 0);
        let highlighted = include_bytes!("../../resources/tile_grid_highlighted.png");
        let mut highlighted =
            renderer::image::IndexedImage::read(std::io::Cursor::new(highlighted), &renderer::palette::PALETTE_FLAT)
                .unwrap();
        highlighted.offset = glam::IVec2::new(-32, 0);
        GridImages { grid, highlighted }
    }
}

const HIGHLIGHTED_TILE_COLOURS: [openrct2::colour::Colour; 7] = [
    openrct2::colour::Colour::BrightRed,
    openrct2::colour::Colour::BrightGreen,
    openrct2::colour::Colour::DarkBlue,
    openrct2::colour::Colour::Yellow,
    openrct2::colour::Colour::DarkWater,
    openrct2::colour::Colour::BrightPink,
    openrct2::colour::Colour::White,
];

pub fn draw_grid(
    image: &mut renderer::image::Image,
    grid_images: &GridImages,
    dimensions: i16,
    z: i16,
    rotation: usize,
    highlighted_tiles: &[[i16; 3]],
) {
    for x in 0..dimensions {
        for y in 0..dimensions {
            let x = (x - (dimensions / 2)) * 32;
            let y = (y - (dimensions / 2)) * 32;
            let coords = Coords::new(x.into(), y.into(), z.into()).rotate(rotation);
            let (tile_image, colour) =
                if let Some(index) = highlighted_tiles.iter().position(|tile| tile[0] == x && tile[1] == y) {
                    let colour = *HIGHLIGHTED_TILE_COLOURS.get(index).unwrap_or(&openrct2::colour::Colour::Grey);
                    (&grid_images.highlighted, colour)
                } else {
                    (&grid_images.grid, openrct2::colour::Colour::White)
                };
            blit::draw_indexed_image(image, tile_image, &coords, colour, colour);
        }
    }
}
