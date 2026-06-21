mod atlas;
mod ride_desc;
mod ride_object;

struct VehicleRotation {
    pitch: f32,
    roll: f32,
    yaw: f32,
}

impl VehicleRotation {
    const fn new(pitch: f32, roll: f32, yaw: f32) -> Self {
        Self { pitch, roll, yaw }
    }
}

fn corkscrew_right_pitch(angle: f32) -> f32 {
    -(-angle.sin() / 2.0_f32.sqrt()).asin()
}
fn corkscrew_right_roll(angle: f32) -> f32 {
    -(angle.sin() / 2.0_f32.sqrt()).atan2(angle.cos())
}
fn corkscrew_right_yaw(angle: f32) -> f32 {
    (0.5 * (1.0 - angle.cos())).atan2(1.0 - 0.5 * (1.0 - angle.cos()))
}
fn corkscrew_left_pitch(angle: f32) -> f32 {
    -corkscrew_right_pitch(-angle)
}
fn corkscrew_left_roll(angle: f32) -> f32 {
    -corkscrew_right_roll(angle)
}
fn corkscrew_left_yaw(angle: f32) -> f32 {
    -corkscrew_right_yaw(angle)
}

#[derive(Clone, Copy, Debug)]
struct PitchRollYaw {
    pitch: f32,
    roll: f32,
    yaw: f32,
}

struct VehicleAngles {
    pitch_flat: f32,
    pitch_gentle: f32,
    pitch_steep: f32,
    pitch_vertical: f32,
    pitch_flat_to_gentle: f32,
    pitch_gentle_to_steep: f32,
    pitch_steep_to_vertical: f32,
    pitch_gentle_diag: f32,
    pitch_steep_diag: f32,
    pitch_flat_to_gentle_diag: f32,
    roll_bank: f32,
    roll_flat_to_bank: f32,
    corkscrew_right_up: [PitchRollYaw; 5],
    corkscrew_right_down: [PitchRollYaw; 5],
    corkscrew_left_up: [PitchRollYaw; 5],
    corkscrew_left_down: [PitchRollYaw; 5],
}

