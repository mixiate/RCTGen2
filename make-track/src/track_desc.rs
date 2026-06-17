fn bool_true() -> bool {
    true
}

fn default_bank_angle() -> f32 {
    45.0
}

fn float_1() -> f32 {
    1.0
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackGroup {
    Flat,
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
    Helices,
    MediumQuarterHelices,
    MediumBankedQuarterHelices,
    SmallSlopeTransitions,
    SmallSlopeTransitionsDiagonal,
    LargeSlopeTransitions,
    LargeSlopeTransitionsDiagonal,
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
    TurnBankTransitions,
    VerySmallTurns,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
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
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    pub sections: std::collections::HashSet<TrackGroup>,
    pub masks: std::path::PathBuf,
    pub name: String,
    pub suffix: Option<String>,
    pub length: f32,
    #[serde(default)]
    pub tie_length: f32,
    pub z_offset: i32,
    #[serde(default = "float_1")]
    pub support_spacing: f32,
    #[serde(default)]
    pub pivot: f32,
    #[serde(default = "default_bank_angle")]
    bank_angle: f32,
    #[serde(default)]
    pub lift: bool,
    pub models: Models<std::path::PathBuf>,
}

impl Track {
    pub fn bank_angle(&self) -> f32 {
        self.bank_angle.to_radians()
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Offsets {
    pub flat: [[f32; 2]; 4],
    pub gentle: [[f32; 2]; 4],
    pub steep: [[f32; 2]; 4],
    pub flat_banked: [[f32; 2]; 4],
    pub gentle_banked: [[f32; 2]; 4],
    pub inverted: [[f32; 2]; 4],
    pub diagonal: [[f32; 2]; 4],
    pub diagonal_gentle: [[f32; 2]; 4],
    pub diagonal_steep: [[f32; 2]; 4],
    pub diagonal_banked: [[f32; 2]; 4],
    pub vertical: [[f32; 2]; 4],
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Light {
    pub direction: [f32; 3],
    pub diffuse_strength: f32,
    pub specular_strength: f32,
    pub shadow: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Desc {
    pub tracks: Vec<Track>,
    pub offsets: Option<Offsets>,
    pub lights: Vec<Light>,
    #[serde(default = "bool_true")]
    pub dither: bool,
    pub edge_distance: Option<f32>,
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
                direction: glam::Vec3::from(x.direction).normalize(),
                diffuse_strength: x.diffuse_strength,
                specular_strength: x.specular_strength,
                shadow: x.shadow,
            })
            .collect()
    }
}
