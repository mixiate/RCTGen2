#[derive(Debug, serde::Deserialize)]
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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum RunningSound {
    WoodenOld,
    Wooden,
    Steel,
    SteelSmooth,
    Train,
    Engine,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum SecondarySound {
    Scream1,
    Scream2,
    Scream3,
    Bell,
}

#[derive(Debug, serde::Deserialize)]
struct Configuration {
    default: i32,
    front: Option<i32>,
    second: Option<i32>,
    third: Option<i32>,
    rear: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
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

fn render_rotations(
    scene: &renderer::Scene,
    camera: &glam::Mat4,
    lights: &[LightDesc],
    count: i32,
    pitch: f32,
    roll: f32,
    yaw: f32,
) -> Vec<renderer::image::IndexedImage> {
    let mut images = Vec::new();
    for i in 0..count {
        let yaw = yaw + ((2.0 * i as f32 * std::f32::consts::PI) / count as f32);
        let view_rotation = glam::Mat4::from_euler(glam::EulerRot::XYZ, roll, yaw, pitch);

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

fn render(
    output_directory: &std::path::Path,
    ride_desc: &RideDesc,
    models: &[renderer::model::Model],
) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    const TILE_SIZE: f32 = 3.3;
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

    for (vehicle_index, vehicle) in ride_desc.vehicles.iter().enumerate() {
        let mut transformed_models = Vec::new();
        for (model_index, model_desc) in vehicle.model.iter().enumerate() {
            let model = models.get(model_desc.mesh_index).context(format!(
                "Invalid mesh index {} in vehicle {vehicle_index} model {model_index}",
                model_desc.mesh_index
            ))?;

            let model_translation: glam::Vec3 = (model_desc.position).into();

            let model_rotation = model_desc.orientation.first().context(format!(
                "Frame 0 not in vehicle {vehicle_index} model {model_index} orientations"
            ))?;
            let model_rotation = glam::Quat::from_euler(
                glam::EulerRot::XYZ,
                model_rotation[0].to_radians(),
                model_rotation[1].to_radians(),
                model_rotation[2].to_radians(),
            );

            transformed_models.push(model.transform(&model_translation, &model_rotation, None, None));
        }

        let scene = renderer::Scene::new(&render_device, transformed_models)?;

        let mut images = Vec::new();

        if ride_desc.sprites.contains(&SpriteGroup::Flat) {
            images.extend(render_rotations(&scene, &camera, &ride_desc.lights, 32, 0.0, 0.0, 0.0));
        }

        let (image, _coords) = renderer::image::create_atlas(&images);
        let file_path = output_directory.join(format!("car_{vehicle_index}")).with_extension("png");
        image.save(&file_path)?;
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

    let base_directory = ride_description_path.parent().context(format!(
        "Could not get parent directory of {}",
        ride_description_path.display()
    ))?;

    let output_directory = base_directory.join("object");
    std::fs::create_dir_all(&output_directory)
        .with_context(|| format!("Could not create directory {}", output_directory.display()))?;

    let ride_description = {
        let json = std::fs::read_to_string(&ride_description_path)
            .context(format!("Could not read file {}", ride_description_path.display()))?;

        serde_json::from_str::<RideDesc>(&json).context(format!(
            "Could not parse json in file {}",
            ride_description_path.display()
        ))?
    };

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

    render(&images_directory, &ride_description, &models)?;

    println!("Time taken: {} milliseconds", start_time.elapsed().as_millis());

    Ok(())
}
