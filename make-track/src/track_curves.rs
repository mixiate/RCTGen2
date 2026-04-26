use crate::CLEARANCE_HEIGHT;

pub const FLAT_LENGTH: f32 = 1.0;
pub const FLAT_TO_GENTLE_LENGTH: f32 = 1.027122;
pub const GENTLE_LENGTH: f32 = 1.080123;
pub const GENTLE_TO_STEEP_LENGTH: f32 = 1.314179;
pub const STEEP_LENGTH: f32 = 1.914854;
pub const STEEP_TO_VERTICAL_LENGTH: f32 = 1.535172;
pub const VERTICAL_TO_STEEP_LENGTH: f32 = 1.531568;
pub const VERTICAL_LENGTH: f32 = CLEARANCE_HEIGHT * 4.0;
pub const SMALL_FLAT_TO_STEEP_LENGTH: f32 = 1.22088;
pub const FLAT_TO_STEEP_LENGTH: f32 = 4.792426;

pub const VERY_SMALL_TURN_RADIUS: f32 = 0.5;
pub const SMALL_TURN_RADIUS: f32 = 1.5;
pub const MEDIUM_TURN_RADIUS: f32 = 2.5;

pub const VERY_SMALL_TURN_LENGTH: f32 = (VERY_SMALL_TURN_RADIUS / 2.0) * std::f32::consts::PI;
pub const SMALL_TURN_LENGTH: f32 = (SMALL_TURN_RADIUS / 2.0) * std::f32::consts::PI;
pub const MEDIUM_TURN_LENGTH: f32 = (MEDIUM_TURN_RADIUS / 2.0) * std::f32::consts::PI;
pub const LARGE_TURN_LENGTH: f32 = 2.7571;

pub const FLAT_DIAG_LENGTH: f32 = std::f32::consts::SQRT_2;
pub const FLAT_TO_GENTLE_DIAG_LENGTH: f32 = 1.433617;
pub const GENTLE_DIAG_LENGTH: f32 = 1.47196;
pub const GENTLE_TO_STEEP_DIAG_LENGTH: f32 = 1.656243;
pub const STEEP_DIAG_LENGTH: f32 = 2.160247;
pub const SMALL_FLAT_TO_STEEP_DIAG_LENGTH: f32 = 1.584328;
pub const FLAT_TO_STEEP_DIAG_LENGTH: f32 = 4.956727;

pub const SMALL_TURN_GENTLE_LENGTH: f32 = 2.493656;
pub const MEDIUM_TURN_GENTLE_LENGTH: f32 = 4.25299;
pub const LARGE_TURN_GENTLE_LENGTH: f32 = 3.017199;
pub const VERY_SMALL_TURN_STEEP_LENGTH: f32 = 1.812048;
pub const VERTICAL_TWIST_LENGTH: f32 = CLEARANCE_HEIGHT * 12.0;

pub const S_BEND_LENGTH: f32 = 3.24075;

pub const SMALL_HELIX_LENGTH: f32 = 2.36502;
pub const MEDIUM_HELIX_LENGTH: f32 = 3.932292;

pub const MEDIUM_QUARTER_HELIX_LENGTH: f32 = 3.948154;

pub const SMALL_TURN_BANK_TO_GENTLE_LENGTH: f32 = 2.44229;

pub const BARREL_ROLL_LENGTH: f32 = 3.091882;

const HALF_LOOP_SEGMENT_1_LENGTH: f32 = 0.540062;
const HALF_LOOP_SEGMENT_2_LENGTH: f32 = 2.685141;
const HALF_LOOP_SEGMENT_3_LENGTH: f32 = 1.956695;
pub const HALF_LOOP_LENGTH: f32 = HALF_LOOP_SEGMENT_1_LENGTH + HALF_LOOP_SEGMENT_2_LENGTH + HALF_LOOP_SEGMENT_3_LENGTH;

const VERTICAL_LOOP_FACTOR: f32 = 1.006604;
const VERTICAL_LOOP_SEGMENT_1_LENGTH: f32 = 0.540062;
const VERTICAL_LOOP_SEGMENT_2_LENGTH: f32 = VERTICAL_LOOP_SEGMENT_1_LENGTH + 2.686603;
pub const VERTICAL_LOOP_LENGTH: f32 = (VERTICAL_LOOP_SEGMENT_2_LENGTH + 1.730928) * VERTICAL_LOOP_FACTOR;

pub const QUARTER_LOOP_LENGTH: f32 = 4.253756;

const CORKSCREW_SEGMENT_1_LENGTH: f32 = 1.682311;
const CORKSCREW_SEGMENT_2_LENGTH: f32 = 1.744083;
pub const CORKSCREW_LENGTH: f32 = CORKSCREW_SEGMENT_1_LENGTH + CORKSCREW_SEGMENT_2_LENGTH;

const LARGE_CORKSCREW_SEGMENT_LENGTH: f32 = 2.665301;
pub const LARGE_CORKSCREW_LENGTH: f32 = LARGE_CORKSCREW_SEGMENT_LENGTH * 2.0;

const MEDIUM_HALF_LOOP_FACTOR: f32 = 1.0050563;
const MEDIUM_HALF_LOOP_SEGMENT_1_LENGTH: f32 = 4.605006;
const MEDIUM_HALF_LOOP_SEGMENT_2_LENGTH: f32 = 2.988654;
pub const MEDIUM_HALF_LOOP_LENGTH: f32 =
    (MEDIUM_HALF_LOOP_SEGMENT_1_LENGTH + MEDIUM_HALF_LOOP_SEGMENT_2_LENGTH) * MEDIUM_HALF_LOOP_FACTOR;

