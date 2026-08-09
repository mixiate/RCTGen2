mod blit;
mod track;

pub use track::draw;

pub struct Options {
    pub indexed: bool,
    pub adjacent_track: bool,
    pub original_track: bool,
    pub colour_1: openrct2::colour::Colour,
    pub colour_2: openrct2::colour::Colour,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            indexed: true,
            adjacent_track: false,
            original_track: false,
            colour_1: openrct2::colour::Colour::LightBlue,
            colour_2: openrct2::colour::Colour::BrightPink,
        }
    }
}
