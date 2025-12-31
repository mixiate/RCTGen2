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

pub const VERY_SMALL_TURN_RADIUS: f32 = 0.5;
pub const SMALL_TURN_RADIUS: f32 = 1.5;
pub const MEDIUM_TURN_RADIUS: f32 = 2.5;

pub const VERY_SMALL_TURN_LENGTH: f32 = (VERY_SMALL_TURN_RADIUS / 2.0) * std::f32::consts::PI;
pub const SMALL_TURN_LENGTH: f32 = (SMALL_TURN_RADIUS / 2.0) * std::f32::consts::PI;
pub const MEDIUM_TURN_LENGTH: f32 = (MEDIUM_TURN_RADIUS / 2.0) * std::f32::consts::PI;
pub const LARGE_TURN_LENGTH: f32 = 2.757_1;

pub const FLAT_DIAG_LENGTH: f32 = std::f32::consts::SQRT_2;
pub const FLAT_TO_GENTLE_DIAG_LENGTH: f32 = 1.433617;
pub const GENTLE_DIAG_LENGTH: f32 = 1.471_96;
pub const GENTLE_TO_STEEP_DIAG_LENGTH: f32 = 1.656243;
pub const STEEP_DIAG_LENGTH: f32 = 2.160247;
pub const SMALL_FLAT_TO_STEEP_DIAG_LENGTH: f32 = 1.584328;
pub const FLAT_TO_STEEP_DIAG_LENGTH: f32 = 4.956727;

pub const SMALL_TURN_GENTLE_LENGTH: f32 = 2.493656;
pub const MEDIUM_TURN_GENTLE_LENGTH: f32 = 4.252_99;
pub const LARGE_TURN_GENTLE_LENGTH: f32 = 3.017199;
pub const VERY_SMALL_TURN_STEEP_LENGTH: f32 = 1.812048;
pub const VERTICAL_TWIST_LENGTH: f32 = CLEARANCE_HEIGHT * 12.0;

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

pub fn small_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::turn_left(distance, SMALL_TURN_RADIUS)
}

pub fn medium_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::turn_left(distance, MEDIUM_TURN_RADIUS)
}

pub fn large_turn_left_to_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_horizontal(
        68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
        7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
        44.0 * CLEARANCE_HEIGHT / 3.0,
        0.0,
        2.0 - 8.0 * CLEARANCE_HEIGHT,
        8.0 * CLEARANCE_HEIGHT - 3.0,
        0.0,
        0.0,
        1.762_491_6e-5,
        -8.695_496e-5,
        2.063_995_9e-4,
        5.643_969_6e-4,
        -1.450_647_5e-4,
        4.397_764e-3,
        3.340_337e-1,
        distance,
    )
}

pub fn large_turn_right_to_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_horizontal(
        68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
        7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
        44.0 * CLEARANCE_HEIGHT / 3.0,
        0.0,
        8.0 * CLEARANCE_HEIGHT - 2.0,
        3.0 - 8.0 * CLEARANCE_HEIGHT,
        0.0,
        0.0,
        1.762_491_6e-5,
        -8.695_496e-5,
        2.063_995_9e-4,
        5.643_969_6e-4,
        -1.450_647_5e-4,
        4.397_764e-3,
        3.340_337e-1,
        distance,
    )
}

pub fn flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let position = distance / FLAT_DIAG_LENGTH;
    let tangent = 0.5_f32.sqrt();
    crate::curves::plane_curve_horizontal(
        &glam::Vec3::new(position, 0.0, position),
        &glam::Vec3::new(tangent, 0.0, tangent),
    )
}

pub fn flat_to_gentle_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        0.0,
        0.0,
        FLAT_DIAG_LENGTH,
        0.0,
        0.0,
        CLEARANCE_HEIGHT,
        0.0,
        0.0,
        -1.738_419_3e-6,
        -9.544_348e-6,
        1.454_93e-4,
        -8.696_576_5e-6,
        -4.907_240_6e-3,
        -5.745_786e-7,
        7.071_068e-1,
        distance,
    )
}