const LARGE_HALF_LOOP_FACTOR: f32 = 1.0050563;
const LARGE_HALF_LOOP_SEGMENT_1_LENGTH: f32 = 1.5 * GENTLE_LENGTH;
const LARGE_HALF_LOOP_SEGMENT_2_LENGTH: f32 = LARGE_HALF_LOOP_SEGMENT_1_LENGTH + 4.766127;
pub const LARGE_HALF_LOOP_LENGTH: f32 = (LARGE_HALF_LOOP_SEGMENT_2_LENGTH + 3.54535) * LARGE_HALF_LOOP_FACTOR;

const ZERO_G_ROLL_BASE_LENGTH: f32 = 3.083249;
pub const ZERO_G_ROLL_LENGTH: f32 = 3.266924;

const LARGE_ZERO_G_ROLL_BASE_LENGTH: f32 = 5.385804;
pub const LARGE_ZERO_G_ROLL_LENGTH: f32 = 5.568162;

pub const DIVE_LOOP_45_LENGTH: f32 = 5.335896;

pub fn flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::plane_curve_vertical(&glam::Vec3::new(0.0, 0.0, distance), &glam::Vec3::new(0.0, 0.0, 1.0))
}

pub fn flat_to_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[0.0, 0.0, 1.0, 0.0],
        &[0.0, CLEARANCE_HEIGHT, 0.0, 0.0],
        1.3920753e-5,
        -9.1366813e-4,
        3.8269162e-3,
        -3.9361094e-4,
        -2.7675979e-2,
        -1.2737915e-5,
        1.0000006,
        distance,
    )
}

pub fn gentle_to_flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[0.0, 0.0, 1.0, 0.0],
        &[0.0, -CLEARANCE_HEIGHT, 2.0 * CLEARANCE_HEIGHT, 0.0],
        1.3920822e-5,
        8.135797e-4,
        -1.4953648e-3,
        -5.3293784e-3,
        -8.178555e-3,
        6.1235122e-2,
        9.2581964e-1,
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
        &[-0.5, 1.0, 0.5, 0.0],
        &[CLEARANCE_HEIGHT, 2.0 * CLEARANCE_HEIGHT, CLEARANCE_HEIGHT, 0.0],
        9.3843585e-1,
        -4.9362073,
        1.0667583e1,
        -1.230785e1,
        8.338881,
        -3.6155617,
        1.741669,
        distance,
    )
}

pub fn steep_to_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[-0.5, 0.5, 1.0, 0.0],
        &[CLEARANCE_HEIGHT, -5.0 * CLEARANCE_HEIGHT, 8.0 * CLEARANCE_HEIGHT, 0.0],
        9.3843585e-1,
        -3.696703,
        5.78079,
        -4.4583874,
        1.7740451,
        -2.3827001e-1,
        5.4290205e-1,
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
        &[-1.0 / 6.0, -1.0 / 6.0, 5.0 / 6.0, -1.0 / 2.0],
        &[
            2.0 * CLEARANCE_HEIGHT / 3.0,
            -CLEARANCE_HEIGHT / 3.0,
            20.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        5.185948e-4,
        1.0548112e-3,
        -9.063116e-3,
        3.8757909e-3,
        -4.515805e-3,
        3.6441747e-2,
        6.266248e-1,
        distance,
    )
}

pub fn vertical_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[-1.0 / 6.0, 2.0 / 3.0, 0.0, 0.0],
        &[
            -2.0 * CLEARANCE_HEIGHT / 3.0,
            CLEARANCE_HEIGHT,
            20.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        -1.1517829e-3,
        8.245657e-3,
        -2.3461025e-2,
        3.383574e-2,
        -1.6111651e-3,
        -8.180113e-2,
        7.348989e-1,
        distance,
    )
}

pub fn vertical(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::plane_curve_vertical(&glam::Vec3::new(0.0, distance, 0.0), &glam::Vec3::new(0.0, 1.0, 0.0))
}

pub fn small_flat_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[
            2.0 * CLEARANCE_HEIGHT - 1.0,
            2.0 - 4.0 * CLEARANCE_HEIGHT,
            2.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        &[2.0 * CLEARANCE_HEIGHT, 1.0 * CLEARANCE_HEIGHT, 0.0, 0.0],
        2.7488382,
        -1.3277029e1,
        2.6213465e1,
        -2.736744e1,
        1.6441196e1,
        -6.035096,
        2.154704,
        distance,
    )
}

pub fn small_steep_to_flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[2.0 * CLEARANCE_HEIGHT - 1.0, 1.0 - 2.0 * CLEARANCE_HEIGHT, 1.0, 0.0],
        &[
            2.0 * CLEARANCE_HEIGHT,
            -7.0 * CLEARANCE_HEIGHT,
            8.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        2.7488382,
        -1.021498e1,
        1.4998282e1,
        -1.0879142e1,
        4.042006,
        -5.643233e-1,
        5.634841e-1,
        distance,
    )
}

