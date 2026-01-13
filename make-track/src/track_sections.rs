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
    position_offset: glam::Vec3,
    pub mask_offset_y: bool,
}

impl TrackSection {
    pub fn sample_curve(
        &self,
        distance: f32,
        bank_angle: f32,
        offset_start: &glam::Vec3,
        offset_end: &glam::Vec3,
    ) -> TrackPoint {
        let mut track_point = if distance < 0.0 {
            let mut point = (self.curve)(0.0, bank_angle);
            point.position += point.tangent * distance;
            point
        } else if distance > self.length {
            let mut point = (self.curve)(self.length, bank_angle);
            point.position += point.tangent * (distance - self.length);
            point
        } else {
            (self.curve)(distance, bank_angle)
        };

        track_point.position += self.position_offset;

        let v = (distance / self.length).clamp(0.0, 1.0);
        track_point.position += offset_start * (2.0 * v * v * v - 3.0 * v * v + 1.0);
        track_point.position += offset_end * (-2.0 * v * v * v + 3.0 * v * v);

        track_point
    }
}

const POSITION_OFFSET_NONE: glam::Vec3 = glam::Vec3::new(0.0, 0.0, 0.0);
const POSITION_OFFSET_ORTHOGONAL: glam::Vec3 = glam::Vec3::new(0.0, 0.0, -0.5);
const POSITION_OFFSET_DIAGONAL: glam::Vec3 = glam::Vec3::new(-0.5, 0.0, -0.5);

