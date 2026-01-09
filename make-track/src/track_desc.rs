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

fn default_bank_angle() -> f32 {
    45.0
}

fn float_1() -> f32 {
    1.0
}

#[derive(Debug, serde::Deserialize)]
pub struct Models<T> {
    pub track: T,
    pub mask: T,
    pub tie: Option<T>,
    pub track_tie: Option<T>,
    pub track_alt: Option<T>,
    pub support_flat: Option<T>,
    pub support_bank_sixth: Option<T>,
    pub support_bank_third: Option<T>,
    pub support_bank_half: Option<T>,
    pub support_bank_two_thirds: Option<T>,
    pub support_bank_five_sixths: Option<T>,
    pub support_bank: Option<T>,
    pub support_base: Option<T>,
    pub brake: Option<T>,
    pub block_brake: Option<T>,
    pub booster: Option<T>,
    pub magnetic_brake: Option<T>,
    pub support_steep_to_vertical: Option<T>,
    pub support_vertical_to_steep: Option<T>,
    pub support_vertical: Option<T>,
    pub support_vertical_twist: Option<T>,
    pub support_barrel_roll: Option<T>,
    pub support_half_loop: Option<T>,
    pub support_quarter_loop: Option<T>,
    pub support_corkscrew: Option<T>,
    pub support_zero_g_roll: Option<T>,
    pub support_large_zero_g_roll: Option<T>,
}

impl Models<std::path::PathBuf> {
    pub fn load(&self, base_directory: &std::path::Path) -> anyhow::Result<Models<renderer::model::Model>> {
        let load_optional_model = |path: &Option<std::path::PathBuf>| {
            path.as_ref().map(|x| renderer::model::Model::load(&base_directory.join(x))).transpose()
        };

        Ok(Models::<renderer::model::Model> {
            track: renderer::model::Model::load(&base_directory.join(&self.track))?,
            mask: renderer::model::Model::load(&base_directory.join(&self.mask))?,
            tie: load_optional_model(&self.tie)?,
            track_tie: load_optional_model(&self.track_tie)?,
            track_alt: load_optional_model(&self.track_alt)?,
            support_flat: load_optional_model(&self.support_flat)?,
            support_bank_sixth: load_optional_model(&self.support_bank_sixth)?,
            support_bank_third: load_optional_model(&self.support_bank_third)?,
            support_bank_half: load_optional_model(&self.support_bank_half)?,
            support_bank_two_thirds: load_optional_model(&self.support_bank_two_thirds)?,
            support_bank_five_sixths: load_optional_model(&self.support_bank_five_sixths)?,
            support_bank: load_optional_model(&self.support_bank)?,
            support_base: load_optional_model(&self.support_base)?,
            brake: load_optional_model(&self.brake)?,
            block_brake: load_optional_model(&self.block_brake)?,
            booster: load_optional_model(&self.booster)?,
            magnetic_brake: load_optional_model(&self.magnetic_brake)?,
            support_steep_to_vertical: load_optional_model(&self.support_steep_to_vertical)?,
            support_vertical_to_steep: load_optional_model(&self.support_vertical_to_steep)?,
            support_vertical: load_optional_model(&self.support_vertical)?,
            support_vertical_twist: load_optional_model(&self.support_vertical_twist)?,
            support_barrel_roll: load_optional_model(&self.support_barrel_roll)?,
            support_half_loop: load_optional_model(&self.support_half_loop)?,
            support_quarter_loop: load_optional_model(&self.support_quarter_loop)?,
            support_corkscrew: load_optional_model(&self.support_corkscrew)?,
            support_zero_g_roll: load_optional_model(&self.support_zero_g_roll)?,
            support_large_zero_g_roll: load_optional_model(&self.support_large_zero_g_roll)?,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Track {
    #[expect(unused)]
    #[serde(default)]
    pub flags: std::collections::HashSet<Flag>,
    #[expect(unused)]
    pub sections: std::collections::HashSet<Section>,
    pub masks: std::path::PathBuf,
    pub name: String,
    #[expect(unused)]
    #[serde(default = "default_lift_offset")]
    pub lift_offset: i32, // unused. unknown default value
    pub length: f32,
    #[expect(unused)]
    #[serde(default = "float_1")]
    pub brake_length: f32,
    #[expect(unused)]
    pub tie_length: Option<f32>,
    pub z_offset: f32,
    #[expect(unused)]
    #[serde(default = "float_1")]
    pub support_spacing: f32,
    #[expect(unused)]
    #[serde(default)]
    pub pivot: f32,
    #[serde(default = "default_bank_angle")]
    pub bank_angle: f32,
    pub models: Models<std::path::PathBuf>,
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
pub struct Light {
    pub r#type: LightType,
    pub shadow: bool,
    pub direction: [f32; 3],
    pub strength: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Desc {
    pub tracks: Vec<Track>,
    #[expect(unused)]
    pub offsets: Option<Offsets>,
    pub lights: Vec<Light>,
    #[serde(default = "bool_true")]
    pub dither: bool,
}

impl Desc {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Desc> {
        use anyhow::Context as _;
        let json = std::fs::read_to_string(path).with_context(|| format!("Could not read file {}", path.display()))?;
        serde_json::from_str::<Desc>(&json).with_context(|| format!("Could not parse json in file {}", path.display()))
    }

    pub fn get_lights(&self) -> Vec<renderer::Light> {
        self.lights
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
                direction: glam::Vec3::from(x.direction).normalize(),
                shadow: x.shadow,
            })
            .collect()
    }
}