pub fn flat_to_steep(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[-0.5, -0.5, 5.0, 0.0],
        &[-2.0 * CLEARANCE_HEIGHT, 13.0 * CLEARANCE_HEIGHT, 0.0, 0.0],
        8.279922e-8,
        -1.8801596e-6,
        1.2841849e-5,
        -1.1309668e-5,
        -5.34127e-4,
        3.9958856e-3,
        2.0000076e-1,
        distance,
    )
}

pub fn steep_to_flat(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[-0.5, 2.0, 2.5, 0.0],
        &[
            -2.0 * CLEARANCE_HEIGHT,
            -7.0 * CLEARANCE_HEIGHT,
            20.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        8.279922e-8,
        -8.975044e-7,
        -1.2860592e-6,
        3.2347943e-5,
        -4.1177776e-4,
        1.5880817e-3,
        2.0889288e-1,
        distance,
    )
}

pub fn very_small_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::turn_left(distance, VERY_SMALL_TURN_RADIUS)
}

pub fn small_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::turn_left(distance, SMALL_TURN_RADIUS)
}

pub fn medium_turn_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::turn_left(distance, MEDIUM_TURN_RADIUS)
}

pub fn large_turn_left_to_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_horizontal(
        &[
            68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
            7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
            44.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        &[2.0 - 8.0 * CLEARANCE_HEIGHT, 8.0 * CLEARANCE_HEIGHT - 3.0, 0.0, 0.0],
        1.7624916e-5,
        -8.695496e-5,
        2.0639959e-4,
        5.6439696e-4,
        -1.4506475e-4,
        4.397764e-3,
        3.340337e-1,
        distance,
    )
}

pub fn large_turn_right_to_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_horizontal(
        &[
            68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
            7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
            44.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        &[8.0 * CLEARANCE_HEIGHT - 2.0, 3.0 - 8.0 * CLEARANCE_HEIGHT, 0.0, 0.0],
        1.7624916e-5,
        -8.695496e-5,
        2.0639959e-4,
        5.6439696e-4,
        -1.4506475e-4,
        4.397764e-3,
        3.340337e-1,
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
        &[0.0, 0.0, FLAT_DIAG_LENGTH, 0.0],
        &[0.0, CLEARANCE_HEIGHT, 0.0, 0.0],
        -1.7384193e-6,
        -9.544348e-6,
        1.45493e-4,
        -8.6965765e-6,
        -4.9072406e-3,
        -5.745786e-7,
        7.071068e-1,
        distance,
    )
}

pub fn gentle_to_flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        &[0.0, 0.0, FLAT_DIAG_LENGTH, 0.0],
        &[0.0, -CLEARANCE_HEIGHT, 2.0 * CLEARANCE_HEIGHT, 0.0],
        -1.7384158e-6,
        2.6989921e-5,
        -1.1635489e-5,
        -5.606923e-4,
        -2.7863076e-3,
        1.7752063e-2,
        6.793662e-1,
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
        &[-0.5 * FLAT_DIAG_LENGTH, FLAT_DIAG_LENGTH, 0.5 * FLAT_DIAG_LENGTH, 0.0],
        &[CLEARANCE_HEIGHT, 2.0 * CLEARANCE_HEIGHT, CLEARANCE_HEIGHT, 0.0],
        1.614862e-1,
        -1.0725589,
        2.9278903,
        -4.272877,
        3.6776106,
        -2.0259514,
        1.2860732,
        distance,
    )
}

pub fn steep_to_gentle_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        &[-0.5 * FLAT_DIAG_LENGTH, 0.5 * FLAT_DIAG_LENGTH, FLAT_DIAG_LENGTH, 0.0],
        &[CLEARANCE_HEIGHT, -5.0 * CLEARANCE_HEIGHT, 8.0 * CLEARANCE_HEIGHT, 0.0],
        1.614862e-1,
        -7.996634e-1,
        1.5719471,
        -1.5197608,
        7.571887e-1,
        -1.4575218e-1,
        4.770328e-1,
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
        &[
            SQRT_2 * (2.0 * CLEARANCE_HEIGHT - 1.0),
            SQRT_2 * (2.0 - 4.0 * CLEARANCE_HEIGHT),
            SQRT_2 * (2.0 * CLEARANCE_HEIGHT),
            0.0,
        ],
        &[2.0 * CLEARANCE_HEIGHT, CLEARANCE_HEIGHT, 0.0, 0.0],
        3.9151412e-1,
        -2.4569545,
        6.3028965,
        -8.5601225,
        6.717914,
        -3.2248986,
        1.5419508,
        distance,
    )
}

pub fn small_steep_to_flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::SQRT_2;
    crate::curves::cubic_curve_vertical_diagonal(
        &[
            SQRT_2 * (2.0 * CLEARANCE_HEIGHT - 1.0),
            SQRT_2 * (1.0 - 2.0 * CLEARANCE_HEIGHT),
            SQRT_2,
            0.0,
        ],
        &[
            2.0 * CLEARANCE_HEIGHT,
            -7.0 * CLEARANCE_HEIGHT,
            8.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        3.9151412e-1,
        -1.8850533,
        3.5846596,
        -3.3555758,
        1.5982183,
        -3.07366e-1,
        4.906735e-1,
        distance,
    )
}

