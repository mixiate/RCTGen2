pub const FLAT_LENGTH: f32 = 1.0;
pub const FLAT_TO_GENTLE_LENGTH: f32 = 1.027122;
pub const GENTLE_LENGTH: f32 = 1.080123;
pub const GENTLE_TO_STEEP_LENGTH: f32 = 1.314179;
pub const STEEP_LENGTH: f32 = 1.914854;
pub const VERTICAL_LENGTH: f32 = crate::CLEARANCE_HEIGHT * 4.0;
pub const MEDIUM_TURN_LEFT_LENGTH: f32 = 1.25 * std::f32::consts::PI;

pub fn flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::plane_curve_vertical(&glam::Vec3::new(0.0, 0.0, distance), &glam::Vec3::new(0.0, 0.0, 1.0))
}

pub fn flat_to_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        crate::CLEARANCE_HEIGHT,
        0.0,
        0.0,
        1.392_075_3e-5,
        -9.136_681_3e-4,
        3.826_916_2e-3,
        -3.936_109_4e-4,
        -2.767_597_9e-2,
        -1.273_791_5e-5,
        1.000_000_6,
        distance,
    )
}

pub fn gentle_to_flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -crate::CLEARANCE_HEIGHT,
        2.0 * crate::CLEARANCE_HEIGHT,
        0.0,
        1.392_082_2e-5,
        8.135_797e-4,
        -1.495_364_8e-3,
        -5.329_378_4e-3,
        -8.178_555e-3,
        6.123_512_2e-2,
        9.258_196_4e-1,
        distance,
    )
}

pub fn gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let u = distance / GENTLE_LENGTH;
    crate::curves::plane_curve_vertical(
        &glam::Vec3::new(0.0, 2.0 * crate::CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(0.0, 2.0 * crate::CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

pub fn gentle_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -0.5,
        1.0,
        0.5,
        0.0,
        crate::CLEARANCE_HEIGHT,
        2.0 * crate::CLEARANCE_HEIGHT,
        crate::CLEARANCE_HEIGHT,
        0.0,
        9.384_358_5e-1,
        -4.936_207_3,
        1.066_758_3e1,
        -1.230_785e1,
        8.338_881,
        -3.615_561_7,
        1.741_669,
        distance,
    )
}

pub fn steep_to_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -0.5,
        0.5,
        1.0,
        0.0,
        crate::CLEARANCE_HEIGHT,
        -5.0 * crate::CLEARANCE_HEIGHT,
        8.0 * crate::CLEARANCE_HEIGHT,
        0.0,
        9.384_358_5e-1,
        -3.696_703,
        5.780_79,
        -4.458_387_4,
        1.774_045_1,
        -2.382_700_1e-1,
        5.429_020_5e-1,
        distance,
    )
}

pub fn steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let u = distance / STEEP_LENGTH;
    crate::curves::plane_curve_vertical(
        &glam::Vec3::new(0.0, 8.0 * crate::CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(0.0, 8.0 * crate::CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

pub fn vertical(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::plane_curve_vertical(&glam::Vec3::new(0.0, distance, 0.0), &glam::Vec3::new(0.0, 1.0, 0.0))
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
