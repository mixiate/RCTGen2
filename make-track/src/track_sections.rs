pub struct TrackPoint {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub tangent: glam::Vec3,
    pub binormal: glam::Vec3,
}

pub struct TrackSection {
    pub name: &'static str,
    pub curve: fn(f32, f32) -> TrackPoint,
    pub length: f32,
}

pub const FLAT: TrackSection = TrackSection {
    name: "flat",
    curve: crate::track_curves::flat,
    length: crate::track_curves::FLAT_LENGTH,
};

pub const GENTLE: TrackSection = TrackSection {
    name: "gentle",
    curve: crate::track_curves::gentle,
    length: crate::track_curves::GENTLE_LENGTH,
};

pub const MEDIUM_TURN_LEFT: TrackSection = TrackSection {
    name: "medium_turn_left",
    curve: crate::track_curves::medium_turn_left,
    length: crate::track_curves::MEDIUM_TURN_LEFT_LENGTH,
};

pub const FLAT_TO_LEFT_BANK: TrackSection = TrackSection {
    name: "flat_to_left_bank",
    curve: crate::track_curves::flat_to_left_bank,
    length: crate::track_curves::FLAT_LENGTH,
};
