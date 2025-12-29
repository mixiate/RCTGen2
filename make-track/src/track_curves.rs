use crate::CLEARANCE_HEIGHT;

pub const FLAT_LENGTH: f32 = 1.0;
pub const FLAT_TO_GENTLE_LENGTH: f32 = 1.027122;
pub const GENTLE_LENGTH: f32 = 1.080123;
pub const GENTLE_TO_STEEP_LENGTH: f32 = 1.314179;
pub const STEEP_LENGTH: f32 = 1.914854;
pub const STEEP_TO_VERTICAL_LENGTH: f32 = 1.535172;
pub const VERTICAL_TO_STEEP_LENGTH: f32 = 1.531568;
pub const VERTICAL_LENGTH: f32 = CLEARANCE_HEIGHT * 4.0;
pub const SMALL_FLAT_TO_STEEP_LENGTH: f32 = 1.220_88;
pub const FLAT_TO_STEEP_LENGTH: f32 = 4.792426;
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
        CLEARANCE_HEIGHT,
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
        -CLEARANCE_HEIGHT,
        2.0 * CLEARANCE_HEIGHT,
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
        &glam::Vec3::new(0.0, 2.0 * CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(0.0, 2.0 * CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

pub fn gentle_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -0.5,
        1.0,
        0.5,
        0.0,
        CLEARANCE_HEIGHT,
        2.0 * CLEARANCE_HEIGHT,
        CLEARANCE_HEIGHT,
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
        CLEARANCE_HEIGHT,
        -5.0 * CLEARANCE_HEIGHT,
        8.0 * CLEARANCE_HEIGHT,
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
        &glam::Vec3::new(0.0, 8.0 * CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(0.0, 8.0 * CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

pub fn steep_to_vertical(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -1.0 / 6.0,
        -1.0 / 6.0,
        5.0 / 6.0,
        -1.0 / 2.0,
        2.0 * CLEARANCE_HEIGHT / 3.0,
        -CLEARANCE_HEIGHT / 3.0,
        20.0 * CLEARANCE_HEIGHT / 3.0,
        0.0,
        5.185_948e-4,
        1.054_811_2e-3,
        -9.063_116e-3,
        3.875_790_9e-3,
        -4.515_805e-3,
        3.644_174_7e-2,
        6.266_248e-1,
        distance,
    )
}

pub fn vertical_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -1.0 / 6.0,
        2.0 / 3.0,
        0.0,
        0.0,
        -2.0 * CLEARANCE_HEIGHT / 3.0,
        CLEARANCE_HEIGHT,
        20.0 * CLEARANCE_HEIGHT / 3.0,
        0.0,
        -1.151_782_9e-3,
        8.245_657e-3,
        -2.346_102_5e-2,
        3.383_574e-2,
        -1.611_165_1e-3,
        -8.180_113e-2,
        7.348_989e-1,
        distance,
    )
}

pub fn vertical(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::plane_curve_vertical(&glam::Vec3::new(0.0, distance, 0.0), &glam::Vec3::new(0.0, 1.0, 0.0))
}

pub fn small_flat_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        2.0 * CLEARANCE_HEIGHT - 1.0,
        2.0 - 4.0 * CLEARANCE_HEIGHT,
        2.0 * CLEARANCE_HEIGHT,
        0.0,
        2.0 * CLEARANCE_HEIGHT,
        1.0 * CLEARANCE_HEIGHT,
        0.0,
        0.0,
        2.748_838_2,
        -1.327_702_9e1,
        2.621_346_5e1,
        -2.736_744e1,
        1.644_119_6e1,
        -6.035_096,
        2.154_704,
        distance,
    )
}

pub fn small_steep_to_flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        2.0 * CLEARANCE_HEIGHT - 1.0,
        1.0 - 2.0 * CLEARANCE_HEIGHT,
        1.0,
        0.0,
        2.0 * CLEARANCE_HEIGHT,
        -7.0 * CLEARANCE_HEIGHT,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        2.748_838_2,
        -1.021_498e1,
        1.499_828_2e1,
        -1.087_914_2e1,
        4.042_006,
        -5.643_233e-1,
        5.634_841e-1,
        distance,
    )
}

pub fn flat_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -0.5,
        -0.5,
        5.0,
        0.0,
        -2.0 * CLEARANCE_HEIGHT,
        13.0 * CLEARANCE_HEIGHT,
        0.0,
        0.0,
        8.279_922e-8,
        -1.880_159_6e-6,
        1.284_184_9e-5,
        -1.130_966_8e-5,
        -5.341_27e-4,
        3.995_885_6e-3,
        2.000_007_6e-1,
        distance,
    )
}

pub fn steep_to_flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        -0.5,
        2.0,
        2.5,
        0.0,
        -2.0 * CLEARANCE_HEIGHT,
        -7.0 * CLEARANCE_HEIGHT,
        20.0 * CLEARANCE_HEIGHT,
        0.0,
        8.279_922e-8,
        -8.975_044e-7,
        -1.286_059_2e-6,
        3.234_794_3e-5,
        -4.117_777_6e-4,
        1.588_081_7e-3,
        2.088_928_8e-1,
        distance,
    )
}

pub fn medium_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::turn_left(distance, 2.5)
}

pub fn flat_to_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), -bank_angle * distance / FLAT_LENGTH)
}

pub fn flat_to_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), bank_angle * distance / FLAT_LENGTH)
}
