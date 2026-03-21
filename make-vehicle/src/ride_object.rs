#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum ObjectType {
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

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColourType {
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

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum Category {
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

#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingMultipliers {
    excitement: Option<i32>,
    intensity: Option<i32>,
    nausea: Option<i32>,
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

impl SpriteGroups {
    pub fn new(ride_desc: &crate::ride_desc::Ride, vehicle: &crate::ride_desc::Vehicle) -> Self {
        use crate::ride_desc::SpriteGroup;

        let restraint_animation = vehicle
            .flags
            .as_ref()
            .and_then(|x| x.contains(&crate::ride_desc::VehicleFlag::RestraintAnimation).then_some(4));

        let slopes60_banked22 = if ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls) {
            if ride_desc.sprites.contains(&SpriteGroup::DiveLoops) {
                Some(8)
            } else {
                Some(4)
            }
        } else {
            None
        };

        SpriteGroups {
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

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Car {
    rotation_frame_mask: i32,
    spacing: i32,
    mass: i32,
    num_seats: i32,
    num_seat_rows: i32,
    friction_sound_id: i32,
    sound_range: i32,
    car_visual: Option<i32>,
    draw_order: i32,
    sprite_groups: SpriteGroups,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    has_additional_colour1: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    has_additional_colour2: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    has_screaming_riders: bool,
    loading_positions: Vec<i32>,
}

impl Car {
    fn new(ride_desc: &crate::ride_desc::Ride, vehicle: &crate::ride_desc::Vehicle) -> Self {
        let rotation_frame_mask = if vehicle.model.is_empty() { 0 } else { 31 };

        let num_seats = vehicle.capacity.unwrap_or(0);
        let num_seat_rows = vehicle.riders.as_ref().map(|x| x.len() as i32).unwrap_or(0);

        let car_visual = vehicle.model.is_empty().then_some(1);

        let sprite_groups = if vehicle.model.is_empty() {
            SpriteGroups {
                slope_flat: Some(1),
                ..SpriteGroups::default()
            }
        } else {
            SpriteGroups::new(ride_desc, vehicle)
        };

        let has_additional_colour1 =
            vehicle.flags.as_ref().is_some_and(|x| x.contains(&crate::ride_desc::VehicleFlag::SecondaryRemap));
        let has_additional_colour2 =
            vehicle.flags.as_ref().is_some_and(|x| x.contains(&crate::ride_desc::VehicleFlag::TertiaryRemap));
        let has_screaming_riders =
            vehicle.flags.as_ref().is_some_and(|x| x.contains(&crate::ride_desc::VehicleFlag::RidersScream));

        let loading_positions = vehicle
            .riders
            .iter()
            .flatten()
            .flat_map(|rider| {
                let position = (32.0 * rider.position[0] / crate::TILE_SIZE).round() as i32;
                if num_seats > 1 {
                    vec![position - 1, position + 1]
                } else {
                    vec![position]
                }
            })
            .collect();

        Car {
            rotation_frame_mask,
            spacing: ((vehicle.spacing * 278912.0) / crate::TILE_SIZE) as i32,
            mass: vehicle.mass,
            num_seats,
            num_seat_rows,
            friction_sound_id: ride_desc.running_sound as i32,
            sound_range: ride_desc.secondary_sound as i32,
            car_visual,
            draw_order: vehicle.draw_order,
            sprite_groups,
            has_additional_colour1,
            has_additional_colour2,
            has_screaming_riders,
            loading_positions,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Properties {
    #[serde(rename = "type")]
    ride_type: RideType,
    category: Category,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    limit_air_time_bonus: bool,
    min_cars_per_train: i32,
    max_cars_per_train: i32,
    num_empty_cars: i32,
    tab_car: i32,
    default_car: i32,
    head_cars: Option<Vec<i32>>,
    tail_cars: Option<Vec<i32>>,
    build_menu_priority: i32,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    no_collision_crashes: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    rider_controls_speed: bool,
    max_height: Option<i32>,
    #[serde(rename = "ratingMultipler")] // Typo in OpenRCT2
    rating_multipliers: Option<RatingMultipliers>,
    car_colours: Vec<Vec<[ColourType; 3]>>,
    cars: Vec<Car>,
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

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RideObject {
    id: String,
    original_id: Option<String>,
    version: String,
    authors: Vec<String>,
    object_type: ObjectType,
    properties: Properties,
    strings: ObjectStrings,
    images: Vec<openrct2::objects::image::Image>,
}

impl RideObject {
    pub fn new(ride_desc: &crate::ride_desc::Ride, images: Vec<openrct2::objects::image::Image>) -> Self {
        let head_cars: Vec<i32> =
            ride_desc.configuration.front.iter().chain(ride_desc.configuration.second.iter()).copied().collect();

        let no_collision_crashes =
            ride_desc.flags.as_ref().is_some_and(|x| x.contains(&crate::ride_desc::Flag::NoCollisionCrashes));
        let rider_controls_speed =
            ride_desc.flags.as_ref().is_some_and(|x| x.contains(&crate::ride_desc::Flag::RiderControlsSpeed));

        let cars = ride_desc.vehicles.iter().map(|vehicle| Car::new(ride_desc, vehicle)).collect();

        let properties = Properties {
            ride_type: ride_desc.ride_type,
            category: Category::Rollercoaster,
            limit_air_time_bonus: ride_desc.limit_air_time_bonus,
            min_cars_per_train: ride_desc.min_cars_per_train,
            max_cars_per_train: ride_desc.max_cars_per_train,
            num_empty_cars: ride_desc.zero_cars,
            tab_car: ride_desc.preview_tab_car,
            default_car: ride_desc.configuration.default,
            head_cars: (!head_cars.is_empty()).then_some(head_cars),
            tail_cars: ride_desc.configuration.rear.map(|x| vec![x]),
            build_menu_priority: ride_desc.build_menu_priority,
            no_collision_crashes,
            rider_controls_speed,
            rating_multipliers: ride_desc.rating_multipliers,
            max_height: ride_desc.max_height,
            car_colours: ride_desc.default_colors.iter().map(|x| vec![*x]).collect(),
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

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
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