pub fn gentle_to_flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        0.0,
        0.0,
        FLAT_DIAG_LENGTH,
        0.0,
        0.0,
        -CLEARANCE_HEIGHT,
        2.0 * CLEARANCE_HEIGHT,
        0.0,
        -1.738_415_8e-6,
        2.698_992_1e-5,
        -1.163_548_9e-5,
        -5.606_923e-4,
        -2.786_307_6e-3,
        1.775_206_3e-2,
        6.793_662e-1,
        distance,
    )
}

pub fn gentle_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::SQRT_2;
    let u = distance / GENTLE_DIAG_LENGTH;
    crate::curves::plane_curve_vertical_diagonal(
        &glam::Vec3::new(1.0 * u, 2.0 * CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(1.0 / SQRT_2, 2.0 * CLEARANCE_HEIGHT / SQRT_2, 1.0 / SQRT_2).normalize(),
    )
}

pub fn gentle_to_steep_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        -0.5 * FLAT_DIAG_LENGTH,
        FLAT_DIAG_LENGTH,
        0.5 * FLAT_DIAG_LENGTH,
        0.0,
        CLEARANCE_HEIGHT,
        2.0 * CLEARANCE_HEIGHT,
        CLEARANCE_HEIGHT,
        0.0,
        1.614_862e-1,
        -1.072_558_9,
        2.927_890_3,
        -4.272_877,
        3.677_610_6,
        -2.025_951_4,
        1.286_073_2,
        distance,
    )
}

pub fn steep_to_gentle_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        -0.5 * FLAT_DIAG_LENGTH,
        0.5 * FLAT_DIAG_LENGTH,
        FLAT_DIAG_LENGTH,
        0.0,
        CLEARANCE_HEIGHT,
        -5.0 * CLEARANCE_HEIGHT,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        1.614_862e-1,
        -7.996_634e-1,
        1.571_947_1,
        -1.519_760_8,
        7.571_887e-1,
        -1.457_521_8e-1,
        4.770_328e-1,
        distance,
    )
}

pub fn steep_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let u = distance / STEEP_DIAG_LENGTH;
    crate::curves::plane_curve_vertical_diagonal(
        &glam::Vec3::new(u, 8.0 * CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(1.0, 8.0 * CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

pub fn small_flat_to_steep_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::SQRT_2;
    crate::curves::cubic_curve_vertical_diagonal(
        SQRT_2 * (2.0 * CLEARANCE_HEIGHT - 1.0),
        SQRT_2 * (2.0 - 4.0 * CLEARANCE_HEIGHT),
        SQRT_2 * (2.0 * CLEARANCE_HEIGHT),
        0.0,
        2.0 * CLEARANCE_HEIGHT,
        CLEARANCE_HEIGHT,
        0.0,
        0.0,
        3.915_141_2e-1,
        -2.456_954_5,
        6.302_896_5,
        -8.560_122_5,
        6.717_914,
        -3.224_898_6,
        1.541_950_8,
        distance,
    )
}

pub fn small_steep_to_flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::SQRT_2;
    crate::curves::cubic_curve_vertical_diagonal(
        SQRT_2 * (2.0 * CLEARANCE_HEIGHT - 1.0),
        SQRT_2 * (1.0 - 2.0 * CLEARANCE_HEIGHT),
        SQRT_2,
        0.0,
        2.0 * CLEARANCE_HEIGHT,
        -7.0 * CLEARANCE_HEIGHT,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        3.915_141_2e-1,
        -1.885_053_3,
        3.584_659_6,
        -3.355_575_8,
        1.598_218_3,
        -3.073_66e-1,
        4.906_735e-1,
        distance,
    )
}

pub fn flat_to_steep_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        0.0,
        -FLAT_DIAG_LENGTH,
        4.0 * FLAT_DIAG_LENGTH,
        0.0,
        -6.0 * CLEARANCE_HEIGHT,
        17.0 * CLEARANCE_HEIGHT,
        0.0,
        0.0,
        8.712_086_7e-7,
        -1.521_515_3e-5,
        1.200_665_8e-4,
        -3.473_318_4e-4,
        -4.313_369e-4,
        7.658_432_7e-3,
        1.768_087_3e-1,
        distance,
    )
}

