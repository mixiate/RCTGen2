mod blit;
pub mod grid;
mod supports_metal;
pub mod track;

pub use track::draw;

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
