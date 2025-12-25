fn bool_true() -> bool {
    true
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Flag {
    HasLift,
    HasSupports,
    SeparateTie,
    TieAtBoundary,
    SpecialEndOffsets,
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Section {
    Flat,
    Brakes,
    BlockBrakes,
    DiagonalBrakes,
    SlopedBrakes,
    MagneticBrakes,
    Turns,
    GentleSlopes,
    SteepSlopes,
    VerticalSlopes,
    Diagonals,
    SlopedTurns,
    GentleSlopedTurns,
    BankedTurns,
    BankedSlopedTurns,
    LargeSlopedTurns,
    LargeBankedSlopedTurns,
    SBends,
    BankedSBends,
    Helices,
    SmallSlopeTransitions,
    LargeSlopeTransitions,
    BarrelRolls,
    InlineTwists,
    QuarterLoops,
    Corkscrews,
    LargeCorkscrews,
    HalfLoops,
    VerticalLoops,
    MediumHalfLoops,
    LargeHalfLoops,
    ZeroGRolls,
    DiveLoops,
    Boosters,
    LaunchedLifts,
    TurnBankTransitions,
    VerticalBoosters,
}

fn default_lift_offset() -> i32 {
    13
}

fn float_1() -> f32 {
    1.0
}

#[derive(Debug, serde::Deserialize)]
pub struct Models {
    #[expect(unused)]
    track: std::path::PathBuf,
    #[expect(unused)]
    mask: std::path::PathBuf,
    #[expect(unused)]
    tie: Option<std::path::PathBuf>,
    #[expect(unused)]
    track_tie: Option<std::path::PathBuf>,
    #[expect(unused)]
    track_alt: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_flat: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_bank_sixth: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_bank_third: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_bank_half: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_bank_two_thirds: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_bank_five_sixths: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_bank: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_base: Option<std::path::PathBuf>,
    #[expect(unused)]
    brake: Option<std::path::PathBuf>,
    #[expect(unused)]
    block_brake: Option<std::path::PathBuf>,
    #[expect(unused)]
    booster: Option<std::path::PathBuf>,
    #[expect(unused)]
    magnetic_brake: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_steep_to_vertical: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_vertical_to_steep: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_vertical: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_vertical_twist: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_barrel_roll: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_half_loop: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_quarter_loop: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_corkscrew: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_zero_g_roll: Option<std::path::PathBuf>,
    #[expect(unused)]
    support_large_zero_g_roll: Option<std::path::PathBuf>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Track {
    #[expect(unused)]
    #[serde(default)]
    pub flags: std::collections::HashSet<Flag>,
    #[expect(unused)]
    pub sections: std::collections::HashSet<Section>,
    #[expect(unused)]
    pub masks: std::path::PathBuf,
    #[expect(unused)]
    pub name: Option<String>,
    #[expect(unused)]
    #[serde(default = "default_lift_offset")]
    pub lift_offset: i32, // unused. unknown default value
    #[expect(unused)]
    pub length: f32,
    #[expect(unused)]
    #[serde(default = "float_1")]
    pub brake_length: f32,
    #[expect(unused)]
    pub tie_length: Option<f32>,
    #[expect(unused)]
    pub z_offset: f32,
    #[expect(unused)]
    #[serde(default = "float_1")]
    pub support_spacing: f32,
    #[expect(unused)]
    #[serde(default)]
    pub pivot: f32,
    #[expect(unused)]
    pub models: Models,
}

#[derive(Debug, serde::Deserialize)]
#[expect(unused)]
pub struct Offsets {
    flat: [f32; 8],
    gentle: [f32; 8],
    steep: [f32; 8],
    flat_banked: [f32; 8],
    gentle_banked: [f32; 8],
    inverted: [f32; 8],
    diagonal: [f32; 8],
    diagonal_gentle: [f32; 8],
    diagonal_steep: [f32; 8],
    diagonal_banked: [f32; 8],
    vertical: [f32; 8],
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightType {
    Diffuse,
    Specular,
}

#[derive(Debug, serde::Deserialize)]
#[expect(unused)]
pub struct Light {
    pub r#type: LightType,
    pub shadow: bool,
    pub direction: [f32; 3],
    pub strength: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Desc {
    #[expect(unused)]
    pub tracks: Vec<Track>,
    #[expect(unused)]
    pub offsets: Option<Offsets>,
    #[expect(unused)]
    pub lights: Vec<Light>,
    #[expect(unused)]
    #[serde(default = "bool_true")]
    pub dither: bool,
}

impl Desc {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Desc> {
        use anyhow::Context as _;
        let json = std::fs::read_to_string(path).with_context(|| format!("Could not read file {}", path.display()))?;
        serde_json::from_str::<Desc>(&json).with_context(|| format!("Could not parse json in file {}", path.display()))
    }
}