impl VehicleAngles {
    fn new() -> Self {
        const FRAC_PI_12: f32 = std::f32::consts::PI / 12.0;
        const SQRT_6: f32 = 2.4494898;
        const TILE_SLOPE: f32 = 1.0 / SQRT_6;

        const CORKSCREW_ANGLES: [f32; 5] = [
            2.0 * FRAC_PI_12,
            4.0 * FRAC_PI_12,
            std::f32::consts::FRAC_PI_2,
            8.0 * FRAC_PI_12,
            10.0 * FRAC_PI_12,
        ];

        let pitch_gentle = TILE_SLOPE.atan();
        let pitch_steep = (4.0 * TILE_SLOPE).atan();
        let pitch_vertical = std::f32::consts::FRAC_PI_2;
        let pitch_gentle_diag = (TILE_SLOPE * std::f32::consts::FRAC_1_SQRT_2).atan();

        let roll_bank = std::f32::consts::FRAC_PI_4;

        let corkscrew_right_up: [PitchRollYaw; 5] = CORKSCREW_ANGLES
            .iter()
            .map(|angle| PitchRollYaw {
                pitch: corkscrew_right_pitch(*angle),
                roll: corkscrew_right_roll(*angle),
                yaw: corkscrew_right_yaw(*angle),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let corkscrew_right_down: [PitchRollYaw; 5] = CORKSCREW_ANGLES
            .iter()
            .map(|angle| PitchRollYaw {
                pitch: corkscrew_right_pitch(-*angle),
                roll: corkscrew_right_roll(-*angle),
                yaw: corkscrew_right_yaw(-*angle),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let corkscrew_left_up: [PitchRollYaw; 5] = CORKSCREW_ANGLES
            .iter()
            .map(|angle| PitchRollYaw {
                pitch: corkscrew_left_pitch(*angle),
                roll: corkscrew_left_roll(*angle),
                yaw: corkscrew_left_yaw(*angle),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let corkscrew_left_down: [PitchRollYaw; 5] = CORKSCREW_ANGLES
            .iter()
            .map(|angle| PitchRollYaw {
                pitch: corkscrew_left_pitch(-*angle),
                roll: corkscrew_left_roll(-*angle),
                yaw: corkscrew_left_yaw(-*angle),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            pitch_flat: 0.0,
            pitch_gentle,
            pitch_steep,
            pitch_vertical,
            pitch_flat_to_gentle: pitch_gentle / 2.0,
            pitch_gentle_to_steep: (pitch_gentle + pitch_steep) / 2.0,
            pitch_steep_to_vertical: (pitch_steep + pitch_vertical) / 2.0,
            pitch_gentle_diag,
            pitch_steep_diag: (4.0 * TILE_SLOPE * std::f32::consts::FRAC_1_SQRT_2).atan(),
            pitch_flat_to_gentle_diag: pitch_gentle_diag / 2.0,
            roll_bank,
            roll_flat_to_bank: roll_bank / 2.0,
            corkscrew_right_up,
            corkscrew_right_down,
            corkscrew_left_up,
            corkscrew_left_down,
        }
    }
}

fn add_rotations_to_list(rots: &mut Vec<VehicleRotation>, count: i32, pitch: f32, roll: f32, yaw: f32) {
    for i in 0..count {
        let yaw = yaw + ((2.0 * i as f32 * std::f32::consts::PI) / count as f32);
        rots.push(VehicleRotation::new(pitch, roll, yaw));
    }
}

fn list_vehicle_rotations(
    sprite_groups: &openrct2::objects::ride::SpriteGroups,
    angles: &VehicleAngles,
) -> Vec<VehicleRotation> {
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

    const FRAC_PI_12: f32 = PI / 12.0;

    let VehicleAngles {
        pitch_flat,
        pitch_gentle,
        pitch_steep,
        pitch_vertical,
        pitch_flat_to_gentle,
        pitch_gentle_to_steep,
        pitch_steep_to_vertical,
        pitch_gentle_diag,
        pitch_steep_diag,
        pitch_flat_to_gentle_diag,
        roll_bank,
        roll_flat_to_bank,
        corkscrew_right_up,
        corkscrew_right_down,
        corkscrew_left_up,
        corkscrew_left_down,
    } = *angles;

    let mut rots = Vec::with_capacity(1024);
    if let Some(count) = sprite_groups.slope_flat {
        add_rotations_to_list(&mut rots, count, pitch_flat, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes12 {
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes25 {
        add_rotations_to_list(&mut rots, count, pitch_gentle, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes42 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes60 {
        add_rotations_to_list(&mut rots, count, pitch_steep, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_steep, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes75 {
        add_rotations_to_list(&mut rots, count, pitch_steep_to_vertical, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_steep_to_vertical, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes90 {
        add_rotations_to_list(&mut rots, count, pitch_vertical, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_vertical, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes_loop {
        add_rotations_to_list(&mut rots, count, pitch_vertical + FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_vertical - FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_vertical + 2.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_vertical - 2.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_vertical + 3.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_vertical - 3.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_vertical + 4.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_vertical - 4.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_vertical + 5.0 * FRAC_PI_12, 0.0, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_vertical - 5.0 * FRAC_PI_12, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slope_inverted {
        add_rotations_to_list(&mut rots, count, PI, 0.0, 0.0);
    }
    if let Some(count) = sprite_groups.slopes8 {
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle_diag, 0.0, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle_diag, 0.0, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.slopes16 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_diag, 0.0, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_diag, 0.0, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.slopes50 {
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, 0.0, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, 0.0, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.flat_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_flat, roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -roll_flat_to_bank, 0.0);
    }
    if let Some(count) = sprite_groups.flat_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_flat, roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -roll_bank, 0.0);
    }
    if let Some(count) = sprite_groups.flat_banked67 {
        add_rotations_to_list(&mut rots, count, pitch_flat, 3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -3.0 * FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.flat_banked90 {
        add_rotations_to_list(&mut rots, count, pitch_flat, FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -FRAC_PI_2, 0.0);
    }
    if let Some(count) = sprite_groups.inline_twists {
        add_rotations_to_list(&mut rots, count, pitch_flat, 5.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -5.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, 3.0 * FRAC_PI_4, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -3.0 * FRAC_PI_4, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, 7.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat, -7.0 * FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes12_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle, roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle, -roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle, roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle, -roll_flat_to_bank, 0.0);
    }
    if let Some(count) = sprite_groups.slopes8_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle_diag, roll_flat_to_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle_diag, -roll_flat_to_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle_diag, roll_flat_to_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle_diag, -roll_flat_to_bank, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.slopes25_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_gentle, roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, roll_flat_to_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -roll_flat_to_bank, 0.0);
    }
    if let Some(count) = sprite_groups.slopes8_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle_diag, roll_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle_diag, -roll_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle_diag, roll_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle_diag, -roll_bank, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.slopes16_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_diag, roll_flat_to_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, pitch_gentle_diag, -roll_flat_to_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_diag, roll_flat_to_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_diag, -roll_flat_to_bank, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.slopes16_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_diag, roll_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, pitch_gentle_diag, -roll_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_diag, roll_bank, FRAC_PI_4);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_diag, -roll_bank, FRAC_PI_4);
    }
    if let Some(count) = sprite_groups.slopes25_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_gentle, roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -roll_bank, 0.0);
    }
    if let Some(count) = sprite_groups.slopes12_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle, roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_flat_to_gentle, -roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle, roll_bank, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_flat_to_gentle, -roll_bank, 0.0);
    }
    if let Some(count) = sprite_groups.slopes25_banked67 {
        add_rotations_to_list(&mut rots, count, pitch_gentle, 3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, 3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -3.0 * FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes25_banked90 {
        add_rotations_to_list(&mut rots, count, pitch_gentle, FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -FRAC_PI_2, 0.0);
    }
    if let Some(count) = sprite_groups.slopes25_inline_twists {
        add_rotations_to_list(&mut rots, count, pitch_gentle, 5.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -5.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, 5.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -5.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, 3.0 * FRAC_PI_4, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -3.0 * FRAC_PI_4, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, 3.0 * FRAC_PI_4, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -3.0 * FRAC_PI_4, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, 7.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle, -7.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, 7.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle, -7.0 * FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes42_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, -FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, -FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes42_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, 2.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, -2.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, 2.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, -2.0 * FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes42_banked67 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, 3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, -3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, 3.0 * FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, -3.0 * FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes42_banked90 {
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_gentle_to_steep, -FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, FRAC_PI_2, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_gentle_to_steep, -FRAC_PI_2, 0.0);
    }
    if let Some(count) = sprite_groups.slopes60_banked22 {
        add_rotations_to_list(&mut rots, count, pitch_steep, FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, pitch_steep, -FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_steep, FRAC_PI_8, 0.0);
        add_rotations_to_list(&mut rots, count, -pitch_steep, -FRAC_PI_8, 0.0);
    }
    if let Some(count) = sprite_groups.slopes50_banked45 {
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, FRAC_PI_4, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, -FRAC_PI_4, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, FRAC_PI_4, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, -FRAC_PI_4, FRAC_PI_8);
    }
    if let Some(count) = sprite_groups.slopes50_banked67 {
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, 3.0 * FRAC_PI_8, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, -3.0 * FRAC_PI_8, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, 3.0 * FRAC_PI_8, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, -3.0 * FRAC_PI_8, FRAC_PI_8);
    }
    if let Some(count) = sprite_groups.slopes50_banked90 {
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, FRAC_PI_2, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, pitch_steep_diag, -FRAC_PI_2, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, FRAC_PI_2, FRAC_PI_8);
        add_rotations_to_list(&mut rots, count, -pitch_steep_diag, -FRAC_PI_2, FRAC_PI_8);
    }
    if let Some(count) = sprite_groups.corkscrews {
        for angles in corkscrew_right_up {
            add_rotations_to_list(&mut rots, count, angles.pitch, angles.roll, angles.yaw);
        }
        for angles in corkscrew_right_down {
            add_rotations_to_list(&mut rots, count, angles.pitch, angles.roll, angles.yaw);
        }
        for angles in corkscrew_left_up {
            add_rotations_to_list(&mut rots, count, angles.pitch, angles.roll, angles.yaw);
        }
        for angles in corkscrew_left_down {
            add_rotations_to_list(&mut rots, count, angles.pitch, angles.roll, angles.yaw);
        }
    }
    rots
}

fn add_models_to_scene<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &[ride_desc::ModelRenderDesc<'a>],
    mesh_type: renderer::MeshType,
) -> anyhow::Result<()> {
    for model in models {
        scene.add_model(
            model.model,
            &glam::Affine3::from_rotation_translation(model.rotation, model.translation),
            mesh_type,
            None,
        )?;
    }
    Ok(())
}

fn add_restraint_models_to_scene<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &[ride_desc::ModelRenderDesc<'a>],
    mesh_type: renderer::MeshType,
    frame: usize,
) -> anyhow::Result<()> {
    for model in models {
        scene.add_model(
            model.model,
            &glam::Affine3::from_rotation_translation(
                model.restraint_rotations[frame],
                model.restraint_translations[frame],
            ),
            mesh_type,
            None,
        )?;
    }
    Ok(())
}

fn render_rotation(
    scene: &renderer::Scene,
    mesh_types: &[renderer::MeshType],
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    rotation: &VehicleRotation,
) -> renderer::image::IndexedImage {
    let view_rotation = glam::Mat4::from_euler(glam::EulerRot::YXZ, rotation.yaw, -rotation.pitch, rotation.roll);

    let camera = camera * view_rotation;

    let view_rotation_inverse = view_rotation.inverse();
    let lights = lights.iter().map(|x| x.transform(&view_rotation_inverse)).collect::<Vec<_>>();

    const EDGE_DISTANCE: f32 = 0.088388346; // ?
    let framebuffer = renderer::render_scene(scene, mesh_types, &camera, &lights, 4, 4, EDGE_DISTANCE);
    framebuffer.into_cropped_indexed_image(true)
}

fn render_vehicle(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    angles: &VehicleAngles,
    vehicle: &ride_desc::VehicleRenderDesc,
) -> anyhow::Result<Vec<renderer::image::IndexedImage>> {
    use rayon::prelude::*;

    let rotations = list_vehicle_rotations(&vehicle.sprite_groups, angles);
    let restraint_rotations = vehicle.sprite_groups.restraint_animation.map(|rotation_count| {
        let mut restraint_rotations = Vec::with_capacity(rotation_count.try_into().unwrap_or_default());
        add_rotations_to_list(&mut restraint_rotations, rotation_count, 0.0, 0.0, 0.0);
        restraint_rotations
    });

    let mut images = Vec::new();
    {
        let mut scene = renderer::SceneBuilder::new(render_device)?;
        add_models_to_scene(&mut scene, &vehicle.models, renderer::MeshType::Normal)?;
        let (scene, mesh_types) = scene.build();

        images.par_extend(rotations.par_iter().map(|x| render_rotation(&scene, &mesh_types, camera, lights, x)));

        if let Some(ref restraint_rotations) = restraint_rotations {
            for frame in 0..3 {
                let mut scene = renderer::SceneBuilder::new(render_device)?;
                add_restraint_models_to_scene(&mut scene, &vehicle.models, renderer::MeshType::Normal, frame)?;
                let (scene, mesh_types) = scene.build();
                for rotation in restraint_rotations {
                    images.push(render_rotation(&scene, &mesh_types, camera, lights, rotation));
                }
            }
        }
    }

    for (rider_index, model_desc) in vehicle.riders.iter().enumerate() {
        use renderer::MeshType;

        let mut scene = renderer::SceneBuilder::new(render_device)?;
        add_models_to_scene(&mut scene, std::slice::from_ref(model_desc), renderer::MeshType::Normal)?;
        add_models_to_scene(&mut scene, &vehicle.models, MeshType::Mask)?;
        add_models_to_scene(&mut scene, &vehicle.riders[0..rider_index], MeshType::Mask)?;
        let (scene, mesh_types) = scene.build();

        images.par_extend(rotations.par_iter().map(|x| render_rotation(&scene, &mesh_types, camera, lights, x)));

        if let Some(ref restraint_rotations) = restraint_rotations {
            for frame in 0..3 {
                let mut scene = renderer::SceneBuilder::new(render_device)?;
                add_restraint_models_to_scene(&mut scene, std::slice::from_ref(model_desc), MeshType::Normal, frame)?;
                add_restraint_models_to_scene(&mut scene, &vehicle.models, MeshType::Mask, frame)?;
                add_restraint_models_to_scene(&mut scene, &vehicle.riders[0..rider_index], MeshType::Mask, frame)?;
                let (scene, mesh_types) = scene.build();

                for rotation in restraint_rotations {
                    images.push(render_rotation(&scene, &mesh_types, camera, lights, rotation));
                }
            }
        }
    }

    Ok(images)
}

fn render(
    vehicles: &[ride_desc::VehicleRenderType],
    lights: &[renderer::Light],
) -> anyhow::Result<Vec<Vec<renderer::image::IndexedImage>>> {
    use anyhow::Context as _;

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    let camera = glam::Mat4::from_mat3(
        glam::Mat3::from_cols(
            glam::Vec3::new(32.0, 0.0, 32.0),
            glam::Vec3::new(16.0, -16.0 * 6.0_f32.sqrt(), -16.0),
            glam::Vec3::new(-16.0 * 3.0_f32.sqrt(), -16.0 * 2.0_f32.sqrt(), 16.0 * 3.0_f32.sqrt()),
        )
        .transpose(),
    );

    let angles = VehicleAngles::new();

    let mut images = Vec::new();

    for vehicle in vehicles {
        match vehicle {
            ride_desc::VehicleRenderType::Regular(vehicle) => {
                let car_images = render_vehicle(&render_device, &camera, lights, &angles, vehicle)?;
                images.push(car_images);
            }
            ride_desc::VehicleRenderType::Invisible => {
                images.push(vec![renderer::image::IndexedImage::new(1, 1)]);
            }
        }
    }

    Ok(images)
}

fn create_parkobj(
    output_directory: &std::path::Path,
    parkobj_path: &std::path::Path,
    file_paths: &[std::path::PathBuf],
) -> anyhow::Result<()> {
    use std::io::Write as _;

    let file = std::fs::File::create(parkobj_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for file_path in file_paths {
        let file_name = file_path.strip_prefix(output_directory)?;
        let file_bytes = std::fs::read(file_path)?;
        zip.start_file_from_path(file_name, options)?;
        zip.write_all(&file_bytes)?;
    }

    Ok(())
}

#[derive(Copy, Clone)]
pub enum ImageOutputType {
    Dat,
    Atlas,
}

pub fn make_vehicle(
    ride_description_path: &std::path::Path,
    intermediate_output_directory: &std::path::Path,
    parkobj_output_directory: &std::path::Path,
    image_output_type: ImageOutputType,
) -> anyhow::Result<()> {
    use anyhow::Context as _;
    use openrct2::objects::image as object_image;

    let ride_description_path = ride_description_path
        .canonicalize()
        .with_context(|| format!("Invalid file path {}", ride_description_path.display()))?;

    let base_directory = ride_description_path
        .parent()
        .with_context(|| format!("Could not get parent directory of {}", ride_description_path.display()))?;

    let ride_description = ride_desc::Ride::load(&ride_description_path)?;

    let output_directory = intermediate_output_directory.join(&ride_description.id);
    std::fs::create_dir_all(&output_directory)
        .with_context(|| format!("Could not create directory {}", output_directory.display()))?;

    let models = ride_description.load_models(base_directory)?;
    let lights = ride_description.get_lights();
    let vehicles = ride_description.get_vehicle_render_descs(&models)?;

    let images = render(&vehicles, &lights)?;

    let preview_image = if let Some(ref preview_file_path) = ride_description.preview {
        let preview_file_path = base_directory.join(preview_file_path);
        let mut preview_image =
            renderer::image::IndexedImage::load(&preview_file_path, &renderer::palette::PALETTE_FLAT)?;
        preview_image.water_colours_to_regular_colours();
        preview_image
    } else {
        renderer::image::IndexedImage::new(1, 1)
    };

    let mut file_paths = Vec::new();

    let object_images = match image_output_type {
        ImageOutputType::Dat => {
            let mut archive = rct::csg::Archive::with_capacity(images.len() + 3);

            archive.add_sprite(preview_image.as_raw(), preview_image.width(), preview_image.height(), 0, 0);

            // previews 2 and 3, currently unimplemented
            let empty_image = renderer::image::IndexedImage::new(1, 1);
            archive.add_sprite(empty_image.as_raw(), empty_image.width(), empty_image.height(), 0, 0);
            archive.add_sprite(empty_image.as_raw(), empty_image.width(), empty_image.height(), 0, 0);

            for images in &images {
                for image in images {
                    let sprite = rct::csg::EncodedSprite::new(image.as_raw(), image.width(), image.height());
                    archive.add_encoded_sprite(&sprite, image.offset.x, image.offset.y);
                }
            }

            let file_path = output_directory.join("images").with_extension("dat");
            archive.save(&file_path)?;
            file_paths.push(file_path);

            let lgx_string = format!("$LGX:images.dat[0..{}]", archive.len() - 1);
            vec![object_image::Image::String(lgx_string)]
        }
        ImageOutputType::Atlas => {
            let images_directory = output_directory.join("images");
            std::fs::create_dir_all(&images_directory)
                .with_context(|| format!("Could not create directory {}", images_directory.display()))?;

            let preview_output_file_path = images_directory.join("preview").with_extension("png");
            preview_image
                .save(&preview_output_file_path)
                .with_context(|| format!("Could not save preview image {}", preview_output_file_path.display()))?;

            file_paths.push(preview_output_file_path);

            let mut object_images = vec![
                object_image::Image::ImageFile(object_image::ImageFile {
                    path: "images/preview.png".to_string(),
                    x: Some(0),
                    y: Some(0),
                    format: Some(object_image::Format::Raw),
                    palette: Some(object_image::PaletteType::Keep),
                    ..Default::default()
                }),
                object_image::Image::String("".to_owned()),
                object_image::Image::String("".to_owned()),
            ];

            for (i, images) in images.iter().enumerate() {
                let image_count = images.len().try_into().unwrap_or(32);
                let atlas = atlas::create_atlas(images, std::cmp::min(image_count, 32));
                let file_path = images_directory.join(format!("car_{i}")).with_extension("png");
                atlas.image.save(&file_path)?;

                for (image, coord) in images.iter().zip(atlas.coords.iter()) {
                    object_images.push(object_image::Image::ImageFile(object_image::ImageFile {
                        path: format!("images/car_{i}.png"),
                        x: Some(image.offset.x),
                        y: Some(image.offset.y),
                        src_x: Some(coord.x),
                        src_y: Some(coord.y),
                        src_width: Some(image.width().into()),
                        src_height: Some(image.height().into()),
                        format: None,
                        palette: Some(object_image::PaletteType::Keep),
                    }));
                }

                file_paths.push(file_path);
            }

            object_images
        }
    };

    let object = ride_object::create_ride_object(&ride_description, object_images);
    let object_json_file_path = output_directory.join("object").with_extension("json");
    ride_object::save_ride_object(&object, &object_json_file_path)?;

    file_paths.push(object_json_file_path);

    let parkobj_path = parkobj_output_directory.join(&ride_description.id).with_added_extension("parkobj");
    create_parkobj(&output_directory, &parkobj_path, &file_paths)
        .with_context(|| format!("Could not create parkobj file {}", parkobj_path.display()))?;

    Ok(())
}