pub fn flat_to_steep_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        &[0.0, -FLAT_DIAG_LENGTH, 4.0 * FLAT_DIAG_LENGTH, 0.0],
        &[-6.0 * CLEARANCE_HEIGHT, 17.0 * CLEARANCE_HEIGHT, 0.0, 0.0],
        8.7120867e-7,
        -1.5215153e-5,
        1.2006658e-4,
        -3.4733184e-4,
        -4.313369e-4,
        7.6584327e-3,
        1.7680873e-1,
        distance,
    )
}

pub fn steep_to_flat_diag(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical_diagonal(
        &[0.0, FLAT_DIAG_LENGTH, 2.0 * FLAT_DIAG_LENGTH, 0.0],
        &[-6.0 * CLEARANCE_HEIGHT, CLEARANCE_HEIGHT, 16.0 * CLEARANCE_HEIGHT, 0.0],
        8.7120867e-7,
        -1.5013254e-5,
        1.17064288e-04,
        -7.344354e-4,
        3.5291363e-3,
        -1.3236339e-2,
        2.3142363e-1,
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
        &[
            68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
            7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
            44.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        &[2.0 - 8.0 * CLEARANCE_HEIGHT, 8.0 * CLEARANCE_HEIGHT - 3.0, 0.0, 0.0],
        distance,
    )
}

pub fn large_turn_right_to_diag_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_diag_gentle(
        &[
            68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
            7.5 - 112.0 * CLEARANCE_HEIGHT / 3.0,
            44.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        &[8.0 * CLEARANCE_HEIGHT - 2.0, 3.0 - 8.0 * CLEARANCE_HEIGHT, 0.0, 0.0],
        distance,
    )
}

pub fn large_turn_left_to_orthogonal_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_orthogonal_gentle(
        &[
            68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
            7.5 - 92.0 * CLEARANCE_HEIGHT / 3.0,
            8.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        &[
            24.0 * CLEARANCE_HEIGHT / 3.0 - 2.0,
            3.0 - 48.0 * CLEARANCE_HEIGHT / 3.0,
            8.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        distance,
    )
}

pub fn large_turn_right_to_orthogonal_gentle(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::large_turn_to_orthogonal_gentle(
        &[
            24.0 * CLEARANCE_HEIGHT / 3.0 - 2.0,
            3.0 - 48.0 * CLEARANCE_HEIGHT / 3.0,
            8.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        &[
            68.0 * CLEARANCE_HEIGHT / 3.0 - 5.0,
            7.5 - 92.0 * CLEARANCE_HEIGHT / 3.0,
            8.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
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

pub fn gentle_to_gentle_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_diag(distance, 0.0), -bank_angle * distance / GENTLE_DIAG_LENGTH)
}

pub fn gentle_to_gentle_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_diag(distance, 0.0), bank_angle * distance / GENTLE_DIAG_LENGTH)
}

pub fn gentle_left_bank_to_gentle_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_diag(distance, 0.0),
        -bank_angle * (1.0 - distance / GENTLE_DIAG_LENGTH),
    )
}

pub fn gentle_right_bank_to_gentle_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_diag(distance, 0.0),
        bank_angle * (1.0 - distance / GENTLE_DIAG_LENGTH),
    )
}

pub fn left_bank_to_gentle_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_to_gentle_diag(distance, 0.0), -bank_angle)
}

pub fn right_bank_to_gentle_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&flat_to_gentle_diag(distance, 0.0), bank_angle)
}

pub fn gentle_left_bank_to_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_to_flat_diag(distance, 0.0), -bank_angle)
}

pub fn gentle_right_bank_to_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_to_flat_diag(distance, 0.0), bank_angle)
}

pub fn gentle_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_diag(distance, 0.0), -bank_angle)
}

pub fn gentle_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&gentle_diag(distance, 0.0), bank_angle)
}

pub fn flat_to_gentle_left_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle_diag(distance, 0.0),
        -bank_angle * distance / FLAT_TO_GENTLE_DIAG_LENGTH,
    )
}

pub fn flat_to_gentle_right_bank_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &flat_to_gentle_diag(distance, 0.0),
        bank_angle * distance / FLAT_TO_GENTLE_DIAG_LENGTH,
    )
}

pub fn gentle_left_bank_to_flat_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat_diag(distance, 0.0),
        -bank_angle * (1.0 - distance / FLAT_TO_GENTLE_DIAG_LENGTH),
    )
}

pub fn gentle_right_bank_to_flat_diag(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &gentle_to_flat_diag(distance, 0.0),
        bank_angle * (1.0 - distance / FLAT_TO_GENTLE_DIAG_LENGTH),
    )
}

pub fn small_turn_left_bank_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&small_turn_left_gentle(distance, 0.0), -bank_angle)
}

pub fn small_turn_right_bank_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&small_turn_right_gentle(distance, 0.0), bank_angle)
}

pub fn medium_turn_left_bank_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&medium_turn_left_gentle(distance, 0.0), -bank_angle)
}

pub fn medium_turn_right_bank_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&medium_turn_right_gentle(distance, 0.0), bank_angle)
}

pub fn large_turn_left_bank_to_diag_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&large_turn_left_to_diag_gentle(distance, 0.0), -bank_angle)
}

pub fn large_turn_right_bank_to_diag_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&large_turn_right_to_diag_gentle(distance, 0.0), bank_angle)
}

pub fn large_turn_left_bank_to_orthogonal_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&large_turn_left_to_orthogonal_gentle(distance, 0.0), -bank_angle)
}

