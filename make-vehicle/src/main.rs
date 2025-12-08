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

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;

    let start_time = std::time::Instant::now();

    let args: Vec<String> = std::env::args().collect();

    let ride_description_path = std::path::PathBuf::from(args.get(1).context("No description file path argument")?);
    let ride_description_path = ride_description_path
        .canonicalize()
        .context(format!("Invalid file path {}", ride_description_path.display()))?;

    let ride_description = {
        let json = std::fs::read_to_string(&ride_description_path)
            .context(format!("Could not read file {}", ride_description_path.display()))?;

        serde_json::from_str::<RideDesc>(&json).context(format!(
            "Could not parse json in file {}",
            ride_description_path.display()
        ))?
    };

    println!("{ride_description:?}");

    println!("Time taken: {} milliseconds", start_time.elapsed().as_millis());

    Ok(())
}
