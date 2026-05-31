#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ObjectType {
    Ride,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RideType {
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

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Transport,
    Gentle,
    Rollercoaster,
    Thrill,
    Water,
    Stall,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingMultipliers {
    pub excitement: Option<i32>,
    pub intensity: Option<i32>,
    pub nausea: Option<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteGroups {
    pub slope_flat: Option<i32>,
    pub slopes12: Option<i32>,
    pub slopes25: Option<i32>,
    pub slopes42: Option<i32>,
    pub slopes60: Option<i32>,
    pub slopes75: Option<i32>,
    pub slopes90: Option<i32>,
    pub slopes_loop: Option<i32>,
    pub slope_inverted: Option<i32>,
    pub slopes8: Option<i32>,
    pub slopes16: Option<i32>,
    pub slopes50: Option<i32>,
    pub flat_banked22: Option<i32>,
    pub flat_banked45: Option<i32>,
    pub flat_banked67: Option<i32>,
    pub flat_banked90: Option<i32>,
    pub inline_twists: Option<i32>,
    pub slopes12_banked22: Option<i32>,
    pub slopes8_banked22: Option<i32>,
    pub slopes25_banked22: Option<i32>,
    pub slopes8_banked45: Option<i32>,
    pub slopes16_banked22: Option<i32>,
    pub slopes16_banked45: Option<i32>,
    pub slopes25_banked45: Option<i32>,
    pub slopes12_banked45: Option<i32>,
    pub slopes25_banked67: Option<i32>,
    pub slopes25_banked90: Option<i32>,
    pub slopes25_inline_twists: Option<i32>,
    pub slopes42_banked22: Option<i32>,
    pub slopes42_banked45: Option<i32>,
    pub slopes42_banked67: Option<i32>,
    pub slopes42_banked90: Option<i32>,
    pub slopes60_banked22: Option<i32>,
    pub slopes50_banked45: Option<i32>,
    pub slopes50_banked67: Option<i32>,
    pub slopes50_banked90: Option<i32>,
    pub corkscrews: Option<i32>,
    pub restraint_animation: Option<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub rotation_frame_mask: i32,
    pub spacing: i32,
    pub mass: i32,
    pub num_seats: i32,
    pub num_seat_rows: i32,
    pub friction_sound_id: i32,
    pub sound_range: i32,
    pub car_visual: Option<i32>,
    pub draw_order: i32,
    pub sprite_groups: SpriteGroups,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub has_additional_colour1: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub has_additional_colour2: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub has_screaming_riders: bool,
    pub loading_positions: Vec<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "type")]
    pub ride_type: RideType,
    pub category: Category,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub limit_air_time_bonus: bool,
    pub min_cars_per_train: i32,
    pub max_cars_per_train: i32,
    pub num_empty_cars: i32,
    pub tab_car: i32,
    pub default_car: i32,
    pub head_cars: Option<Vec<i32>>,
    pub tail_cars: Option<Vec<i32>>,
    pub build_menu_priority: i32,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_collision_crashes: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub rider_controls_speed: bool,
    pub max_height: Option<i32>,
    #[serde(rename = "ratingMultipler")] // Typo in OpenRCT2
    pub rating_multipliers: Option<RatingMultipliers>,
    pub car_colours: Vec<Vec<[crate::colour::Colour; 3]>>,
    pub cars: Vec<Car>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectString {
    #[serde(rename = "en-GB")]
    pub en_gb: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStrings {
    pub name: ObjectString,
    pub description: ObjectString,
    pub capacity: ObjectString,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RideObject {
    pub id: String,
    pub original_id: Option<String>,
    pub version: String,
    pub authors: Vec<String>,
    pub object_type: ObjectType,
    pub properties: Properties,
    pub strings: ObjectStrings,
    pub images: Vec<crate::objects::image::Image>,
}
