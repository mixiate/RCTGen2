use crate::world_coords::Coords;

pub struct ScreenCoords {
    pub x: i32,
    pub y: i32,
}

impl std::convert::From<&Coords> for ScreenCoords {
    fn from(coords: &Coords) -> Self {
        let x = -(coords.x - coords.y);
        let y = -((-(coords.x + coords.y) / 2) + coords.z);
        ScreenCoords { x, y }
    }
}