pub fn steep_to_flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        0.0,
        FLAT_DIAG_LENGTH,
        2.0 * FLAT_DIAG_LENGTH,
        0.0,
        -6.0 * CLEARANCE_HEIGHT,
        CLEARANCE_HEIGHT,
        16.0 * CLEARANCE_HEIGHT,
        0.0,
        8.712_086_7e-7,
        -1.501_325_4e-5,
        1.17064288e-04,
        -7.344_354e-4,
        3.529_136_3e-3,
        -1.323_633_9e-2,
        2.314_236_3e-1,
        distance,
    )
}

pub fn flat_to_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), -bank_angle * distance / FLAT_LENGTH)
}

pub fn flat_to_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), bank_angle * distance / FLAT_LENGTH)
}

pub fn left_bank_to_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle(distance, 0.0),
        -bank_angle * (1.0 - distance / FLAT_TO_GENTLE_LENGTH),
    )
}

pub fn right_bank_to_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle(distance, 0.0),
        bank_angle * (1.0 - distance / FLAT_TO_GENTLE_LENGTH),
    )
}

pub fn gentle_to_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat(distance, 0.0),
        -bank_angle * distance / FLAT_TO_GENTLE_LENGTH,
    )
}

pub fn gentle_to_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat(distance, 0.0),
        bank_angle * distance / FLAT_TO_GENTLE_LENGTH,
    )
}

pub fn left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat(distance, 0.0), -bank_angle)
}

pub fn small_turn_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&small_turn_left(distance, 0.0), -bank_angle)
}

pub fn medium_turn_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&medium_turn_left(distance, 0.0), -bank_angle)
}

pub fn large_turn_left_to_diag_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&large_turn_left_to_diag(distance, 0.0), -bank_angle)
}

pub fn large_turn_right_to_diag_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&large_turn_right_to_diag(distance, 0.0), bank_angle)
}

pub fn flat_to_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_diag(distance, 0.0), -bank_angle * distance / FLAT_DIAG_LENGTH)
}

pub fn flat_to_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_diag(distance, 0.0), bank_angle * distance / FLAT_DIAG_LENGTH)
}

pub fn left_bank_to_gentle_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle_diag(distance, 0.0),
        -bank_angle * (1.0 - distance / FLAT_TO_GENTLE_DIAG_LENGTH),
    )
}

pub fn right_bank_to_gentle_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle_diag(distance, 0.0),
        bank_angle * (1.0 - distance / FLAT_TO_GENTLE_DIAG_LENGTH),
    )
}

pub fn gentle_to_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat_diag(distance, 0.0),
        -bank_angle * distance / FLAT_TO_GENTLE_DIAG_LENGTH,
    )
}

pub fn gentle_to_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat_diag(distance, 0.0),
        bank_angle * distance / FLAT_TO_GENTLE_DIAG_LENGTH,
    )
}

pub fn left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_diag(distance, 0.0), -bank_angle)
}

pub fn small_turn_left_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::sloped_turn_left(SMALL_TURN_RADIUS, 4.0 * CLEARANCE_HEIGHT / SMALL_TURN_LENGTH, distance)
}

pub fn small_turn_right_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::sloped_turn_right(SMALL_TURN_RADIUS, 4.0 * CLEARANCE_HEIGHT / SMALL_TURN_LENGTH, distance)
}

pub fn medium_turn_left_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::sloped_turn_left(
        MEDIUM_TURN_RADIUS,
        8.0 * CLEARANCE_HEIGHT / MEDIUM_TURN_LENGTH,
        distance,
    )
}

pub fn medium_turn_right_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::sloped_turn_right(
        MEDIUM_TURN_RADIUS,
        8.0 * CLEARANCE_HEIGHT / MEDIUM_TURN_LENGTH,
        distance,
    )
}

pub fn large_turn_left_to_diag_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_diag_gentle(
        68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
        7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
        44.0 * CLEARANCE_HEIGHT / 3.0,
        0.0,
        2.0 - 8.0 * CLEARANCE_HEIGHT,
        8.0 * CLEARANCE_HEIGHT - 3.0,
        0.0,
        0.0,
        distance,
    )
}

