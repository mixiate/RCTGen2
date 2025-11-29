#[derive(Debug, serde::Deserialize)]
struct SceneModel {
    mesh_index: usize,
    position: Vec<[f32; 3]>,
    orientation: Vec<[f32; 3]>,
}

#[derive(Debug, serde::Deserialize)]
struct SceneItem {
    name: String,
    rotations: usize,
    frames: usize,
    model: SceneModel,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum LightType {
    Diffuse,
    Specular,
}

#[derive(Debug, serde::Deserialize)]
struct SceneLight {
    r#type: LightType,
    shadow: bool,
    direction: [f32; 3],
    strength: f32,
}

#[derive(Debug, serde::Deserialize)]
struct SceneDesc {
    meshes: Vec<String>,
    items: Vec<SceneItem>,
    lights: Vec<SceneLight>,
}

const SQRT_6: f32 = 2.449_489_8;
const TILE_SIZE: f32 = 3.3;
const CLEARANCE_HEIGHT: f32 = 0.5 * TILE_SIZE / SQRT_6;

fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    let args: Vec<String> = std::env::args().collect();
    let scene_description_path = std::path::PathBuf::from(args.get(1).context("No description file path argument.")?);
    let scene_description_path = scene_description_path
        .canonicalize()
        .context(format!("Invalid file path {}", scene_description_path.display()))?;
    let base_directory = scene_description_path.parent().context(format!(
        "Could not get parent directory of {}",
        scene_description_path.display()
    ))?;

    let scene_desc = {
        let json = std::fs::read_to_string(&scene_description_path)
            .context(format!("Could not read file {}", scene_description_path.display()))?;
        serde_json::from_str::<SceneDesc>(&json).context(format!(
            "Could not parse json in file {}",
            scene_description_path.display()
        ))?
    };

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    let models = &scene_desc
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

    for (item_index, item) in scene_desc.items.iter().enumerate() {
        for frame_index in 0..item.frames {
            let model = models.get(item.model.mesh_index).context(format!(
                "Invalid mesh index {} in item {item_index}",
                item.model.mesh_index
            ))?;

            let translation = item
                .model
                .position
                .get(frame_index)
                .context(format!("Frame {frame_index} not in item {item_index} positions"))?;
            let translation: glam::Vec3 = (*translation).into();

            let rotation = item
                .model
                .orientation
                .get(frame_index)
                .context(format!("Frame {frame_index} not in item {item_index} orientations"))?;
            let rotation = glam::Quat::from_euler(
                glam::EulerRot::XYZ,
                rotation[0].to_radians(),
                rotation[1].to_radians(),
                rotation[2].to_radians(),
            );

            let scene = renderer::Scene::new(&render_device, vec![model.transform(&translation, &rotation)])?;

            for rotation_index in 0..item.rotations {
                let view_translation = {
                    let offsets = [0.0, -1.0, 0.0, -1.5, 0.0, -1.0, 0.0, -1.5];
                    glam::Mat4::from_translation(glam::Vec3::new(
                        CLEARANCE_HEIGHT * offsets[2 * rotation_index] / 8.0,
                        CLEARANCE_HEIGHT * offsets[2 * rotation_index + 1] / 8.0,
                        0.0,
                    ))
                };
                let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation_index as f32);
                let camera = camera * view_rotation * view_translation;

                let view_rotation_inverse = view_rotation.inverse();
                let lights: Vec<_> = scene_desc
                    .lights
                    .iter()
                    .map(|x| renderer::Light {
                        diffuse: x.r#type == LightType::Diffuse,
                        specular: x.r#type == LightType::Specular,
                        direction: view_rotation_inverse.transform_vector3(x.direction.into()).normalize(),
                        strength: x.strength,
                        shadow: x.shadow,
                    })
                    .collect();

                let framebuffer = renderer::render_scene(&scene, &camera, &lights, 4, 4);
                let image = framebuffer.to_cropped_indexed_image();

                let image_path =
                    base_directory.join(format!("{}_{}", item.name, rotation_index + 1)).with_extension("png");
                let image_file = std::fs::File::create(image_path)?;
                let w = std::io::BufWriter::new(image_file);

                let mut encoder = png::Encoder::new(w, image.width.try_into()?, image.height.try_into()?);
                encoder.set_color(png::ColorType::Indexed);
                encoder.set_depth(png::BitDepth::Eight);
                encoder.set_palette(&renderer::palette::PALETTE_FLAT);
                encoder.set_trns(&[0]);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&image.pixels)?;
            }
        }
    }

    Ok(())
}
