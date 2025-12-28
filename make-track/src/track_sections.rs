pub struct TrackPoint {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub tangent: glam::Vec3,
    pub binormal: glam::Vec3,
}

pub struct TrackSection {
    pub name: &'static str,
    curve: fn(f32, f32) -> TrackPoint,
    pub length: f32,
}

impl TrackSection {
    pub fn sample_curve(&self, distance: f32, bank_angle: f32) -> TrackPoint {
        if distance < 0.0 {
            let mut point = (self.curve)(0.0, bank_angle);
            point.position += point.tangent * distance;
            point
        } else if distance > self.length {
            let mut point = (self.curve)(self.length, bank_angle);
            point.position += point.tangent * (distance - self.length);
            point
        } else {
            (self.curve)(distance, bank_angle)
        }
    }
}

pub const FLAT: TrackSection = TrackSection {
    name: "flat",
    curve: crate::track_curves::flat,
    length: crate::track_curves::FLAT_LENGTH,
};

pub const FLAT_TO_GENTLE: TrackSection = TrackSection {
    name: "flat_to_gentle",
    curve: crate::track_curves::flat_to_gentle,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
};

pub const GENTLE: TrackSection = TrackSection {
    name: "gentle",
    curve: crate::track_curves::gentle,
    length: crate::track_curves::GENTLE_LENGTH,
};

pub const GENTLE_TO_FLAT: TrackSection = TrackSection {
    name: "gentle_to_flat",
    curve: crate::track_curves::gentle_to_flat,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
};

pub const GENTLE_TO_STEEP: TrackSection = TrackSection {
    name: "gentle_to_steep",
    curve: crate::track_curves::gentle_to_steep,
    length: crate::track_curves::GENTLE_TO_STEEP_LENGTH,
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

pub const FLAT_TO_RIGHT_BANK: TrackSection = TrackSection {
    name: "flat_to_right_bank",
    curve: crate::track_curves::flat_to_right_bank,
    length: crate::track_curves::FLAT_LENGTH,
};
