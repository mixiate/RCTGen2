mod ride_object;

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum RideFlag {
    NoCollisionCrashes,
    RiderControlsSpeed,
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum SpriteGroup {
    Flat,
    GentleSlopes,
    SteepSlopes,
    VerticalSlopes,
    Diagonals,
    BankedTurns,
    InlineTwists,
    SlopeBankTransition,
    DiagonalBankTransition,
    SlopedBankTransition,
    BankedSlopedTurns,
    BankedSlopeTransition,
    Corkscrews,
    ZeroGRolls,
    DiagonalSlopedBankTransition,
    DiveLoops,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum RunningSound {
    WoodenOld = 1,
    Wooden = 54,
    Steel = 2,
    SteelSmooth = 57,
    Train = 31,
    Engine = 21,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum SecondarySound {
    Scream1 = 0,
    Scream2 = 1,
    Scream3 = 2,
    Whistle = 3,
    Bell = 4,
}

#[derive(Debug, serde::Deserialize)]
struct Configuration {
    default: i32,
    front: Option<i32>,
    rear: Option<i32>,
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum VehicleFlag {
    SecondaryRemap,
    TertiaryRemap,
    RidersScream,
    RestraintAnimation,
}

#[derive(Debug, serde::Deserialize)]
struct ModelDesc {
    mesh_index: usize,
    position: [f32; 3],
    orientation: Vec<[f32; 3]>,
}

struct ModelTransform<'a> {
    model: &'a renderer::model::Model,
    translation: glam::Vec3,
    rotation: glam::Quat,
}

impl ModelDesc {
    fn get_model_transform<'a>(
        &self,
        models: &'a [renderer::model::Model],
        frame: usize,
    ) -> anyhow::Result<ModelTransform<'a>> {
        use anyhow::Context as _;

        let model = models.get(self.mesh_index).context(format!("Invalid mesh index {}", self.mesh_index))?;

        let translation = (self.position).into();

        let rotation = self.orientation.get(frame).context(format!("No orientation found for frame {frame}"))?;
        let rotation = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation[0].to_radians(),
            rotation[1].to_radians(),
            rotation[2].to_radians(),
        );

        Ok(ModelTransform {
            model,
            translation,
            rotation,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
struct VehicleDesc {
    spacing: f32,
    mass: i32,
    draw_order: i32,
    flags: Option<std::collections::HashSet<VehicleFlag>>,
    model: Vec<ModelDesc>,
    capacity: Option<i32>,
    riders: Option<Vec<ModelDesc>>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum LightType {
    Diffuse,
    Specular,
}

#[derive(Debug, serde::Deserialize)]
struct LightDesc {
    r#type: LightType,
    shadow: bool,
    direction: [f32; 3],
    strength: f32,
}

#[derive(Debug, serde::Deserialize)]
struct RideDesc {
    id: String,
    original_id: Option<String>,
    name: String,
    description: String,
    capacity: String,
    author: String,
    version: Option<String>,
    preview: Option<std::path::PathBuf>,
    ride_type: ride_object::RideType,
    flags: Option<std::collections::HashSet<RideFlag>>,
    sprites: std::collections::HashSet<SpriteGroup>,
    zero_cars: i32,
    preview_tab_car: i32,
    build_menu_priority: i32,
    running_sound: RunningSound,
    secondary_sound: SecondarySound,
    min_cars_per_train: i32,
    max_cars_per_train: i32,
    configuration: Configuration,
    default_colors: Vec<[ride_object::ColourType; 3]>,
    meshes: Vec<std::path::PathBuf>,
    vehicles: Vec<VehicleDesc>,
    lights: Vec<LightDesc>,
}

struct VehicleRotation {
    count: i32,
    pitch: f32,
    roll: f32,
    yaw: f32,
}

impl VehicleRotation {
    fn new(count: i32, pitch: f32, roll: f32, yaw: f32) -> Self {
        Self {
            count,
            pitch,
            roll,
            yaw,
        }
    }
}

fn render_rotations(
    scene: &renderer::Scene,
    camera: &glam::Mat4,
    lights: &[LightDesc],
    rotation: &VehicleRotation,
) -> Vec<renderer::image::IndexedImage> {
    let mut images = Vec::new();
    for i in 0..rotation.count {
        let yaw = rotation.yaw + ((2.0 * i as f32 * std::f32::consts::PI) / rotation.count as f32);
        let view_rotation = glam::Mat4::from_euler(glam::EulerRot::YZX, yaw, rotation.pitch, rotation.roll);

        let camera = camera * view_rotation;

        let view_rotation_inverse = view_rotation.inverse();
        let lights: Vec<_> = lights
            .iter()
            .map(|x| renderer::Light {
                diffuse_strength: if x.r#type == LightType::Diffuse {
                    x.strength
                } else {
                    0.0
                },
                specular_strength: if x.r#type == LightType::Specular {
                    x.strength
                } else {
                    0.0
                },
                direction: view_rotation_inverse.transform_vector3(x.direction.into()).normalize(),
                shadow: x.shadow,
            })
            .collect();

        let framebuffer = renderer::render_scene(scene, &camera, &lights, 4, 4);

        images.push(framebuffer.into_cropped_indexed_image(true));
    }
    images
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
        const SQRT_6: f32 = 2.449_489_8;
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

fn render_vehicle(
    scene: &renderer::Scene,
    camera: &glam::Mat4,
    lights: &[LightDesc],
    sprite_groups: &std::collections::HashSet<SpriteGroup>,
    angles: &VehicleAngles,
) -> Vec<renderer::image::IndexedImage> {
    use VehicleRotation as VR;
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

    let mut rots = Vec::with_capacity(256);
    if sprite_groups.contains(&SpriteGroup::Flat) {
        rots.push(VR::new(32, pitch_flat, 0.0, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::GentleSlopes) {
        rots.push(VR::new(4, pitch_flat_to_gentle, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_flat_to_gentle, 0.0, 0.0));
        rots.push(VR::new(32, pitch_gentle, 0.0, 0.0));
        rots.push(VR::new(32, -pitch_gentle, 0.0, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::SteepSlopes) {
        rots.push(VR::new(8, pitch_gentle_to_steep, 0.0, 0.0));
        rots.push(VR::new(8, -pitch_gentle_to_steep, 0.0, 0.0));
        rots.push(VR::new(32, pitch_steep, 0.0, 0.0));
        rots.push(VR::new(32, -pitch_steep, 0.0, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::VerticalSlopes) {
        rots.push(VR::new(4, pitch_steep_to_vertical, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_steep_to_vertical, 0.0, 0.0));
        rots.push(VR::new(32, pitch_vertical, 0.0, 0.0));
        rots.push(VR::new(32, -pitch_vertical, 0.0, 0.0));
        rots.push(VR::new(4, pitch_vertical + FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_vertical - FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, pitch_vertical + 2.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_vertical - 2.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, pitch_vertical + 3.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_vertical - 3.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, pitch_vertical + 4.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_vertical - 4.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, pitch_vertical + 5.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, -pitch_vertical - 5.0 * FRAC_PI_12, 0.0, 0.0));
        rots.push(VR::new(4, PI, 0.0, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::Diagonals) {
        rots.push(VR::new(4, pitch_flat_to_gentle_diag, 0.0, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_flat_to_gentle_diag, 0.0, FRAC_PI_4));
        rots.push(VR::new(4, pitch_gentle_diag, 0.0, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_gentle_diag, 0.0, FRAC_PI_4));
        rots.push(VR::new(4, pitch_steep_diag, 0.0, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_steep_diag, 0.0, FRAC_PI_4));
    }
    if sprite_groups.contains(&SpriteGroup::BankedTurns) {
        rots.push(VR::new(8, pitch_flat, roll_flat_to_bank, 0.0));
        rots.push(VR::new(8, pitch_flat, -roll_flat_to_bank, 0.0));
        rots.push(VR::new(32, pitch_flat, roll_bank, 0.0));
        rots.push(VR::new(32, pitch_flat, -roll_bank, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::InlineTwists) {
        rots.push(VR::new(4, pitch_flat, 3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_flat, -3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_flat, FRAC_PI_2, 0.0));
        rots.push(VR::new(4, pitch_flat, -FRAC_PI_2, 0.0));
        rots.push(VR::new(4, pitch_flat, 5.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_flat, -5.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_flat, 3.0 * FRAC_PI_4, 0.0));
        rots.push(VR::new(4, pitch_flat, -3.0 * FRAC_PI_4, 0.0));
        rots.push(VR::new(4, pitch_flat, 7.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_flat, -7.0 * FRAC_PI_8, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::SlopeBankTransition) {
        rots.push(VR::new(32, pitch_flat_to_gentle, roll_flat_to_bank, 0.0));
        rots.push(VR::new(32, pitch_flat_to_gentle, -roll_flat_to_bank, 0.0));
        rots.push(VR::new(32, -pitch_flat_to_gentle, roll_flat_to_bank, 0.0));
        rots.push(VR::new(32, -pitch_flat_to_gentle, -roll_flat_to_bank, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::DiagonalBankTransition) {
        rots.push(VR::new(4, pitch_flat_to_gentle_diag, roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, pitch_flat_to_gentle_diag, -roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_flat_to_gentle_diag, roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_flat_to_gentle_diag, -roll_flat_to_bank, FRAC_PI_4));
    }
    if sprite_groups.contains(&SpriteGroup::SlopedBankTransition) {
        rots.push(VR::new(4, pitch_gentle, roll_flat_to_bank, 0.0));
        rots.push(VR::new(4, pitch_gentle, -roll_flat_to_bank, 0.0));
        rots.push(VR::new(4, -pitch_gentle, roll_flat_to_bank, 0.0));
        rots.push(VR::new(4, -pitch_gentle, -roll_flat_to_bank, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::DiagonalSlopedBankTransition) {
        rots.push(VR::new(4, pitch_flat_to_gentle_diag, roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, pitch_flat_to_gentle_diag, -roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_flat_to_gentle_diag, roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_flat_to_gentle_diag, -roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, pitch_gentle_diag, roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, pitch_gentle_diag, -roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_gentle_diag, roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_gentle_diag, -roll_flat_to_bank, FRAC_PI_4));
        rots.push(VR::new(4, pitch_gentle_diag, roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, pitch_gentle_diag, -roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_gentle_diag, roll_bank, FRAC_PI_4));
        rots.push(VR::new(4, -pitch_gentle_diag, -roll_bank, FRAC_PI_4));
    }
    if sprite_groups.contains(&SpriteGroup::BankedSlopedTurns) {
        rots.push(VR::new(32, pitch_gentle, roll_bank, 0.0));
        rots.push(VR::new(32, pitch_gentle, -roll_bank, 0.0));
        rots.push(VR::new(32, -pitch_gentle, roll_bank, 0.0));
        rots.push(VR::new(32, -pitch_gentle, -roll_bank, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::BankedSlopeTransition) {
        rots.push(VR::new(4, pitch_flat_to_gentle, roll_bank, 0.0));
        rots.push(VR::new(4, pitch_flat_to_gentle, -roll_bank, 0.0));
        rots.push(VR::new(4, -pitch_flat_to_gentle, roll_bank, 0.0));
        rots.push(VR::new(4, -pitch_flat_to_gentle, -roll_bank, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::ZeroGRolls) {
        //Gentle bank 67.5
        rots.push(VR::new(4, pitch_gentle, 3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_gentle, -3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle, 3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle, -3.0 * FRAC_PI_8, 0.0));
        //Gentle bank 90
        rots.push(VR::new(4, pitch_gentle, FRAC_PI_2, 0.0));
        rots.push(VR::new(4, pitch_gentle, -FRAC_PI_2, 0.0));
        rots.push(VR::new(4, -pitch_gentle, FRAC_PI_2, 0.0));
        rots.push(VR::new(4, -pitch_gentle, -FRAC_PI_2, 0.0));
        //Gentle 112.5
        rots.push(VR::new(4, pitch_gentle, 5.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_gentle, -5.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle, 5.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle, -5.0 * FRAC_PI_8, 0.0));
        //Gentle bank 135
        rots.push(VR::new(4, pitch_gentle, 3.0 * FRAC_PI_4, 0.0));
        rots.push(VR::new(4, pitch_gentle, -3.0 * FRAC_PI_4, 0.0));
        rots.push(VR::new(4, -pitch_gentle, 3.0 * FRAC_PI_4, 0.0));
        rots.push(VR::new(4, -pitch_gentle, -3.0 * FRAC_PI_4, 0.0));
        //Gentle bank 157.5
        rots.push(VR::new(4, pitch_gentle, 7.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_gentle, -7.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle, 7.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle, -7.0 * FRAC_PI_8, 0.0));
        //Gentle-to-steep bank 22.5
        rots.push(VR::new(4, pitch_gentle_to_steep, FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_gentle_to_steep, -FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, -FRAC_PI_8, 0.0));
        //Gentle-to-steep bank 45
        rots.push(VR::new(4, pitch_gentle_to_steep, 2.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_gentle_to_steep, -2.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, 2.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, -2.0 * FRAC_PI_8, 0.0));
        //Gentle-to-steep bank 67.5
        rots.push(VR::new(4, pitch_gentle_to_steep, 3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, pitch_gentle_to_steep, -3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, 3.0 * FRAC_PI_8, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, -3.0 * FRAC_PI_8, 0.0));
        //Gentle-to-steep bank 90
        rots.push(VR::new(4, pitch_gentle_to_steep, FRAC_PI_2, 0.0));
        rots.push(VR::new(4, pitch_gentle_to_steep, -FRAC_PI_2, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, FRAC_PI_2, 0.0));
        rots.push(VR::new(4, -pitch_gentle_to_steep, -FRAC_PI_2, 0.0));
        //Steep bank 22.5
        let count = if sprite_groups.contains(&SpriteGroup::DiveLoops) {
            8
        } else {
            4
        };
        rots.push(VR::new(count, pitch_steep, FRAC_PI_8, 0.0));
        rots.push(VR::new(count, pitch_steep, -FRAC_PI_8, 0.0));
        rots.push(VR::new(count, -pitch_steep, FRAC_PI_8, 0.0));
        rots.push(VR::new(count, -pitch_steep, -FRAC_PI_8, 0.0));
    }
    if sprite_groups.contains(&SpriteGroup::DiveLoops) {
        //Steep bank 45
        rots.push(VR::new(8, pitch_steep_diag, FRAC_PI_4, FRAC_PI_8));
        rots.push(VR::new(8, pitch_steep_diag, -FRAC_PI_4, FRAC_PI_8));
        rots.push(VR::new(8, -pitch_steep_diag, FRAC_PI_4, FRAC_PI_8));
        rots.push(VR::new(8, -pitch_steep_diag, -FRAC_PI_4, FRAC_PI_8));
        //Steep bank 67.5
        rots.push(VR::new(8, pitch_steep_diag, 3.0 * FRAC_PI_8, FRAC_PI_8));
        rots.push(VR::new(8, pitch_steep_diag, -3.0 * FRAC_PI_8, FRAC_PI_8));
        rots.push(VR::new(8, -pitch_steep_diag, 3.0 * FRAC_PI_8, FRAC_PI_8));
        rots.push(VR::new(8, -pitch_steep_diag, -3.0 * FRAC_PI_8, FRAC_PI_8));
        //Diagonal steep bank 90
        rots.push(VR::new(8, pitch_steep_diag, FRAC_PI_2, FRAC_PI_8));
        rots.push(VR::new(8, pitch_steep_diag, -FRAC_PI_2, FRAC_PI_8));
        rots.push(VR::new(8, -pitch_steep_diag, FRAC_PI_2, FRAC_PI_8));
        rots.push(VR::new(8, -pitch_steep_diag, -FRAC_PI_2, FRAC_PI_8));
    }
    if sprite_groups.contains(&SpriteGroup::Corkscrews) {
        for angles in corkscrew_right_up {
            rots.push(VR::new(4, angles.pitch, angles.roll, angles.yaw));
        }
        for angles in corkscrew_right_down {
            rots.push(VR::new(4, angles.pitch, angles.roll, angles.yaw));
        }
        for angles in corkscrew_left_up {
            rots.push(VR::new(4, angles.pitch, angles.roll, angles.yaw));
        }
        for angles in corkscrew_left_down {
            rots.push(VR::new(4, angles.pitch, angles.roll, angles.yaw));
        }
    }

    rots.iter().flat_map(|x| render_rotations(scene, camera, lights, x)).collect()
}

fn get_model_transforms<'a>(
    models: &'a [renderer::model::Model],
    model_descs: &'a [ModelDesc],
    frame: usize,
) -> anyhow::Result<Vec<ModelTransform<'a>>> {
    use anyhow::Context as _;
    model_descs
        .iter()
        .enumerate()
        .map(|(i, x)| x.get_model_transform(models, frame).with_context(|| format!("Model {i}")))
        .collect()
}

const TILE_SIZE: f32 = 3.3;

struct RenderResult {
    images: Vec<ride_object::Image>,
    file_paths: Vec<std::path::PathBuf>,
}

fn render(
    output_directory: &std::path::Path,
    ride_desc: &RideDesc,
    models: &[renderer::model::Model],
) -> anyhow::Result<RenderResult> {
    use anyhow::Context as _;

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    let camera = glam::Mat4::from_mat3(
        glam::Mat3::from_cols(
            glam::Vec3::new(32.0 / TILE_SIZE, 0.0, -32.0 / TILE_SIZE),
            glam::Vec3::new(-16.0 / TILE_SIZE, -16.0 * 6.0_f32.sqrt() / TILE_SIZE, -16.0 / TILE_SIZE),
            glam::Vec3::new(
                16.0 * 3.0_f32.sqrt() / TILE_SIZE,
                -16.0 * 2.0_f32.sqrt() / TILE_SIZE,
                16.0 * 3.0_f32.sqrt() / TILE_SIZE,
            ),
        )
        .transpose(),
    );

    let angles = VehicleAngles::new();

    let mut object_images = vec![
        ride_object::Image {
            path: "images/preview.png".to_string(),
            x: 0,
            y: 0,
            src_x: None,
            src_y: None,
            src_width: None,
            src_height: None,
            format: Some(ride_object::ImageFormat::Raw),
            palette: ride_object::ImagePaletteType::Keep
        };
        3
    ];

    let mut file_paths = Vec::new();

    for (vehicle_index, vehicle) in ride_desc.vehicles.iter().enumerate() {
        let mut images = Vec::new();

        let vehicle_models = get_model_transforms(models, &vehicle.model, 0)
            .with_context(|| format!("Error in vehicle {vehicle_index}"))?;

        {
            let scene_models = vehicle_models
                .iter()
                .map(|model_desc| renderer::SceneModelDesc {
                    model: model_desc.model,
                    translation: model_desc.translation,
                    rotation: model_desc.rotation,
                    is_mask: None,
                    is_ghost: None,
                })
                .collect::<Vec<_>>();
            let scene = renderer::Scene::new(&render_device, &scene_models)?;
            images.extend(render_vehicle(&scene, &camera, &ride_desc.lights, &ride_desc.sprites, &angles));
        }

        if let Some(riders) = &vehicle.riders {
            let rider_models = get_model_transforms(models, riders, 0)
                .with_context(|| format!("Error in vehicle {vehicle_index} riders"))?;

            for (rider_index, model_desc) in rider_models.iter().enumerate() {
                let mut scene_models = Vec::new();

                scene_models.push(renderer::SceneModelDesc {
                    model: model_desc.model,
                    translation: model_desc.translation,
                    rotation: model_desc.rotation,
                    is_mask: None,
                    is_ghost: None,
                });

                scene_models.extend(vehicle_models.iter().map(|model_desc| renderer::SceneModelDesc {
                    model: model_desc.model,
                    translation: model_desc.translation,
                    rotation: model_desc.rotation,
                    is_mask: Some(true),
                    is_ghost: None,
                }));

                scene_models.extend(rider_models[0..rider_index].iter().map(|model_desc| renderer::SceneModelDesc {
                    model: model_desc.model,
                    translation: model_desc.translation,
                    rotation: model_desc.rotation,
                    is_mask: Some(true),
                    is_ghost: None,
                }));

                let scene = renderer::Scene::new(&render_device, &scene_models)?;
                images.extend(render_vehicle(&scene, &camera, &ride_desc.lights, &ride_desc.sprites, &angles));
            }
        };

        let (image, coords) = renderer::image::create_atlas(&images);
        let file_path = output_directory.join(format!("car_{vehicle_index}")).with_extension("png");
        image.save(&file_path)?;

        for (image, coord) in images.iter().zip(coords.iter()) {
            object_images.push(ride_object::Image {
                path: format!("images/car_{vehicle_index}.png"),
                x: image.offset().x,
                y: image.offset().y,
                src_x: Some(coord.x),
                src_y: Some(coord.y),
                src_width: Some(image.width().try_into().unwrap()),
                src_height: Some(image.height().try_into().unwrap()),
                format: None,
                palette: ride_object::ImagePaletteType::Keep,
            });
        }

        file_paths.push(file_path);
    }

    Ok(RenderResult {
        images: object_images,
        file_paths,
    })
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

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;

    let start_time = std::time::Instant::now();

    let args: Vec<String> = std::env::args().collect();

    let ride_description_path = std::path::PathBuf::from(args.get(1).context("No description file path argument")?);
    let ride_description_path = ride_description_path
        .canonicalize()
        .context(format!("Invalid file path {}", ride_description_path.display()))?;

    let base_directory = ride_description_path
        .parent()
        .context(format!("Could not get parent directory of {}", ride_description_path.display()))?;

    let ride_description = {
        let json = std::fs::read_to_string(&ride_description_path)
            .context(format!("Could not read file {}", ride_description_path.display()))?;

        serde_json::from_str::<RideDesc>(&json)
            .context(format!("Could not parse json in file {}", ride_description_path.display()))?
    };

    let output_directory = base_directory.join(&ride_description.id);
    std::fs::create_dir_all(&output_directory)
        .with_context(|| format!("Could not create directory {}", output_directory.display()))?;

    let models = ride_description
        .meshes
        .iter()
        .map(|x| {
            let x = std::path::PathBuf::from(x);
            let file_path = if x.is_absolute() { x } else { base_directory.join(x) };
            renderer::model::Model::load(&file_path)
        })
        .collect::<anyhow::Result<Vec<renderer::model::Model>>>()?;

    let images_directory = output_directory.join("images");
    std::fs::create_dir_all(&images_directory)
        .with_context(|| format!("Could not create directory {}", images_directory.display()))?;

    let preview_output_file_path = images_directory.join("preview").with_extension("png");
    if let Some(ref preview_file_path) = ride_description.preview {
        let preview_file_path = base_directory.join(preview_file_path);
        if preview_file_path != preview_output_file_path {
            std::fs::copy(&preview_file_path, &preview_output_file_path).with_context(|| {
                format!(
                    "Could not copy preview image {} to {}",
                    preview_file_path.display(),
                    preview_output_file_path.display()
                )
            })?;
        }
    } else {
        let image = renderer::image::IndexedImage::new(1, 1);
        image
            .save(&preview_output_file_path)
            .with_context(|| format!("Could not save preview image {}", preview_output_file_path.display()))?;
    }

    let RenderResult { images, mut file_paths } = render(&images_directory, &ride_description, &models)?;

    let object = ride_object::RideObject::new(&ride_description, images);
    let object_json_file_path = output_directory.join("object").with_extension("json");
    object.save(&object_json_file_path)?;

    file_paths.push(preview_output_file_path);
    file_paths.push(object_json_file_path);

    let parkobj_path = output_directory.with_added_extension("parkobj");
    create_parkobj(&output_directory, &parkobj_path, &file_paths)
        .with_context(|| format!("Could not create parkobj file {}", parkobj_path.display()))?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
