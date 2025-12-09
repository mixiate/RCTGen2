#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum RideType {
    AirPoweredVerticalRc,
    AlpineRc,
    BoatHire,
    BobsleighRc,
    CarRide,
    CashMachine,
    Chairlift,
    Circus,
    ClassicMiniRc,
    ClassicStandUpRc,
    ClassicWoodenRc,
    ClassicWoodenTwisterRc,
    CompactInvertedRc,
    CorkscrewRc,
    CrookedHouse,
    DinghySlide,
    Dodgems,
    DrinkStall,
    Enterprise,
    FerrisWheel,
    FirstAid,
    FlyingRc,
    FlyingRcAlt,
    FlyingSaucers,
    FoodStall,
    GhostTrain,
    GigaRc,
    GoKarts,
    HauntedHouse,
    HeartlineTwisterRc,
    HybridRc,
    Hypercoaster,
    HyperTwister,
    InformationKiosk,
    InvertedHairpinRc,
    InvertedImpulseRc,
    InvertedRc,
    JuniorRc,
    LaunchedFreefall,
    LayDownRc,
    LayDownRcAlt,
    Lift,
    LimLaunchedRc,
    LogFlume,
    LoopingRc,
    LsmRc,
    MagicCarpet,
    Maze,
    MerryGoRound,
    MineRide,
    MineTrainRc,
    MiniatureRailway,
    MiniGolf,
    MiniHelicopters,
    MiniRc,
    MiniSuspendedRc,
    Monorail,
    MonorailCycles,
    MonsterTrucks,
    MotionSimulator,
    MultiDimensionRc,
    MultiDimensionRcAlt,
    ObservationTower,
    ReverseFreefallRc,
    ReverserRc,
    RiverRafts,
    RiverRapids,
    RotoDrop,
    Shop,
    SideFrictionRc,
    SingleRailRc,
    SpaceRings,
    SpinningWildMouse,
    SpiralRc,
    SpiralSlide,
    SplashBoats,
    StandUpRc,
    SteelWildMouse,
    Steeplechase,
    SubmarineRide,
    SuspendedMonorail,
    SuspendedSwingingRc,
    SwingingInverterShip,
    SwingingShip,
    Toilets,
    TopSpin,
    Twist,
    TwisterRc,
    VerticalDropRc,
    VirginiaReel,
    WaterCoaster,
    WoodenRc,
    WoodenWildMouse,
}

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

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum RunningSound {
    WoodenOld = 1,
    Wooden = 54,
    Steel = 2,
    SteelSmooth = 57,
    Train = 31,
    Engine = 21,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum ColourType {
    Black,
    Grey,
    White,
    DarkPurple,
    LightPurple,
    BrightPurple,
    DarkBlue,
    LightBlue,
    IcyBlue,
    Teal,
    Aquamarine,
    SaturatedGreen,
    DarkGreen,
    MossGreen,
    BrightGreen,
    OliveGreen,
    DarkOliveGreen,
    BrightYellow,
    Yellow,
    DarkYellow,
    LightOrange,
    DarkOrange,
    LightBrown,
    SaturatedBrown,
    DarkBrown,
    SalmonPink,
    BordeauxRed,
    SaturatedRed,
    BrightRed,
    DarkPink,
    BrightPink,
    LightPink,
    DarkOliveDark,
    DarkOliveLight,
    SaturatedBrownLight,
    BordeauxRedDark,
    BordeauxRedLight,
    GrassGreenDark,
    GrassGreenLight,
    OliveDark,
    OliveLight,
    SaturatedGreenLight,
    TanDark,
    TanLight,
    DullPurpleLight,
    DullGreenDark,
    DullGreenLight,
    SaturatedPurpleDark,
    SaturatedPurpleLight,
    OrangeLight,
    AquaDark,
    MagentaLight,
    DullBrownDark,
    DullBrownLight,
    Invisible,
    Void,
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
struct VehicleModelDesc {
    mesh_index: usize,
    position: [f32; 3],
    orientation: Vec<[f32; 3]>,
}

#[derive(Debug, serde::Deserialize)]
struct VehicleDesc {
    spacing: f32,
    mass: i32,
    draw_order: i32,
    flags: Option<std::collections::HashSet<VehicleFlag>>,
    model: Vec<VehicleModelDesc>,
    capacity: Option<i32>,
    riders: Option<Vec<VehicleModelDesc>>,
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
    ride_type: RideType,
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
    default_colors: Vec<[ColourType; 3]>,
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

const TILE_SIZE: f32 = 3.3;

fn render(
    output_directory: &std::path::Path,
    ride_desc: &RideDesc,
    models: &[renderer::model::Model],
) -> anyhow::Result<Vec<ObjectImage>> {
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
        ObjectImage {
            path: "images/preview.png".to_string(),
            x: 0,
            y: 0,
            src_x: None,
            src_y: None,
            src_width: None,
            src_height: None,
            format: Some(ObjectImageFormat::Raw),
            palette: ObjectImagePaletteType::Keep
        };
        3
    ];

    for (vehicle_index, vehicle) in ride_desc.vehicles.iter().enumerate() {
        let mut images = Vec::new();

        // The way transformed models work and are added to the scene is terrible. Fix this at some point
        let vehicle_models = {
            let mut transformed_models = Vec::new();
            for (model_index, model_desc) in vehicle.model.iter().enumerate() {
                let model = models.get(model_desc.mesh_index).context(format!(
                    "Invalid mesh index {} in vehicle {vehicle_index} model {model_index}",
                    model_desc.mesh_index
                ))?;

                let model_translation: glam::Vec3 = (model_desc.position).into();

                let model_rotation = model_desc
                    .orientation
                    .first()
                    .context(format!("Frame 0 not in vehicle {vehicle_index} model {model_index} orientations"))?;
                let model_rotation = glam::Quat::from_euler(
                    glam::EulerRot::XYZ,
                    model_rotation[0].to_radians(),
                    model_rotation[1].to_radians(),
                    model_rotation[2].to_radians(),
                );

                transformed_models.push(model.transform(&model_translation, &model_rotation, None, None));
            }
            transformed_models
        };

        {
            let scene = renderer::Scene::new(&render_device, vehicle_models.clone())?;
            images.extend(render_vehicle(&scene, &camera, &ride_desc.lights, &ride_desc.sprites, &angles));
        }

        let rider_models = vehicle
            .riders
            .iter()
            .flatten()
            .enumerate()
            .map(|(rider_index, rider)| {
                let model = models.get(rider.mesh_index).context(format!(
                    "Invalid mesh index {} in vehicle {vehicle_index} rider {rider_index}",
                    rider.mesh_index
                ))?;

                let model_translation: glam::Vec3 = (rider.position).into();

                let model_rotation = rider
                    .orientation
                    .first()
                    .context(format!("Frame 0 not in vehicle {vehicle_index} rider {rider_index} orientations"))?;
                let model_rotation = glam::Quat::from_euler(
                    glam::EulerRot::XYZ,
                    model_rotation[0].to_radians(),
                    model_rotation[1].to_radians(),
                    model_rotation[2].to_radians(),
                );

                Ok(model.transform(&model_translation, &model_rotation, None, None))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        for (rider_index, rider_model) in rider_models.iter().enumerate() {
            let mut models = Vec::new();

            models.push(rider_model.clone());

            let mut vehicle_models = vehicle_models.clone();
            for model in &mut vehicle_models {
                for mesh in &mut model.meshes {
                    mesh.is_mask = true;
                }
            }
            models.append(&mut vehicle_models);

            for rider_model in rider_models[0..rider_index].iter() {
                let mut rider_model = rider_model.clone();
                for mesh in &mut rider_model.meshes {
                    mesh.is_mask = true;
                }
                models.push(rider_model);
            }

            let scene = renderer::Scene::new(&render_device, models)?;
            images.extend(render_vehicle(&scene, &camera, &ride_desc.lights, &ride_desc.sprites, &angles));
        }

        let (image, coords) = renderer::image::create_atlas(&images);
        let file_path = output_directory.join(format!("car_{vehicle_index}")).with_extension("png");
        image.save(&file_path)?;

        for (image, coord) in images.iter().zip(coords.iter()) {
            object_images.push(ObjectImage {
                path: format!("images/car_{vehicle_index}.png"),
                x: image.offset.x,
                y: image.offset.y,
                src_x: Some(coord.x),
                src_y: Some(coord.y),
                src_width: Some(image.width as i32),
                src_height: Some(image.height as i32),
                format: None,
                palette: ObjectImagePaletteType::Keep,
            });
        }
    }

    Ok(object_images)
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum ObjectType {
    Ride,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum ObjectCategory {
    #[expect(unused)]
    Transport,
    #[expect(unused)]
    Gentle,
    Rollercoaster,
    #[expect(unused)]
    Thrill,
    #[expect(unused)]
    Water,
    #[expect(unused)]
    Stall,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectSpriteGroups {
    #[serde(skip_serializing_if = "Option::is_none")]
    slope_flat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes12: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes25: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes42: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes60: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes75: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes90: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes_loop: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slope_inverted: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes8: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes16: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes50: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_banked67: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_banked90: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_twists: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes12_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes8_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes25_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes8_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes16_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes16_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes25_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes12_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes25_banked67: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes25_banked90: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes25_inline_twists: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes42_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes42_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes42_banked67: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes42_banked90: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes60_banked22: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes50_banked45: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes50_banked67: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slopes50_banked90: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    corkscrews: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restraint_animation: Option<i32>,
}

impl ObjectSpriteGroups {
    fn new(ride_desc: &RideDesc, vehicle: &VehicleDesc) -> Self {
        let restraint_animation =
            vehicle.flags.as_ref().and_then(|x| x.contains(&VehicleFlag::RestraintAnimation).then_some(4));

        let slopes60_banked22 = if ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls) {
            if ride_desc.sprites.contains(&SpriteGroup::DiveLoops) {
                Some(8)
            } else {
                Some(4)
            }
        } else {
            None
        };

        ObjectSpriteGroups {
            slope_flat: ride_desc.sprites.contains(&SpriteGroup::Flat).then_some(32),
            slopes12: ride_desc.sprites.contains(&SpriteGroup::GentleSlopes).then_some(4),
            slopes25: ride_desc.sprites.contains(&SpriteGroup::GentleSlopes).then_some(32),
            slopes42: ride_desc.sprites.contains(&SpriteGroup::SteepSlopes).then_some(8),
            slopes60: ride_desc.sprites.contains(&SpriteGroup::SteepSlopes).then_some(32),
            slopes75: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(4),
            slopes90: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(32),
            slopes_loop: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(4),
            slope_inverted: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(4),
            slopes8: ride_desc.sprites.contains(&SpriteGroup::Diagonals).then_some(4),
            slopes16: ride_desc.sprites.contains(&SpriteGroup::Diagonals).then_some(4),
            slopes50: ride_desc.sprites.contains(&SpriteGroup::Diagonals).then_some(4),
            flat_banked22: ride_desc.sprites.contains(&SpriteGroup::BankedTurns).then_some(8),
            flat_banked45: ride_desc.sprites.contains(&SpriteGroup::BankedTurns).then_some(32),
            flat_banked67: ride_desc.sprites.contains(&SpriteGroup::InlineTwists).then_some(4),
            flat_banked90: ride_desc.sprites.contains(&SpriteGroup::InlineTwists).then_some(4),
            inline_twists: ride_desc.sprites.contains(&SpriteGroup::InlineTwists).then_some(4),
            slopes12_banked22: ride_desc.sprites.contains(&SpriteGroup::SlopeBankTransition).then_some(32),
            slopes8_banked22: ride_desc.sprites.contains(&SpriteGroup::DiagonalBankTransition).then_some(4),
            slopes25_banked22: ride_desc.sprites.contains(&SpriteGroup::SlopedBankTransition).then_some(4),
            slopes8_banked45: ride_desc.sprites.contains(&SpriteGroup::DiagonalSlopedBankTransition).then_some(4),
            slopes16_banked22: ride_desc.sprites.contains(&SpriteGroup::DiagonalSlopedBankTransition).then_some(4),
            slopes16_banked45: ride_desc.sprites.contains(&SpriteGroup::DiagonalSlopedBankTransition).then_some(4),
            slopes25_banked45: ride_desc.sprites.contains(&SpriteGroup::BankedSlopedTurns).then_some(32),
            slopes12_banked45: ride_desc.sprites.contains(&SpriteGroup::BankedSlopeTransition).then_some(4),
            slopes25_banked67: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes25_banked90: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes25_inline_twists: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes42_banked22: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes42_banked45: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes42_banked67: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes42_banked90: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
            slopes60_banked22,
            slopes50_banked45: ride_desc.sprites.contains(&SpriteGroup::DiveLoops).then_some(8),
            slopes50_banked67: ride_desc.sprites.contains(&SpriteGroup::DiveLoops).then_some(8),
            slopes50_banked90: ride_desc.sprites.contains(&SpriteGroup::DiveLoops).then_some(8),
            corkscrews: ride_desc.sprites.contains(&SpriteGroup::Corkscrews).then_some(4),
            restraint_animation,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectCar {
    rotation_frame_mask: i32,
    spacing: i32,
    mass: i32,
    num_seats: i32,
    num_seat_rows: i32,
    friction_sound_id: i32,
    sound_range: i32,
    draw_order: i32,
    sprite_groups: ObjectSpriteGroups,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_additional_colour1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_additional_colour2: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_screaming_riders: Option<bool>,
    loading_positions: Vec<i32>,
}

impl ObjectCar {
    fn new(ride_desc: &RideDesc, vehicle: &VehicleDesc) -> Self {
        let num_seats = vehicle.capacity.unwrap_or(0);
        let num_seat_rows = vehicle.riders.as_ref().map(|x| x.len() as i32).unwrap_or(0);

        let has_additional_colour1 =
            vehicle.flags.as_ref().and_then(|x| x.contains(&VehicleFlag::SecondaryRemap).then_some(true));
        let has_additional_colour2 =
            vehicle.flags.as_ref().and_then(|x| x.contains(&VehicleFlag::TertiaryRemap).then_some(true));
        let has_screaming_riders =
            vehicle.flags.as_ref().and_then(|x| x.contains(&VehicleFlag::RidersScream).then_some(true));

        let loading_positions = vehicle
            .riders
            .iter()
            .flatten()
            .flat_map(|rider| {
                let position = (32.0 * rider.position[0] / TILE_SIZE).round() as i32;
                if num_seats > 1 {
                    vec![position - 1, position + 1]
                } else {
                    vec![position]
                }
            })
            .collect();

        ObjectCar {
            rotation_frame_mask: 31,
            spacing: ((vehicle.spacing * 278912.0) / TILE_SIZE) as i32,
            mass: vehicle.mass,
            num_seats,
            num_seat_rows,
            friction_sound_id: ride_desc.running_sound as i32,
            sound_range: ride_desc.secondary_sound as i32,
            draw_order: vehicle.draw_order,
            sprite_groups: ObjectSpriteGroups::new(ride_desc, vehicle),
            has_additional_colour1,
            has_additional_colour2,
            has_screaming_riders,
            loading_positions,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectProperties {
    #[serde(rename = "type")]
    ride_type: RideType,
    category: ObjectCategory,
    min_cars_per_train: i32,
    max_cars_per_train: i32,
    num_empty_cars: i32,
    tab_car: i32,
    default_car: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_cars: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tail_cars: Option<Vec<i32>>,
    build_menu_priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_collision_crashes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rider_controls_speed: Option<bool>,
    car_colors: Vec<Vec<[ColourType; 3]>>,
    cars: Vec<ObjectCar>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectString {
    #[serde(rename = "en-GB")]
    en_gb: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectStrings {
    name: ObjectString,
    description: ObjectString,
    capacity: ObjectString,
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum ObjectImageFormat {
    Raw,
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum ObjectImagePaletteType {
    Keep,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectImage {
    path: String,
    x: i32,
    y: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ObjectImageFormat>,
    palette: ObjectImagePaletteType,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RideObject {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_id: Option<String>,
    version: String,
    authors: Vec<String>,
    object_type: ObjectType,
    properties: ObjectProperties,
    strings: ObjectStrings,
    images: Vec<ObjectImage>,
}

impl RideObject {
    fn new(ride_desc: &RideDesc, images: Vec<ObjectImage>) -> Self {
        let no_collision_crashes =
            ride_desc.flags.as_ref().and_then(|x| x.contains(&RideFlag::NoCollisionCrashes).then_some(true));
        let rider_controls_speed =
            ride_desc.flags.as_ref().and_then(|x| x.contains(&RideFlag::RiderControlsSpeed).then_some(true));

        let cars = ride_desc.vehicles.iter().map(|vehicle| ObjectCar::new(ride_desc, vehicle)).collect();

        let properties = ObjectProperties {
            ride_type: ride_desc.ride_type,
            category: ObjectCategory::Rollercoaster,
            min_cars_per_train: ride_desc.min_cars_per_train,
            max_cars_per_train: ride_desc.max_cars_per_train,
            num_empty_cars: ride_desc.zero_cars,
            tab_car: ride_desc.preview_tab_car,
            default_car: ride_desc.configuration.default,
            head_cars: ride_desc.configuration.front.map(|x| vec![x]),
            tail_cars: ride_desc.configuration.rear.map(|x| vec![x]),
            build_menu_priority: ride_desc.build_menu_priority,
            no_collision_crashes,
            rider_controls_speed,
            car_colors: ride_desc.default_colors.iter().map(|x| vec![*x]).collect(),
            cars,
        };

        let strings = ObjectStrings {
            name: ObjectString {
                en_gb: ride_desc.name.clone(),
            },
            description: ObjectString {
                en_gb: ride_desc.description.clone(),
            },
            capacity: ObjectString {
                en_gb: ride_desc.capacity.clone(),
            },
        };

        Self {
            id: ride_desc.id.clone(),
            original_id: ride_desc.original_id.clone(),
            version: ride_desc.version.clone().unwrap_or("1.0".to_string()),
            authors: vec![ride_desc.author.clone()],
            object_type: ObjectType::Ride,
            properties,
            strings,
            images,
        }
    }

    fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        use anyhow::Context as _;
        use serde::Serialize as _;

        let json_formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut json_buffer = Vec::new();
        let mut json_serializer = serde_json::Serializer::with_formatter(&mut json_buffer, json_formatter);

        self.serialize(&mut json_serializer).with_context(|| "Could not serialize object json")?;

        std::fs::write(path, json_buffer).with_context(|| format!("Could not write object file {}", path.display()))?;

        Ok(())
    }
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
        let image = renderer::image::IndexedImage {
            pixels: vec![0],
            width: 1,
            height: 1,
            offset: glam::IVec2::new(0, 0),
        };
        image
            .save(&preview_output_file_path)
            .with_context(|| format!("Could not save preview image {}", preview_output_file_path.display()))?;
    }

    let images = render(&images_directory, &ride_description, &models)?;

    let object = RideObject::new(&ride_description, images);
    let object_json_file_path = output_directory.join("object").with_extension("json");
    object.save(&object_json_file_path)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
