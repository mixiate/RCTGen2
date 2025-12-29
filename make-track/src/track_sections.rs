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

pub const STEEP_TO_GENTLE: TrackSection = TrackSection {
    name: "steep_to_gentle",
    curve: crate::track_curves::steep_to_gentle,
    length: crate::track_curves::GENTLE_TO_STEEP_LENGTH,
};

pub const STEEP: TrackSection = TrackSection {
    name: "steep",
    curve: crate::track_curves::steep,
    length: crate::track_curves::STEEP_LENGTH,
};

pub const STEEP_TO_VERTICAL: TrackSection = TrackSection {
    name: "steep_to_vertical",
    curve: crate::track_curves::steep_to_vertical,
    length: crate::track_curves::STEEP_TO_VERTICAL_LENGTH,
};

pub const VERTICAL_TO_STEEP: TrackSection = TrackSection {
    name: "vertical_to_steep",
    curve: crate::track_curves::vertical_to_steep,
    length: crate::track_curves::VERTICAL_TO_STEEP_LENGTH,
};

pub const VERTICAL: TrackSection = TrackSection {
    name: "vertical",
    curve: crate::track_curves::vertical,
    length: crate::track_curves::VERTICAL_LENGTH,
};

pub const SMALL_FLAT_TO_STEEP: TrackSection = TrackSection {
    name: "small_flat_to_steep",
    curve: crate::track_curves::small_flat_to_steep,
    length: crate::track_curves::SMALL_FLAT_TO_STEEP_LENGTH,
};

pub const SMALL_STEEP_TO_FLAT: TrackSection = TrackSection {
    name: "small_steep_to_flat",
    curve: crate::track_curves::small_steep_to_flat,
    length: crate::track_curves::SMALL_FLAT_TO_STEEP_LENGTH,
};

pub const FLAT_TO_STEEP: TrackSection = TrackSection {
    name: "flat_to_steep",
    curve: crate::track_curves::flat_to_steep,
    length: crate::track_curves::FLAT_TO_STEEP_LENGTH,
};

pub const STEEP_TO_FLAT: TrackSection = TrackSection {
    name: "steep_to_flat",
    curve: crate::track_curves::steep_to_flat,
    length: crate::track_curves::FLAT_TO_STEEP_LENGTH,
};

pub const SMALL_TURN_LEFT: TrackSection = TrackSection {
    name: "small_turn_left",
    curve: crate::track_curves::small_turn_left,
    length: crate::track_curves::SMALL_TURN_LEFT_LENGTH,
};

pub const MEDIUM_TURN_LEFT: TrackSection = TrackSection {
    name: "medium_turn_left",
    curve: crate::track_curves::medium_turn_left,
    length: crate::track_curves::MEDIUM_TURN_LEFT_LENGTH,
};

pub const LARGE_TURN_LEFT_TO_DIAG: TrackSection = TrackSection {
    name: "large_turn_left_to_diag",
    curve: crate::track_curves::large_turn_left_to_diag,
    length: crate::track_curves::LARGE_TURN_LEFT_TO_DIAG_LENGTH,
};

pub const LARGE_TURN_RIGHT_TO_DIAG: TrackSection = TrackSection {
    name: "large_turn_right_to_diag",
    curve: crate::track_curves::large_turn_right_to_diag,
    length: crate::track_curves::LARGE_TURN_LEFT_TO_DIAG_LENGTH,
};

pub const FLAT_DIAG: TrackSection = TrackSection {
    name: "flat_diag",
    curve: crate::track_curves::flat_diag,
    length: crate::track_curves::FLAT_DIAG_LENGTH,
};

pub const FLAT_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "flat_to_gentle_diag",
    curve: crate::track_curves::flat_to_gentle_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
};

pub const GENTLE_TO_FLAT_DIAG: TrackSection = TrackSection {
    name: "gentle_to_flat_diag",
    curve: crate::track_curves::gentle_to_flat_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
};

pub const GENTLE_DIAG: TrackSection = TrackSection {
    name: "gentle_diag",
    curve: crate::track_curves::gentle_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
};

pub const GENTLE_TO_STEEP_DIAG: TrackSection = TrackSection {
    name: "gentle_to_steep_diag",
    curve: crate::track_curves::gentle_to_steep_diag,
    length: crate::track_curves::GENTLE_TO_STEEP_DIAG_LENGTH,
};

pub const STEEP_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "steep_to_gentle_diag",
    curve: crate::track_curves::steep_to_gentle_diag,
    length: crate::track_curves::GENTLE_TO_STEEP_DIAG_LENGTH,
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

pub const LEFT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "left_bank_to_gentle",
    curve: crate::track_curves::left_bank_to_gentle,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
};

pub const RIGHT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "right_bank_to_gentle",
    curve: crate::track_curves::right_bank_to_gentle,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
};

pub const GENTLE_TO_LEFT_BANK: TrackSection = TrackSection {
    name: "gentle_to_left_bank",
    curve: crate::track_curves::gentle_to_left_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
};

pub const GENTLE_TO_RIGHT_BANK: TrackSection = TrackSection {
    name: "gentle_to_right_bank",
    curve: crate::track_curves::gentle_to_right_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
};

pub const LEFT_BANK: TrackSection = TrackSection {
    name: "left_bank",
    curve: crate::track_curves::left_bank,
    length: crate::track_curves::FLAT_LENGTH,
};

pub const SMALL_TURN_LEFT_BANK: TrackSection = TrackSection {
    name: "small_turn_left_bank",
    curve: crate::track_curves::small_turn_left_bank,
    length: crate::track_curves::SMALL_TURN_LEFT_LENGTH,
};

pub const MEDIUM_TURN_LEFT_BANK: TrackSection = TrackSection {
    name: "medium_turn_left_bank",
    curve: crate::track_curves::medium_turn_left_bank,
    length: crate::track_curves::MEDIUM_TURN_LEFT_LENGTH,
};

pub const LARGE_TURN_LEFT_TO_DIAG_BANK: TrackSection = TrackSection {
    name: "large_turn_left_to_diag_bank",
    curve: crate::track_curves::large_turn_left_to_diag_bank,
    length: crate::track_curves::LARGE_TURN_LEFT_TO_DIAG_LENGTH,
};

pub const LARGE_TURN_RIGHT_TO_DIAG_BANK: TrackSection = TrackSection {
    name: "large_turn_right_to_diag_bank",
    curve: crate::track_curves::large_turn_right_to_diag_bank,
    length: crate::track_curves::LARGE_TURN_LEFT_TO_DIAG_LENGTH,
};