pub fn large_turn_right_bank_to_orthogonal_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&large_turn_right_to_orthogonal_gentle(distance, 0.0), bank_angle)
}

pub fn s_bend_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_horizontal(
        &[
            152.0 * CLEARANCE_HEIGHT / 3.0 - 6.0,
            9.0 - 76.0 * CLEARANCE_HEIGHT,
            76.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        &[2.0, -3.0, 0.0, 0.0],
        -1.6356732e-3,
        1.8552829e-2,
        -7.5233884e-2,
        1.22409634e-01,
        -6.922434e-2,
        7.410992e-2,
        1.9046305e-1,
        distance,
    )
}

pub fn s_bend_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_horizontal(
        &[
            152.0 * CLEARANCE_HEIGHT / 3.0 - 6.0,
            9.0 - 76.0 * CLEARANCE_HEIGHT,
            76.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        &[-2.0, 3.0, 0.0, 0.0],
        -1.6356732e-3,
        1.8552829e-2,
        -7.5233884e-2,
        1.22409634e-01,
        -6.922434e-2,
        7.410992e-2,
        1.9046305e-1,
        distance,
    )
}

pub fn small_helix_left(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &crate::curves::sloped_turn_left(SMALL_TURN_RADIUS, CLEARANCE_HEIGHT / SMALL_TURN_LENGTH, distance),
        -bank_angle,
    )
}

pub fn small_helix_right(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &crate::curves::sloped_turn_right(SMALL_TURN_RADIUS, CLEARANCE_HEIGHT / SMALL_TURN_LENGTH, distance),
        bank_angle,
    )
}

pub fn medium_helix_left(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &crate::curves::sloped_turn_left(MEDIUM_TURN_RADIUS, CLEARANCE_HEIGHT / MEDIUM_TURN_LENGTH, distance),
        -bank_angle,
    )
}

pub fn medium_helix_right(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(
        &crate::curves::sloped_turn_right(MEDIUM_TURN_RADIUS, CLEARANCE_HEIGHT / MEDIUM_TURN_LENGTH, distance),
        bank_angle,
    )
}

pub fn medium_quarter_helix_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flatten_ends(
        crate::curves::sloped_turn_left(
            MEDIUM_TURN_RADIUS,
            2.0 * CLEARANCE_HEIGHT / MEDIUM_TURN_LENGTH,
            distance,
        ),
        distance / MEDIUM_QUARTER_HELIX_LENGTH,
    )
}

pub fn medium_quarter_helix_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flatten_ends(
        crate::curves::sloped_turn_right(
            MEDIUM_TURN_RADIUS,
            2.0 * CLEARANCE_HEIGHT / MEDIUM_TURN_LENGTH,
            distance,
        ),
        distance / MEDIUM_QUARTER_HELIX_LENGTH,
    )
}

pub fn medium_quarter_helix_left_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&medium_quarter_helix_left(distance, 0.0), -bank_angle)
}

pub fn medium_quarter_helix_right_bank(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::banked_curve(&medium_quarter_helix_right(distance, 0.0), bank_angle)
}

pub fn small_turn_left_bank_to_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    let radius = SMALL_TURN_RADIUS;
    let distance = crate::curves::reparameterize(
        1.2051404e-11,
        -1.0873811e-9,
        2.5629598e-8,
        3.909113e-7,
        -2.875509e-5,
        -2.6704837e-4,
        1.2781729e-1,
        distance * 3.3,
    );

    let a = 0.34953994;
    let b = 0.2628325;

    let rot = 0.5 * std::f32::consts::PI;
    let (sin, cos) = (rot * distance).sin_cos();

    let position = glam::Vec3::new(radius * (cos - 1.0), distance * (a * distance + b), radius * sin);
    let tangent = glam::Vec3::new(-rot * radius * sin, 2.0 * a * distance + b, rot * radius * cos).normalize();
    let binormal = glam::Vec3::new(0.0, 1.0, 0.0).cross(tangent).normalize();
    let normal = tangent.cross(binormal);

    let point = crate::track_sections::TrackPoint {
        position,
        tangent,
        normal,
        binormal,
    };

    crate::curves::banked_curve(&point, -bank_angle * (1.0 - distance))
}

pub fn small_turn_right_bank_to_gentle(distance: f32, bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(small_turn_left_bank_to_gentle(distance, bank_angle))
}

pub fn barrel_roll_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let radius = 7.0 * CLEARANCE_HEIGHT / 6.0;
    crate::curves::roll_left(BARREL_ROLL_LENGTH, radius, distance)
}

pub fn barrel_roll_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(barrel_roll_left(distance, 0.0))
}

pub fn inline_twist_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let radius = CLEARANCE_HEIGHT / 6.0;
    crate::curves::roll_left(BARREL_ROLL_LENGTH, radius, distance)
}

pub fn inline_twist_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(inline_twist_left(distance, 0.0))
}

