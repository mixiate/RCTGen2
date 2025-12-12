#[derive(Debug, serde::Deserialize)]
struct ModelDesc {
    mesh_index: usize,
    position: Vec<[f32; 3]>,
    orientation: Vec<[f32; 3]>,
}

#[derive(Debug, serde::Deserialize)]
struct ItemDesc {
    name: String,
    rotations: usize,
    frames: usize,
    model: ModelDesc,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
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
struct TestDesc {
    meshes: Vec<String>,
    items: Vec<ItemDesc>,
    lights: Vec<LightDesc>,
    sprite_directory: std::path::PathBuf,
}

const SQRT_6: f32 = 2.449_489_8;
const TILE_SIZE: f32 = 3.3;
const CLEARANCE_HEIGHT: f32 = 0.5 * TILE_SIZE / SQRT_6;

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;

    let start_time = std::time::Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let scene_description_path = std::path::PathBuf::from(args.get(1).context("No description file path argument.")?);
    let scene_description_path = scene_description_path
        .canonicalize()
        .with_context(|| format!("Invalid file path {}", scene_description_path.display()))?;
    let base_directory = scene_description_path
        .parent()
        .with_context(|| format!("Could not get parent directory of {}", scene_description_path.display()))?;

    let test_desc = {
        let json = std::fs::read_to_string(&scene_description_path)
            .with_context(|| format!("Could not read file {}", scene_description_path.display()))?;
        serde_json::from_str::<TestDesc>(&json)
            .with_context(|| format!("Could not parse json in file {}", scene_description_path.display()))?
    };

    let sprite_directory = if test_desc.sprite_directory.is_absolute() {
        test_desc.sprite_directory
    } else {
        base_directory.join(test_desc.sprite_directory)
    };
    std::fs::create_dir_all(&sprite_directory)
        .with_context(|| format!("Could not create directory {}", sprite_directory.display()))?;

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    let models = &test_desc
        .meshes
        .iter()
        .map(|x| {
            let x = std::path::PathBuf::from(x);
            let file_path = if x.is_absolute() { x } else { base_directory.join(x) };
            renderer::model::Model::load(&file_path)
        })
        .collect::<anyhow::Result<Vec<renderer::model::Model>>>()?;

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

    for (item_index, item) in test_desc.items.iter().enumerate() {
        for frame_index in 0..item.frames {
            let model = models
                .get(item.model.mesh_index)
                .with_context(|| format!("Invalid mesh index {} in item {item_index}", item.model.mesh_index))?;

            let model_translation = item
                .model
                .position
                .get(frame_index)
                .with_context(|| format!("Frame {frame_index} not in item {item_index} positions"))?;
            let model_translation: glam::Vec3 = (*model_translation).into();

            let model_rotation = item
                .model
                .orientation
                .get(frame_index)
                .with_context(|| format!("Frame {frame_index} not in item {item_index} orientations"))?;
            let model_rotation = glam::Quat::from_euler(
                glam::EulerRot::XYZ,
                model_rotation[0].to_radians(),
                model_rotation[1].to_radians(),
                model_rotation[2].to_radians(),
            );

            for rotation_index in 0..item.rotations {
                let scene = {
                    let model_translation = {
                        let offsets = [0.0, -1.0, 0.0, -1.5, 0.0, -1.0, 0.0, -1.5];
                        let offset = glam::Vec3::new(
                            CLEARANCE_HEIGHT * offsets[2 * rotation_index] / 8.0,
                            CLEARANCE_HEIGHT * offsets[2 * rotation_index + 1] / 8.0,
                            0.0,
                        );
                        model_translation + offset
                    };
                    let scene_models = &[renderer::SceneModelDesc {
                        model,
                        translation: model_translation,
                        rotation: model_rotation,
                        is_mask: None,
                        is_ghost: None,
                    }];
                    renderer::Scene::new(&render_device, scene_models)?
                };

                let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation_index as f32);
                let camera = camera * view_rotation;

                let view_rotation_inverse = view_rotation.inverse();
                let lights: Vec<_> = test_desc
                    .lights
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

                let framebuffer = renderer::render_scene(&scene, &camera, &lights, 4, 4);

                let image = framebuffer.to_image();
                let image_path =
                    sprite_directory.join(format!("{}_{}_rgb", item.name, rotation_index + 1)).with_extension("png");
                image.save(&image_path)?;

                let image = framebuffer.into_cropped_indexed_image(true);
                let image_path =
                    sprite_directory.join(format!("{}_{}", item.name, rotation_index + 1)).with_extension("png");
                image.save(&image_path)?;

                println!(
                    "{} {}",
                    image_path.strip_prefix(base_directory).unwrap_or(&image_path).display(),
                    image.offset()
                );
            }
        }
    }

    println!("Time taken: {} milliseconds", start_time.elapsed().as_millis());

    Ok(())
}
