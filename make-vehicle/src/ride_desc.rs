#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Flag {
    NoCollisionCrashes,
    RiderControlsSpeed,
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpriteGroup {
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
pub enum RunningSound {
    WoodenOld = 1,
    Wooden = 54,
    Steel = 2,
    SteelSmooth = 57,
    Train = 31,
    Engine = 21,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecondarySound {
    Scream1 = 0,
    Scream2 = 1,
    Scream3 = 2,
    Whistle = 3,
    Bell = 4,
}

#[derive(Debug, serde::Deserialize)]
pub struct Configuration {
    pub default: i32,
    pub front: Option<i32>,
    pub second: Option<i32>,
    pub rear: Option<i32>,
}

#[derive(Debug, Eq, Hash, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleFlag {
    SecondaryRemap,
    TertiaryRemap,
    RidersScream,
    RestraintAnimation,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Orientation {
    Single([f32; 3]),
    Restraints([[f32; 3]; 4]),
}

#[derive(Debug, serde::Deserialize)]
pub struct Model {
    pub mesh_index: usize,
    pub position: [f32; 3],
    pub orientation: Orientation,
}

pub struct ModelTransform<'a> {
    pub model: &'a renderer::model::Model,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
}

impl Model {
    pub fn get_model_transform<'a>(
        &self,
        models: &'a [renderer::model::Model],
        frame: usize,
    ) -> anyhow::Result<ModelTransform<'a>> {
        use anyhow::Context as _;

        let model = models.get(self.mesh_index).with_context(|| format!("Invalid mesh index {}", self.mesh_index))?;

        let translation = (self.position).into();

        let rotation = match self.orientation {
            Orientation::Single(orientation) => orientation,
            Orientation::Restraints(orientations) => orientations[frame],
        };
        let rotation = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation[0].to_radians(),
            rotation[2].to_radians(), // Y and Z are swapped. Fix?
            rotation[1].to_radians(),
        );

        Ok(ModelTransform {
            model,
            translation,
            rotation,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Vehicle {
    pub spacing: f32,
    pub mass: i32,
    pub draw_order: i32,
    pub flags: Option<std::collections::HashSet<VehicleFlag>>,
    pub model: Vec<Model>,
    pub capacity: Option<i32>,
    pub riders: Option<Vec<Model>>,
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
pub struct Ride {
    pub id: String,
    pub original_id: Option<String>,
    pub name: String,
    pub description: String,
    pub capacity: String,
    pub author: String,
    pub version: Option<String>,
    pub preview: Option<std::path::PathBuf>,
    pub ride_type: crate::ride_object::RideType,
    pub flags: Option<std::collections::HashSet<Flag>>,
    pub sprites: std::collections::HashSet<SpriteGroup>,
    pub zero_cars: i32,
    pub preview_tab_car: i32,
    pub build_menu_priority: i32,
    pub running_sound: RunningSound,
    pub secondary_sound: SecondarySound,
    pub min_cars_per_train: i32,
    pub max_cars_per_train: i32,
    #[serde(default)]
    pub limit_air_time_bonus: bool,
    pub rating_multipliers: Option<crate::ride_object::RatingMultipliers>,
    pub max_height: Option<i32>,
    pub configuration: Configuration,
    pub default_colors: Vec<[crate::ride_object::ColourType; 3]>,
    pub meshes: Vec<std::path::PathBuf>,
    pub vehicles: Vec<Vehicle>,
    pub lights: Vec<Light>,
}

impl Ride {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Ride> {
        use anyhow::Context as _;

        let json = std::fs::read_to_string(path).with_context(|| format!("Could not read file {}", path.display()))?;

        serde_json::from_str::<Ride>(&json).with_context(|| format!("Could not parse json in file {}", path.display()))
    }

    pub fn load_models(&self, base_directory: &std::path::Path) -> anyhow::Result<Vec<renderer::model::Model>> {
        self.meshes
            .iter()
            .map(|x| {
                let x = std::path::PathBuf::from(x);
                let file_path = if x.is_absolute() { x } else { base_directory.join(x) };
                renderer::model::Model::load(&file_path)
            })
            .collect()
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
