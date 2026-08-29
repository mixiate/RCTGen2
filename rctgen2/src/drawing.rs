mod blit;
mod supports_metal;
mod track;

pub use track::draw;

pub struct Options {
    pub indexed: bool,
    pub adjacent_track: bool,
    pub original_track: bool,
    pub supports: bool,
    pub colour_1: openrct2::colour::Colour,
    pub colour_2: openrct2::colour::Colour,
    pub colour_3: openrct2::colour::Colour,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            indexed: true,
            adjacent_track: false,
            original_track: false,
            supports: false,
            colour_1: openrct2::colour::Colour::LightBlue,
            colour_2: openrct2::colour::Colour::BrightPink,
            colour_3: openrct2::colour::Colour::Yellow,
        }
    }
}

pub fn add_coords(a: &[i16; 3], b: &[i16; 3]) -> [i16; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn rotate_coords(coordinates: &[i16; 3], rotation: usize) -> [i16; 3] {
    let [x, y, z] = *coordinates;
    match rotation {
        1 => [y, -x, z],
        2 => [-x, -y, z],
        3 => [-y, x, z],
        _ => [x, y, z],
    }
}
