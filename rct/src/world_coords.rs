#[derive(Clone, Copy)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coords {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Coords { x, y, z }
    }

    pub fn rotate(self, rotation: usize) -> Self {
        match rotation {
            1 => Coords::new(self.y, -self.x, self.z),
            2 => Coords::new(-self.x, -self.y, self.z),
            3 => Coords::new(-self.y, self.x, self.z),
            _ => Coords::new(self.x, self.y, self.z),
        }
    }
}

impl std::convert::From<&[i16; 3]> for Coords {
    fn from(coords: &[i16; 3]) -> Self {
        Coords::new(coords[0].into(), coords[1].into(), coords[2].into())
    }
}

impl std::convert::From<[i16; 3]> for Coords {
    fn from(coords: [i16; 3]) -> Self {
        Coords::from(&coords)
    }
}

impl std::ops::Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coords::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