pub fn half_loop(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    if distance < HALF_LOOP_SEGMENT_1_LENGTH {
        crate::curves::plane_curve_vertical(
            &glam::Vec3::new(
                0.0,
                CLEARANCE_HEIGHT * (distance / HALF_LOOP_SEGMENT_1_LENGTH),
                0.5 * (distance / HALF_LOOP_SEGMENT_1_LENGTH),
            ),
            &glam::Vec3::new(0.0, 2.0 * CLEARANCE_HEIGHT / 1.0, 1.0).normalize(),
        )
    } else if distance < HALF_LOOP_SEGMENT_1_LENGTH + HALF_LOOP_SEGMENT_2_LENGTH {
        crate::curves::cubic_curve_vertical(
            &[
                3.0 - 32.0 * CLEARANCE_HEIGHT / 3.0,
                16.0 * CLEARANCE_HEIGHT - 6.5,
                4.0,
                0.5,
            ],
            &[
                -14.0 * CLEARANCE_HEIGHT / 3.0,
                19.0 * CLEARANCE_HEIGHT / 3.0,
                8.0 * CLEARANCE_HEIGHT,
                CLEARANCE_HEIGHT,
            ],
            2.3570517e-3,
            -1.826351e-2,
            5.676165e-2,
            -8.77873e-2,
            7.4771136e-2,
            4.925161e-3,
            2.3481797e-1,
            distance - HALF_LOOP_SEGMENT_1_LENGTH,
        )
    } else {
        crate::curves::cubic_curve_vertical(
            &[
                0.0,
                -16.0 * CLEARANCE_HEIGHT / 3.0,
                0.0,
                16.0 * CLEARANCE_HEIGHT / 3.0 + 1.0,
            ],
            &[
                -8.0 * CLEARANCE_HEIGHT / 3.0,
                -4.0 * CLEARANCE_HEIGHT / 3.0,
                32.0 * CLEARANCE_HEIGHT / 3.0,
                32.0 * CLEARANCE_HEIGHT / 3.0,
            ],
            4.5068464e-3,
            -2.1917358e-2,
            3.1167552e-2,
            -1.8880242e-2,
            1.5697306e-2,
            2.6624454e-2,
            4.591454e-1,
            distance - (HALF_LOOP_SEGMENT_1_LENGTH + HALF_LOOP_SEGMENT_2_LENGTH),
        )
    }
}

pub fn vertical_loop_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    let proj_distance = distance / VERTICAL_LOOP_FACTOR;

    let mut point = if proj_distance < VERTICAL_LOOP_SEGMENT_1_LENGTH {
        crate::curves::plane_curve_vertical(
            &glam::Vec3::new(
                0.0,
                CLEARANCE_HEIGHT * (proj_distance / VERTICAL_LOOP_SEGMENT_1_LENGTH),
                0.5 * (proj_distance / VERTICAL_LOOP_SEGMENT_1_LENGTH),
            ),
            &glam::Vec3::new(0.0, 2.0 * CLEARANCE_HEIGHT, 1.0).normalize(),
        )
    } else if proj_distance < VERTICAL_LOOP_SEGMENT_2_LENGTH {
        // reparameterization coefficients from the original
        let distance = (proj_distance - VERTICAL_LOOP_SEGMENT_1_LENGTH) * 3.3;
        let distance = 3.6742346 * distance / 3.3;
        crate::curves::cubic_curve_vertical(
            &[1.0, -3.5, 4.0, 0.5],
            &[
                -20.0 * CLEARANCE_HEIGHT / 3.0,
                26.0 * CLEARANCE_HEIGHT / 3.0,
                8.0 * CLEARANCE_HEIGHT,
                CLEARANCE_HEIGHT,
            ],
            2.6073596e-7,
            -7.4230593e-6,
            8.476578e-5,
            -4.8168618e-4,
            1.5074167e-3,
            3.6482676e-4,
            6.399354e-2,
            distance,
        )
    } else {
        crate::curves::cubic_curve_vertical(
            &[0.0, -1.0, 0.0, 2.0],
            &[
                -11.0 * CLEARANCE_HEIGHT / 3.0,
                9.0 * CLEARANCE_HEIGHT / 6.0,
                8.0 * CLEARANCE_HEIGHT,
                33.0 * CLEARANCE_HEIGHT / 3.0,
            ],
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            (proj_distance - VERTICAL_LOOP_SEGMENT_2_LENGTH)
                / (VERTICAL_LOOP_LENGTH / VERTICAL_LOOP_FACTOR - VERTICAL_LOOP_SEGMENT_2_LENGTH),
        )
    };

    point.position.x -= 0.5 * distance / VERTICAL_LOOP_LENGTH;

    point.tangent.x -= 0.5 * VERTICAL_LOOP_FACTOR / (VERTICAL_LOOP_LENGTH * 3.3);
    point.tangent = point.tangent.normalize();

    point.normal = glam::Vec3::new(0.0, point.tangent.z, -point.tangent.y).normalize();
    point.binormal = point.normal.cross(point.tangent);

    point
}

pub fn vertical_loop_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(vertical_loop_left(distance, 0.0))
}

pub fn quarter_loop(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::cubic_curve_vertical(
        &[
            5.0 - 64.0 * CLEARANCE_HEIGHT / 3.0,
            -7.5 + 64.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
            0.0,
        ],
        &[
            -22.0 * CLEARANCE_HEIGHT / 3.0,
            CLEARANCE_HEIGHT / 3.0,
            64.0 * CLEARANCE_HEIGHT / 3.0,
            0.0,
        ],
        6.498695e-6,
        -9.73333e-5,
        4.5362636e-4,
        -8.8584475e-4,
        1.6349939e-3,
        -1.7888641e-3,
        2.2983077e-1,
        distance,
    )
}

pub fn corkscrew_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(corkscrew_right(distance, 0.0))
}