pub const FLAT: TrackSection = TrackSection {
    name: "flat",
    curve: crate::track_curves::flat,
    length: crate::track_curves::FLAT_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_GENTLE: TrackSection = TrackSection {
    name: "flat_to_gentle",
    curve: crate::track_curves::flat_to_gentle,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE: TrackSection = TrackSection {
    name: "gentle",
    curve: crate::track_curves::gentle,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_FLAT: TrackSection = TrackSection {
    name: "gentle_to_flat",
    curve: crate::track_curves::gentle_to_flat,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_STEEP: TrackSection = TrackSection {
    name: "gentle_to_steep",
    curve: crate::track_curves::gentle_to_steep,
    length: crate::track_curves::GENTLE_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const STEEP_TO_GENTLE: TrackSection = TrackSection {
    name: "steep_to_gentle",
    curve: crate::track_curves::steep_to_gentle,
    length: crate::track_curves::GENTLE_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const STEEP: TrackSection = TrackSection {
    name: "steep",
    curve: crate::track_curves::steep,
    length: crate::track_curves::STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const STEEP_TO_VERTICAL: TrackSection = TrackSection {
    name: "steep_to_vertical",
    curve: crate::track_curves::steep_to_vertical,
    length: crate::track_curves::STEEP_TO_VERTICAL_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const VERTICAL_TO_STEEP: TrackSection = TrackSection {
    name: "vertical_to_steep",
    curve: crate::track_curves::vertical_to_steep,
    length: crate::track_curves::VERTICAL_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const VERTICAL: TrackSection = TrackSection {
    name: "vertical",
    curve: crate::track_curves::vertical,
    length: crate::track_curves::VERTICAL_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const SMALL_FLAT_TO_STEEP: TrackSection = TrackSection {
    name: "small_flat_to_steep",
    curve: crate::track_curves::small_flat_to_steep,
    length: crate::track_curves::SMALL_FLAT_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const SMALL_STEEP_TO_FLAT: TrackSection = TrackSection {
    name: "small_steep_to_flat",
    curve: crate::track_curves::small_steep_to_flat,
    length: crate::track_curves::SMALL_FLAT_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_STEEP: TrackSection = TrackSection {
    name: "flat_to_steep",
    curve: crate::track_curves::flat_to_steep,
    length: crate::track_curves::FLAT_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const STEEP_TO_FLAT: TrackSection = TrackSection {
    name: "steep_to_flat",
    curve: crate::track_curves::steep_to_flat,
    length: crate::track_curves::FLAT_TO_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const SMALL_TURN_LEFT: TrackSection = TrackSection {
    name: "small_turn_left",
    curve: crate::track_curves::small_turn_left,
    length: crate::track_curves::SMALL_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const MEDIUM_TURN_LEFT: TrackSection = TrackSection {
    name: "medium_turn_left",
    curve: crate::track_curves::medium_turn_left,
    length: crate::track_curves::MEDIUM_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_LEFT_TO_DIAG: TrackSection = TrackSection {
    name: "large_turn_left_to_diag",
    curve: crate::track_curves::large_turn_left_to_diag,
    length: crate::track_curves::LARGE_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_RIGHT_TO_DIAG: TrackSection = TrackSection {
    name: "large_turn_right_to_diag",
    curve: crate::track_curves::large_turn_right_to_diag,
    length: crate::track_curves::LARGE_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_DIAG: TrackSection = TrackSection {
    name: "flat_diag",
    curve: crate::track_curves::flat_diag,
    length: crate::track_curves::FLAT_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "flat_to_gentle_diag",
    curve: crate::track_curves::flat_to_gentle_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_FLAT_DIAG: TrackSection = TrackSection {
    name: "gentle_to_flat_diag",
    curve: crate::track_curves::gentle_to_flat_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_DIAG: TrackSection = TrackSection {
    name: "gentle_diag",
    curve: crate::track_curves::gentle_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_STEEP_DIAG: TrackSection = TrackSection {
    name: "gentle_to_steep_diag",
    curve: crate::track_curves::gentle_to_steep_diag,
    length: crate::track_curves::GENTLE_TO_STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const STEEP_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "steep_to_gentle_diag",
    curve: crate::track_curves::steep_to_gentle_diag,
    length: crate::track_curves::GENTLE_TO_STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const STEEP_DIAG: TrackSection = TrackSection {
    name: "steep_diag",
    curve: crate::track_curves::steep_diag,
    length: crate::track_curves::STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const SMALL_FLAT_TO_STEEP_DIAG: TrackSection = TrackSection {
    name: "small_flat_to_steep_diag",
    curve: crate::track_curves::small_flat_to_steep_diag,
    length: crate::track_curves::SMALL_FLAT_TO_STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const SMALL_STEEP_TO_FLAT_DIAG: TrackSection = TrackSection {
    name: "small_steep_to_flat_diag",
    curve: crate::track_curves::small_steep_to_flat_diag,
    length: crate::track_curves::SMALL_FLAT_TO_STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_STEEP_DIAG: TrackSection = TrackSection {
    name: "flat_to_steep_diag",
    curve: crate::track_curves::flat_to_steep_diag,
    length: crate::track_curves::FLAT_TO_STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const STEEP_TO_FLAT_DIAG: TrackSection = TrackSection {
    name: "steep_to_flat_diag",
    curve: crate::track_curves::steep_to_flat_diag,
    length: crate::track_curves::FLAT_TO_STEEP_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_LEFT_BANK: TrackSection = TrackSection {
    name: "flat_to_left_bank",
    curve: crate::track_curves::flat_to_left_bank,
    length: crate::track_curves::FLAT_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_RIGHT_BANK: TrackSection = TrackSection {
    name: "flat_to_right_bank",
    curve: crate::track_curves::flat_to_right_bank,
    length: crate::track_curves::FLAT_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LEFT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "left_bank_to_gentle",
    curve: crate::track_curves::left_bank_to_gentle,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const RIGHT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "right_bank_to_gentle",
    curve: crate::track_curves::right_bank_to_gentle,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_LEFT_BANK: TrackSection = TrackSection {
    name: "gentle_to_left_bank",
    curve: crate::track_curves::gentle_to_left_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_RIGHT_BANK: TrackSection = TrackSection {
    name: "gentle_to_right_bank",
    curve: crate::track_curves::gentle_to_right_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LEFT_BANK: TrackSection = TrackSection {
    name: "left_bank",
    curve: crate::track_curves::left_bank,
    length: crate::track_curves::FLAT_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const SMALL_TURN_LEFT_BANK: TrackSection = TrackSection {
    name: "small_turn_left_bank",
    curve: crate::track_curves::small_turn_left_bank,
    length: crate::track_curves::SMALL_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const MEDIUM_TURN_LEFT_BANK: TrackSection = TrackSection {
    name: "medium_turn_left_bank",
    curve: crate::track_curves::medium_turn_left_bank,
    length: crate::track_curves::MEDIUM_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_LEFT_TO_DIAG_BANK: TrackSection = TrackSection {
    name: "large_turn_left_to_diag_bank",
    curve: crate::track_curves::large_turn_left_to_diag_bank,
    length: crate::track_curves::LARGE_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_RIGHT_TO_DIAG_BANK: TrackSection = TrackSection {
    name: "large_turn_right_to_diag_bank",
    curve: crate::track_curves::large_turn_right_to_diag_bank,
    length: crate::track_curves::LARGE_TURN_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "flat_to_left_bank_diag",
    curve: crate::track_curves::flat_to_left_bank_diag,
    length: crate::track_curves::FLAT_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "flat_to_right_bank_diag",
    curve: crate::track_curves::flat_to_right_bank_diag,
    length: crate::track_curves::FLAT_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const LEFT_BANK_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "left_bank_to_gentle_diag",
    curve: crate::track_curves::left_bank_to_gentle_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const RIGHT_BANK_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "right_bank_to_gentle_diag",
    curve: crate::track_curves::right_bank_to_gentle_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_to_left_bank_diag",
    curve: crate::track_curves::gentle_to_left_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_to_right_bank_diag",
    curve: crate::track_curves::gentle_to_right_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "left_bank_diag",
    curve: crate::track_curves::left_bank_diag,
    length: crate::track_curves::FLAT_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const SMALL_TURN_LEFT_GENTLE: TrackSection = TrackSection {
    name: "small_turn_left_gentle",
    curve: crate::track_curves::small_turn_left_gentle,
    length: crate::track_curves::SMALL_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const SMALL_TURN_RIGHT_GENTLE: TrackSection = TrackSection {
    name: "small_turn_right_gentle",
    curve: crate::track_curves::small_turn_right_gentle,
    length: crate::track_curves::SMALL_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const MEDIUM_TURN_LEFT_GENTLE: TrackSection = TrackSection {
    name: "medium_turn_left_gentle",
    curve: crate::track_curves::medium_turn_left_gentle,
    length: crate::track_curves::MEDIUM_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const MEDIUM_TURN_RIGHT_GENTLE: TrackSection = TrackSection {
    name: "medium_turn_right_gentle",
    curve: crate::track_curves::medium_turn_right_gentle,
    length: crate::track_curves::MEDIUM_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_TURN_LEFT_TO_DIAG_GENTLE: TrackSection = TrackSection {
    name: "large_turn_left_to_diag_gentle",
    curve: crate::track_curves::large_turn_left_to_diag_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_RIGHT_TO_DIAG_GENTLE: TrackSection = TrackSection {
    name: "large_turn_right_to_diag_gentle",
    curve: crate::track_curves::large_turn_right_to_diag_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_LEFT_TO_ORTHOGONAL_GENTLE: TrackSection = TrackSection {
    name: "large_turn_left_to_orthogonal_gentle",
    curve: crate::track_curves::large_turn_left_to_orthogonal_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const LARGE_TURN_RIGHT_TO_ORTHOGONAL_GENTLE: TrackSection = TrackSection {
    name: "large_turn_right_to_orthogonal_gentle",
    curve: crate::track_curves::large_turn_right_to_orthogonal_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const VERY_SMALL_TURN_LEFT_STEEP: TrackSection = TrackSection {
    name: "very_small_turn_left_steep",
    curve: crate::track_curves::very_small_turn_left_steep,
    length: crate::track_curves::VERY_SMALL_TURN_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const VERY_SMALL_TURN_RIGHT_STEEP: TrackSection = TrackSection {
    name: "very_small_turn_right_steep",
    curve: crate::track_curves::very_small_turn_right_steep,
    length: crate::track_curves::VERY_SMALL_TURN_STEEP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const VERTICAL_TWIST_LEFT: TrackSection = TrackSection {
    name: "vertical_twist_left",
    curve: crate::track_curves::vertical_twist_left,
    length: crate::track_curves::VERTICAL_TWIST_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: true,
};

pub const VERTICAL_TWIST_RIGHT: TrackSection = TrackSection {
    name: "vertical_twist_right",
    curve: crate::track_curves::vertical_twist_right,
    length: crate::track_curves::VERTICAL_TWIST_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: true,
};

pub const GENTLE_TO_GENTLE_LEFT_BANK: TrackSection = TrackSection {
    name: "gentle_to_gentle_left_bank",
    curve: crate::track_curves::gentle_to_gentle_left_bank,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_GENTLE_RIGHT_BANK: TrackSection = TrackSection {
    name: "gentle_to_gentle_right_bank",
    curve: crate::track_curves::gentle_to_gentle_right_bank,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "gentle_left_bank_to_gentle",
    curve: crate::track_curves::gentle_left_bank_to_gentle,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const GENTLE_RIGHT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "gentle_right_bank_to_gentle",
    curve: crate::track_curves::gentle_right_bank_to_gentle,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LEFT_BANK_TO_GENTLE_LEFT_BANK: TrackSection = TrackSection {
    name: "left_bank_to_gentle_left_bank",
    curve: crate::track_curves::left_bank_to_gentle_left_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const RIGHT_BANK_TO_GENTLE_RIGHT_BANK: TrackSection = TrackSection {
    name: "right_bank_to_gentle_right_bank",
    curve: crate::track_curves::right_bank_to_gentle_right_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_TO_LEFT_BANK: TrackSection = TrackSection {
    name: "gentle_left_bank_to_left_bank",
    curve: crate::track_curves::gentle_left_bank_to_left_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_RIGHT_BANK_TO_RIGHT_BANK: TrackSection = TrackSection {
    name: "gentle_right_bank_to_right_bank",
    curve: crate::track_curves::gentle_right_bank_to_right_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK: TrackSection = TrackSection {
    name: "gentle_left_bank",
    curve: crate::track_curves::gentle_left_bank,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_RIGHT_BANK: TrackSection = TrackSection {
    name: "gentle_right_bank",
    curve: crate::track_curves::gentle_right_bank,
    length: crate::track_curves::GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_GENTLE_LEFT_BANK: TrackSection = TrackSection {
    name: "flat_to_gentle_left_bank",
    curve: crate::track_curves::flat_to_gentle_left_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_GENTLE_RIGHT_BANK: TrackSection = TrackSection {
    name: "flat_to_gentle_right_bank",
    curve: crate::track_curves::flat_to_gentle_right_bank,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_TO_FLAT: TrackSection = TrackSection {
    name: "gentle_left_bank_to_flat",
    curve: crate::track_curves::gentle_left_bank_to_flat,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const GENTLE_RIGHT_BANK_TO_FLAT: TrackSection = TrackSection {
    name: "gentle_right_bank_to_flat",
    curve: crate::track_curves::gentle_right_bank_to_flat,
    length: crate::track_curves::FLAT_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const GENTLE_TO_GENTLE_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_to_gentle_left_bank_diag",
    curve: crate::track_curves::gentle_to_gentle_left_bank_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_TO_GENTLE_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_to_gentle_right_bank_diag",
    curve: crate::track_curves::gentle_to_gentle_right_bank_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "gentle_left_bank_to_gentle_diag",
    curve: crate::track_curves::gentle_left_bank_to_gentle_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_RIGHT_BANK_TO_GENTLE_DIAG: TrackSection = TrackSection {
    name: "gentle_right_bank_to_gentle_diag",
    curve: crate::track_curves::gentle_right_bank_to_gentle_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const LEFT_BANK_TO_GENTLE_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "left_bank_to_gentle_left_bank_diag",
    curve: crate::track_curves::left_bank_to_gentle_left_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const RIGHT_BANK_TO_GENTLE_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "right_bank_to_gentle_right_bank_diag",
    curve: crate::track_curves::right_bank_to_gentle_right_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_TO_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_left_bank_to_left_bank_diag",
    curve: crate::track_curves::gentle_left_bank_to_left_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_RIGHT_BANK_TO_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_right_bank_to_right_bank_diag",
    curve: crate::track_curves::gentle_right_bank_to_right_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_left_bank_diag",
    curve: crate::track_curves::gentle_left_bank_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "gentle_right_bank_diag",
    curve: crate::track_curves::gentle_right_bank_diag,
    length: crate::track_curves::GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_GENTLE_LEFT_BANK_DIAG: TrackSection = TrackSection {
    name: "flat_to_gentle_left_bank_diag",
    curve: crate::track_curves::flat_to_gentle_left_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const FLAT_TO_GENTLE_RIGHT_BANK_DIAG: TrackSection = TrackSection {
    name: "flat_to_gentle_right_bank_diag",
    curve: crate::track_curves::flat_to_gentle_right_bank_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_LEFT_BANK_TO_FLAT_DIAG: TrackSection = TrackSection {
    name: "gentle_left_bank_to_flat_diag",
    curve: crate::track_curves::gentle_left_bank_to_flat_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const GENTLE_RIGHT_BANK_TO_FLAT_DIAG: TrackSection = TrackSection {
    name: "gentle_right_bank_to_flat_diag",
    curve: crate::track_curves::gentle_right_bank_to_flat_diag,
    length: crate::track_curves::FLAT_TO_GENTLE_DIAG_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const SMALL_TURN_LEFT_BANK_GENTLE: TrackSection = TrackSection {
    name: "small_turn_left_bank_gentle",
    curve: crate::track_curves::small_turn_left_bank_gentle,
    length: crate::track_curves::SMALL_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const SMALL_TURN_RIGHT_BANK_GENTLE: TrackSection = TrackSection {
    name: "small_turn_right_bank_gentle",
    curve: crate::track_curves::small_turn_right_bank_gentle,
    length: crate::track_curves::SMALL_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const MEDIUM_TURN_LEFT_BANK_GENTLE: TrackSection = TrackSection {
    name: "medium_turn_left_bank_gentle",
    curve: crate::track_curves::medium_turn_left_bank_gentle,
    length: crate::track_curves::MEDIUM_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const MEDIUM_TURN_RIGHT_BANK_GENTLE: TrackSection = TrackSection {
    name: "medium_turn_right_bank_gentle",
    curve: crate::track_curves::medium_turn_right_bank_gentle,
    length: crate::track_curves::MEDIUM_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_TURN_LEFT_BANK_TO_DIAG_GENTLE: TrackSection = TrackSection {
    name: "large_turn_left_bank_to_diag_gentle",
    curve: crate::track_curves::large_turn_left_bank_to_diag_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_RIGHT_BANK_TO_DIAG_GENTLE: TrackSection = TrackSection {
    name: "large_turn_right_bank_to_diag_gentle",
    curve: crate::track_curves::large_turn_right_bank_to_diag_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const LARGE_TURN_LEFT_BANK_TO_ORTHOGONAL_GENTLE: TrackSection = TrackSection {
    name: "large_turn_left_bank_to_orthogonal_gentle",
    curve: crate::track_curves::large_turn_left_bank_to_orthogonal_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const LARGE_TURN_RIGHT_BANK_TO_ORTHOGONAL_GENTLE: TrackSection = TrackSection {
    name: "large_turn_right_bank_to_orthogonal_gentle",
    curve: crate::track_curves::large_turn_right_bank_to_orthogonal_gentle,
    length: crate::track_curves::LARGE_TURN_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const S_BEND_LEFT: TrackSection = TrackSection {
    name: "s_bend_left",
    curve: crate::track_curves::s_bend_left,
    length: crate::track_curves::S_BEND_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const S_BEND_RIGHT: TrackSection = TrackSection {
    name: "s_bend_right",
    curve: crate::track_curves::s_bend_right,
    length: crate::track_curves::S_BEND_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const SMALL_HELIX_LEFT: TrackSection = TrackSection {
    name: "small_helix_left",
    curve: crate::track_curves::small_helix_left,
    length: crate::track_curves::SMALL_HELIX_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const SMALL_HELIX_RIGHT: TrackSection = TrackSection {
    name: "small_helix_right",
    curve: crate::track_curves::small_helix_right,
    length: crate::track_curves::SMALL_HELIX_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const MEDIUM_HELIX_LEFT: TrackSection = TrackSection {
    name: "medium_helix_left",
    curve: crate::track_curves::medium_helix_left,
    length: crate::track_curves::MEDIUM_HELIX_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const MEDIUM_HELIX_RIGHT: TrackSection = TrackSection {
    name: "medium_helix_right",
    curve: crate::track_curves::medium_helix_right,
    length: crate::track_curves::MEDIUM_HELIX_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const SMALL_TURN_LEFT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "small_turn_left_bank_to_gentle",
    curve: crate::track_curves::small_turn_left_bank_to_gentle,
    length: crate::track_curves::SMALL_TURN_BANK_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const SMALL_TURN_RIGHT_BANK_TO_GENTLE: TrackSection = TrackSection {
    name: "small_turn_right_bank_to_gentle",
    curve: crate::track_curves::small_turn_right_bank_to_gentle,
    length: crate::track_curves::SMALL_TURN_BANK_TO_GENTLE_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const BARREL_ROLL_LEFT: TrackSection = TrackSection {
    name: "barrel_roll_left",
    curve: crate::track_curves::barrel_roll_left,
    length: crate::track_curves::BARREL_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const BARREL_ROLL_RIGHT: TrackSection = TrackSection {
    name: "barrel_roll_right",
    curve: crate::track_curves::barrel_roll_right,
    length: crate::track_curves::BARREL_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const INLINE_TWIST_LEFT: TrackSection = TrackSection {
    name: "inline_twist_left",
    curve: crate::track_curves::inline_twist_left,
    length: crate::track_curves::BARREL_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const INLINE_TWIST_RIGHT: TrackSection = TrackSection {
    name: "inline_twist_right",
    curve: crate::track_curves::inline_twist_right,
    length: crate::track_curves::BARREL_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const HALF_LOOP: TrackSection = TrackSection {
    name: "half_loop",
    curve: crate::track_curves::half_loop,
    length: crate::track_curves::HALF_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const VERTICAL_LOOP_LEFT: TrackSection = TrackSection {
    name: "vertical_loop_left",
    curve: crate::track_curves::vertical_loop_left,
    length: crate::track_curves::VERTICAL_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const VERTICAL_LOOP_RIGHT: TrackSection = TrackSection {
    name: "vertical_loop_right",
    curve: crate::track_curves::vertical_loop_right,
    length: crate::track_curves::VERTICAL_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: false,
};

pub const QUARTER_LOOP: TrackSection = TrackSection {
    name: "quarter_loop",
    curve: crate::track_curves::quarter_loop,
    length: crate::track_curves::QUARTER_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_NONE,
    mask_offset_y: false,
};

pub const CORKSCREW_LEFT: TrackSection = TrackSection {
    name: "corkscrew_left",
    curve: crate::track_curves::corkscrew_left,
    length: crate::track_curves::CORKSCREW_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const CORKSCREW_RIGHT: TrackSection = TrackSection {
    name: "corkscrew_right",
    curve: crate::track_curves::corkscrew_right,
    length: crate::track_curves::CORKSCREW_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_CORKSCREW_LEFT: TrackSection = TrackSection {
    name: "large_corkscrew_left",
    curve: crate::track_curves::large_corkscrew_left,
    length: crate::track_curves::LARGE_CORKSCREW_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_CORKSCREW_RIGHT: TrackSection = TrackSection {
    name: "large_corkscrew_right",
    curve: crate::track_curves::large_corkscrew_right,
    length: crate::track_curves::LARGE_CORKSCREW_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const MEDIUM_HALF_LOOP_LEFT: TrackSection = TrackSection {
    name: "medium_half_loop_left",
    curve: crate::track_curves::medium_half_loop_left,
    length: crate::track_curves::MEDIUM_HALF_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const MEDIUM_HALF_LOOP_RIGHT: TrackSection = TrackSection {
    name: "medium_half_loop_right",
    curve: crate::track_curves::medium_half_loop_right,
    length: crate::track_curves::MEDIUM_HALF_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_HALF_LOOP_LEFT: TrackSection = TrackSection {
    name: "large_half_loop_left",
    curve: crate::track_curves::large_half_loop_left,
    length: crate::track_curves::LARGE_HALF_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_HALF_LOOP_RIGHT: TrackSection = TrackSection {
    name: "large_half_loop_right",
    curve: crate::track_curves::large_half_loop_right,
    length: crate::track_curves::LARGE_HALF_LOOP_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const ZERO_G_ROLL_LEFT: TrackSection = TrackSection {
    name: "zero_g_roll_left",
    curve: crate::track_curves::zero_g_roll_left,
    length: crate::track_curves::ZERO_G_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const ZERO_G_ROLL_RIGHT: TrackSection = TrackSection {
    name: "zero_g_roll_right",
    curve: crate::track_curves::zero_g_roll_right,
    length: crate::track_curves::ZERO_G_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_ZERO_G_ROLL_LEFT: TrackSection = TrackSection {
    name: "large_zero_g_roll_left",
    curve: crate::track_curves::large_zero_g_roll_left,
    length: crate::track_curves::LARGE_ZERO_G_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const LARGE_ZERO_G_ROLL_RIGHT: TrackSection = TrackSection {
    name: "large_zero_g_roll_right",
    curve: crate::track_curves::large_zero_g_roll_right,
    length: crate::track_curves::LARGE_ZERO_G_ROLL_LENGTH,
    position_offset: POSITION_OFFSET_ORTHOGONAL,
    mask_offset_y: true,
};

pub const DIVE_LOOP_45_LEFT: TrackSection = TrackSection {
    name: "dive_loop_45_left",
    curve: crate::track_curves::dive_loop_45_left,
    length: crate::track_curves::DIVE_LOOP_45_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};

pub const DIVE_LOOP_45_RIGHT: TrackSection = TrackSection {
    name: "dive_loop_45_right",
    curve: crate::track_curves::dive_loop_45_right,
    length: crate::track_curves::DIVE_LOOP_45_LENGTH,
    position_offset: POSITION_OFFSET_DIAGONAL,
    mask_offset_y: false,
};
