pub const FLAT_LENGTH: f32 = 1.0;
pub const GENTLE_LENGTH: f32 = 1.080123;
pub const MEDIUM_TURN_LEFT_LENGTH: f32 = 1.25 * std::f32::consts::PI;

pub fn flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::plane_curve_vertical(&glam::Vec3::new(0.0, 0.0, distance), &glam::Vec3::new(0.0, 0.0, 1.0))
}

pub fn gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let u = distance / GENTLE_LENGTH;
    crate::curves::plane_curve_vertical(
        &glam::Vec3::new(0.0, 2.0 * crate::CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(0.0, 2.0 * crate::CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

pub fn medium_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    const RADIUS: f32 = -2.5;
    let angle = distance / RADIUS;
    let angle_sin = angle.sin();
    let angle_cos = angle.cos();
    let tangent = glam::Vec3::new(angle_sin, 0.0, angle_cos);
    let normal = glam::Vec3::new(0.0, 1.0, 0.0);
    crate::track_sections::TrackPoint {
        position: glam::Vec3::new(RADIUS * (1.0 - angle_cos), 0.0, RADIUS * angle_sin),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

pub fn flat_to_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), -bank_angle * distance / FLAT_LENGTH)
}

pub fn flat_to_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), bank_angle * distance / FLAT_LENGTH)
}