pub fn corkscrew_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    if distance < CORKSCREW_SEGMENT_1_LENGTH {
        crate::curves::bezier3d(
            &[0.31237242, -0.16237243, 0.0, 0.0],
            &[-0.4762897, 1.326807, 0.0, 0.0],
            &[0.16237243, -1.0623724, 2.25, 0.0],
            &[0.104345, -0.906517, 0.5, 0.121773],
            8.822436e-8,
            2.23255e-6,
            -4.1811447e-5,
            6.871737e-5,
            1.08563628e-04,
            8.307646e-3,
            1.4426322e-1,
            distance * 3.3,
        )
    } else {
        crate::curves::bezier3d(
            &[0.16237243, 0.57525516, 0.6123724, 0.15],
            &[-0.4762897, 0.10206212, 1.2247448, 0.8505173],
            &[0.31237242, -0.77474487, 0.6123724, 1.35],
            &[0.729345, -0.031517, -1.0, 0.180399],
            -8.843227e-7,
            1.946593e-5,
            -1.4881507e-4,
            4.7228433e-4,
            -1.5238298e-3,
            4.0206634e-3,
            1.8354385e-1,
            (distance - CORKSCREW_SEGMENT_1_LENGTH) * 3.3,
        )
    }
}

pub fn large_corkscrew_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(large_corkscrew_right(distance, 0.0))
}

pub fn large_corkscrew_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    if distance < LARGE_CORKSCREW_SEGMENT_LENGTH {
        crate::curves::bezier3d(
            &[0.28291696, 0.06708303, 0.0, 0.0],
            &[-0.7482803, 2.007046, 0.0, 0.0],
            &[0.13291697, -1.432917, 3.45, 0.0],
            &[0.474996, -0.837494, 0.250000, -0.033411],
            -9.074917e-5,
            7.445654e-4,
            -1.8782483e-3,
            6.060894e-4,
            1.0376404e-3,
            3.5150442e-2,
            2.8982997e-1,
            distance,
        )
    } else {
        crate::curves::bezier3d(
            &[0.13291697, 1.0341657, 0.98291695, 0.35],
            &[-0.7482803, 0.23779514, 1.7692506, 1.2587655],
            &[0.28291696, -0.91583425, 0.98291695, 2.15],
            &[1.3764, -1.314599, 0.0, -0.028389],
            -9.07493e-5,
            9.485533e-4,
            -3.5093115e-3,
            5.223509e-3,
            -4.2646746e-3,
            -2.0940272e-2,
            4.4444364e-1,
            distance - LARGE_CORKSCREW_SEGMENT_LENGTH,
        )
    }
}

pub fn medium_half_loop_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    if distance < 0.001 {
        return gentle(0.0, 0.0);
    }

    let proj_distance = distance / MEDIUM_HALF_LOOP_FACTOR;

    let mut point = if proj_distance < MEDIUM_HALF_LOOP_SEGMENT_1_LENGTH {
        crate::curves::cubic_curve_vertical(
            &[1.2, -5.3, 7.0, 0.0],
            &[
                -22.0 * CLEARANCE_HEIGHT / 3.0,
                28.0 * CLEARANCE_HEIGHT / 3.0,
                14.0 * CLEARANCE_HEIGHT,
                0.0,
            ],
            6.510708e-5,
            -8.7324856e-4,
            4.688747e-3,
            -1.2421915e-2,
            1.7923763e-2,
            -1.2109005e-3,
            1.3466427e-1,
            proj_distance,
        )
    } else {
        crate::curves::cubic_curve_vertical(
            &[0.65, -2.55, 0.0, 2.9],
            &[
                -56.0 * CLEARANCE_HEIGHT / 3.0 + 3.15,
                28.0 * CLEARANCE_HEIGHT - 1.0 * 6.3,
                3.15,
                16.0 * CLEARANCE_HEIGHT,
            ],
            3.056875e-5,
            -3.1669016e-4,
            1.0755464e-3,
            -1.0474785e-3,
            -4.7734613e-3,
            1.8619826e-2,
            3.1746858e-1,
            proj_distance - MEDIUM_HALF_LOOP_SEGMENT_1_LENGTH,
        )
    };

    point.position.x -= distance / MEDIUM_HALF_LOOP_LENGTH;

    point.tangent.x -= MEDIUM_HALF_LOOP_FACTOR / (MEDIUM_HALF_LOOP_LENGTH * 3.3);
    point.tangent = point.tangent.normalize();

    point.normal = glam::Vec3::new(0.0, point.tangent.z, -point.tangent.y).normalize();
    point.binormal = point.normal.cross(point.tangent);

    point
}

pub fn medium_half_loop_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(medium_half_loop_left(distance, 0.0))
}