pub fn large_turn_right_to_diag_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_diag_gentle(
        68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
        7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
        44.0 * CLEARANCE_HEIGHT / 3.0,
        0.0,
        8.0 * CLEARANCE_HEIGHT - 2.0,
        3.0 - 8.0 * CLEARANCE_HEIGHT,
        0.0,
        0.0,
        distance,
    )
}

pub fn large_turn_left_to_orthogonal_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_orthogonal_gentle(
        68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
        7.5 - 92.0 * CLEARANCE_HEIGHT / 3.0,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        24.0 * CLEARANCE_HEIGHT / 3.0 - 2.0,
        3.0 - 48.0 * CLEARANCE_HEIGHT / 3.0,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        distance,
    )
}

pub fn large_turn_right_to_orthogonal_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_orthogonal_gentle(
        24.0 * CLEARANCE_HEIGHT / 3.0 - 2.0,
        3.0 - 48.0 * CLEARANCE_HEIGHT / 3.0,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
        7.5 - 92.0 * CLEARANCE_HEIGHT / 3.0,
        8.0 * CLEARANCE_HEIGHT,
        0.0,
        distance,
    )
}

pub fn very_small_turn_left_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::sloped_turn_left(
        VERY_SMALL_TURN_RADIUS,
        8.0 * CLEARANCE_HEIGHT / VERY_SMALL_TURN_LENGTH,
        distance,
    )
}

pub fn very_small_turn_right_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::sloped_turn_right(
        VERY_SMALL_TURN_RADIUS,
        8.0 * CLEARANCE_HEIGHT / VERY_SMALL_TURN_LENGTH,
        distance,
    )
}

pub fn vertical_twist_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let tangent = glam::Vec3::new(0.0, 1.0, 0.0);
    let normal = 0.5 * std::f32::consts::PI * distance / VERTICAL_TWIST_LENGTH;
    let normal = glam::Vec3::new(normal.sin(), 0.0, -normal.cos());
    crate::track_sections::TrackPoint {
        position: glam::Vec3::new(0.0, distance, 0.0),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

pub fn vertical_twist_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let tangent = glam::Vec3::new(0.0, 1.0, 0.0);
    let normal = 0.5 * std::f32::consts::PI * distance / VERTICAL_TWIST_LENGTH;
    let normal = glam::Vec3::new(-normal.sin(), 0.0, -normal.cos());
    crate::track_sections::TrackPoint {
        position: glam::Vec3::new(0.0, distance, 0.0),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

pub fn gentle_to_gentle_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle(distance, 0.0), -bank_angle * distance / GENTLE_LENGTH)
}

pub fn gentle_to_gentle_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle(distance, 0.0), bank_angle * distance / GENTLE_LENGTH)
}

pub fn gentle_left_bank_to_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle(distance, 0.0), -bank_angle * (1.0 - distance / GENTLE_LENGTH))
}

pub fn gentle_right_bank_to_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle(distance, 0.0), bank_angle * (1.0 - distance / GENTLE_LENGTH))
}

pub fn left_bank_to_gentle_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_to_gentle(distance, 0.0), -bank_angle)
}

pub fn right_bank_to_gentle_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_to_gentle(distance, 0.0), bank_angle)
}

pub fn gentle_left_bank_to_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_to_flat(distance, 0.0), -bank_angle)
}

pub fn gentle_right_bank_to_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_to_flat(distance, 0.0), bank_angle)
}

pub fn gentle_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle(distance, 0.0), -bank_angle)
}

pub fn gentle_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle(distance, 0.0), bank_angle)
}

pub fn flat_to_gentle_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle(distance, 0.0),
        -bank_angle * distance / FLAT_TO_GENTLE_LENGTH,
    )
}

pub fn flat_to_gentle_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle(distance, 0.0),
        bank_angle * distance / FLAT_TO_GENTLE_LENGTH,
    )
}

pub fn gentle_left_bank_to_flat(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat(distance, 0.0),
        -bank_angle * (1.0 - distance / FLAT_TO_GENTLE_LENGTH),
    )
}

pub fn gentle_right_bank_to_flat(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat(distance, 0.0),
        bank_angle * (1.0 - distance / FLAT_TO_GENTLE_LENGTH),
    )
}
