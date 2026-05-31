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

fn orientation_to_quat(orientation: &[f32; 3]) -> glam::Quat {
    glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        orientation[0].to_radians(),
        orientation[1].to_radians(),
        orientation[2].to_radians(),
    )
}

#[derive(Debug, serde::Deserialize)]
pub struct Model {
    pub mesh_index: usize,
    pub position: [f32; 3],
    pub orientation: Orientation,
}

#[derive(Debug, serde::Deserialize)]
pub struct Vehicle {
    pub spacing: f32,
    pub mass: i32,
    pub draw_order: i32,
    pub flags: Option<std::collections::HashSet<VehicleFlag>>,
    #[serde(default)]
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
    pub ride_type: openrct2::objects::ride::RideType,
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
    pub rating_multipliers: Option<openrct2::objects::ride::RatingMultipliers>,
    pub max_height: Option<i32>,
    pub configuration: Configuration,
    pub default_colors: Vec<[openrct2::objects::ride::Colour; 3]>,
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

    pub fn get_vehicle_render_descs<'a>(
        &self,
        models: &'a [renderer::model::Model],
    ) -> anyhow::Result<Vec<VehicleRenderType<'a>>> {
        let mut vehicles = Vec::with_capacity(self.vehicles.len());
        for vehicle in &self.vehicles {
            if vehicle.model.is_empty() {
                vehicles.push(VehicleRenderType::Invisible);
            } else {
                let vehicle_models = vehicle
                    .model
                    .iter()
                    .map(|x| ModelRenderDesc::new(x, models))
                    .collect::<anyhow::Result<Vec<_>>>()?;

                let riders = vehicle
                    .riders
                    .iter()
                    .flatten()
                    .map(|x| ModelRenderDesc::new(x, models))
                    .collect::<anyhow::Result<Vec<_>>>()?;

                vehicles.push(VehicleRenderType::Regular(VehicleRenderDesc {
                    sprite_groups: crate::ride_object::create_sprite_groups(self, vehicle),
                    models: vehicle_models,
                    riders,
                }));
            }
        }

        Ok(vehicles)
    }
}

pub struct ModelRenderDesc<'a> {
    pub model: &'a renderer::model::Model,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub restraint_translations: [glam::Vec3; 3],
    pub restraint_rotations: [glam::Quat; 3],
}

impl ModelRenderDesc<'_> {
    fn new<'a>(model: &Model, models: &'a [renderer::model::Model]) -> anyhow::Result<ModelRenderDesc<'a>> {
        use anyhow::Context as _;
        let translation = model.position.into();
        let (rotation, restraint_rotations) = match model.orientation {
            Orientation::Single(orientation) => {
                let rotation = orientation_to_quat(&orientation);
                (rotation, [rotation; 3])
            }
            Orientation::Restraints(orientations) => (
                orientation_to_quat(&orientations[0]),
                [
                    orientation_to_quat(&orientations[1]),
                    orientation_to_quat(&orientations[2]),
                    orientation_to_quat(&orientations[3]),
                ],
            ),
        };
        Ok(ModelRenderDesc {
            model: models.get(model.mesh_index).with_context(|| "Invalid mesh index")?,
            translation,
            rotation,
            restraint_translations: [translation; 3],
            restraint_rotations,
        })
    }
}

pub struct VehicleRenderDesc<'a> {
    pub sprite_groups: openrct2::objects::ride::SpriteGroups,
    pub models: Vec<ModelRenderDesc<'a>>,
    pub riders: Vec<ModelRenderDesc<'a>>,
}

#[expect(clippy::large_enum_variant)]
pub enum VehicleRenderType<'a> {
    Regular(VehicleRenderDesc<'a>),
    Invisible,
}