pub fn large_half_loop_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    if distance < 0.001 {
        return gentle(0.0, 0.0);
    }

    let proj_distance = distance / LARGE_HALF_LOOP_FACTOR;

    let mut point = if proj_distance < LARGE_HALF_LOOP_SEGMENT_1_LENGTH {
        gentle(proj_distance, 0.0)
    } else if proj_distance < LARGE_HALF_LOOP_SEGMENT_2_LENGTH {
        crate::curves::cubic_curve_vertical(
            &[-3.6, 4.65, 1.5, 1.5],
            &[-0.122179694, 3.252083, 0.6123724, 0.6123724],
            2.0991832e-4,
            -3.942224e-3,
            3.0215306e-2,
            -1.2230973e-1,
            2.84331e-1,
            -3.9744362e-1,
            5.3066623e-1,
            proj_distance - LARGE_HALF_LOOP_SEGMENT_1_LENGTH,
        )
    } else {
        crate::curves::cubic_curve_vertical(
            &[2.6, -4.65, 0.0, 4.05],
            &[1.1010206, -4.6515307, 6.0, 4.3546486],
            8.508348e-4,
            -8.953556e-3,
            3.7601534e-2,
            -7.9522476e-2,
            8.851718e-2,
            -2.1452371e-2,
            1.740928e-1,
            proj_distance - LARGE_HALF_LOOP_SEGMENT_2_LENGTH,
        )
    };

    point.position.x -= proj_distance / LARGE_HALF_LOOP_LENGTH;

    point.tangent.x -= 0.100688085;
    point.tangent = point.tangent.normalize();

    point.normal = glam::Vec3::new(0.0, point.tangent.z, -point.tangent.y).normalize();
    point.binormal = point.normal.cross(point.tangent);

    point
}

pub fn large_half_loop_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(large_half_loop_left(distance, 0.0))
}

pub fn zero_g_roll_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::PI;

    if distance < 0.001 {
        return gentle(0.0, 0.0);
    }

    let roll_rate_final = 0.75 * PI / 3.0;
    let roll_rate_initial = 0.5 * PI / 3.0;

    let a = (roll_rate_final + roll_rate_initial - 2.0 * PI / ZERO_G_ROLL_BASE_LENGTH)
        / (ZERO_G_ROLL_BASE_LENGTH * ZERO_G_ROLL_BASE_LENGTH);
    let b = (3.0 * PI / ZERO_G_ROLL_BASE_LENGTH - 2.0 * roll_rate_initial - roll_rate_final) / ZERO_G_ROLL_BASE_LENGTH;
    let c = roll_rate_initial;

    crate::curves::zero_g_roll(
        7.0 * CLEARANCE_HEIGHT / 6.0,
        &[-0.5, -1.5, 5.0, 0.0],
        &[
            4.0 * CLEARANCE_HEIGHT,
            -11.0 * CLEARANCE_HEIGHT,
            10.0 * CLEARANCE_HEIGHT,
            0.0,
        ],
        &[a, b, c, 0.0],
        1.1611509e-2,
        -1.1114208e-1,
        4.179097e-1,
        -7.749352e-1,
        7.3424e-1,
        -3.081927e-1,
        2.3402834e-1,
        crate::curves::reparameterize(
            -2.202892e-3,
            2.5988393e-2,
            -1.1924088e-1,
            2.694486e-1,
            -3.1118575e-1,
            1.5233602e-1,
            9.620004e-1,
            distance,
        ),
    )
}

pub fn zero_g_roll_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(zero_g_roll_left(distance, 0.0))
}

pub fn large_zero_g_roll_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::PI;

    let roll_rate_final = 0.85 * PI / 3.0;
    let roll_rate_initial = 0.15 * PI / 3.0;

    let a = (roll_rate_final + roll_rate_initial - 2.0 * PI / LARGE_ZERO_G_ROLL_BASE_LENGTH)
        / (LARGE_ZERO_G_ROLL_BASE_LENGTH * LARGE_ZERO_G_ROLL_BASE_LENGTH);
    let b = (3.0 * PI / LARGE_ZERO_G_ROLL_BASE_LENGTH - 2.0 * roll_rate_initial - roll_rate_final)
        / LARGE_ZERO_G_ROLL_BASE_LENGTH;
    let c = roll_rate_initial;

    crate::curves::zero_g_roll(
        4.0 * CLEARANCE_HEIGHT / 6.0,
        &[0.0, 1.0, 3.0, 0.0],
        &[-8.0 * CLEARANCE_HEIGHT, 0.0, 24.0 * CLEARANCE_HEIGHT, 0.0],
        &[a, b, c, 0.0],
        1.9446561e-7,
        -1.2736247e-5,
        1.2071638e-4,
        -4.8401125e-4,
        1.9343582e-3,
        -3.2755677e-3,
        1.7421886e-1,
        crate::curves::reparameterize(
            1.6635007e-6,
            1.7291109e-6,
            -1.9070526e-4,
            8.341236e-4,
            -1.8818292e-3,
            1.209923e-3,
            9.9934304e-1,
            distance,
        ),
    )
}

pub fn large_zero_g_roll_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis(large_zero_g_roll_left(distance, 0.0))
}

pub fn dive_loop_45_left(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::bezier3d(
        &[1.0, -3.0, 3.0, 0.0],
        &[-2.177324, 0.81649655, 4.898979, 0.0],
        &[0.5, 0.0, 3.0, 0.0],
        &[2.2951088, -3.7003796, 6.238097e-1, 7.814611e-1],
        -6.201532e-6,
        7.9620135e-5,
        -3.9349165e-4,
        1.07427e-3,
        -1.0898163e-3,
        3.5191467e-3,
        1.5417333e-1,
        distance,
    )
}

pub fn dive_loop_45_right(distance: f32, _bank_angle: f32) -> crate::track_sections::TrackPoint {
    crate::curves::flip_x_axis_diagonal(dive_loop_45_left(distance, 0.0))
}
