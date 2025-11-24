mod model;
mod raytrace;

#[derive(Debug, serde::Deserialize)]
struct SceneModel {
    mesh_index: i32,
    position: Vec<[f32; 3]>,
    orientation: Vec<[f32; 3]>,
}

#[derive(Debug, serde::Deserialize)]
struct SceneItem {
    name: String,
    rotations: i32,
    frames: i32,
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

struct Light {
    diffuse: bool,
    specular: bool,
    direction: glam::Vec3,
    strength: f32,
}

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

    let embree_device = embree::Device::try_new().context("Could not create embree device")?;

    let models = &scene_desc
        .meshes
        .iter()
        .map(|x| {
            let x = std::path::PathBuf::from(x);
            let file_path = if x.is_absolute() { x } else { base_directory.join(x) };
            model::Model::load(&file_path)
        })
        .collect::<anyhow::Result<Vec<model::Model>>>()?;

    let camera_matrix = glam::Mat4::from_euler(glam::EulerRot::XYZ, -30.0f32.to_radians(), -45.0f32.to_radians(), 0.0);
    let camera_matrix = camera_matrix * glam::Mat4::from_scale([13.713586; 3].into()); // what?

    for (item_index, item) in scene_desc.items.iter().enumerate() {
        let frame_count: usize = item
            .frames
            .try_into()
            .context(format!("Invalid frame count {} in item {item_index}", item.frames))?;
        for frame_index in 0..frame_count {
            let model = usize::try_from(item.model.mesh_index).ok().and_then(|x| models.get(x)).context(format!(
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

            let scene = raytrace::Scene::new(&embree_device, vec![model.transform(&translation, &rotation)])?;

            for rotation_index in 0..item.rotations {
                let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation_index as f32);
                let camera_matrix = camera_matrix * view_rotation;

                let view_rotation_inverse = view_rotation.inverse();
                let lights: Vec<_> = scene_desc
                    .lights
                    .iter()
                    .map(|x| Light {
                        diffuse: x.r#type == LightType::Diffuse,
                        specular: x.r#type == LightType::Specular,
                        direction: view_rotation_inverse.transform_vector3(x.direction.into()).normalize(),
                        strength: x.strength,
                    })
                    .collect();

                let bounds = scene.get_scene_screen_bounds(&camera_matrix)?;
                let offset = glam::Vec3::new(bounds[0] as f32 - 0.5, bounds[1] as f32, 0.0);

                let camera_inverse = camera_matrix.inverse();

                let width = (bounds[2] - bounds[0]) as usize + 1;
                let height = (bounds[3] - bounds[1]) as usize;
                let mut pixels = vec![255; width * height];

                for y in 0..height {
                    for x in 0..width {
                        let origin = glam::Vec3::new(x as f32, (height - 1 - y) as f32, -512.0) + offset;
                        let origin = camera_inverse.transform_point3(origin);
                        let direction = glam::Vec3::new(0.0, 0.0, 1.0);
                        let direction = camera_inverse.transform_vector3(direction).normalize();

                        if let Some(hit) = scene.trace_ray(&origin, &direction) {
                            let mut pixel: u8 = 0;
                            for light in &lights {
                                if light.diffuse {
                                    const DIFFUSE: f32 = 0.5;

                                    let light = hit.normal.dot(light.direction).max(0.0) * light.strength;
                                    pixel = pixel.saturating_add((255.0 * light * DIFFUSE) as u8);
                                }
                                if light.specular {
                                    const SPECULAR_EXPONENT: f32 = 10.0;
                                    const SPECULAR: f32 = 1.0;

                                    let reflected_direction = hit.normal * (2.0 * light.direction.dot(hit.normal));
                                    let reflected_direction = reflected_direction - light.direction;
                                    let angle = reflected_direction.dot(-direction).max(0.0);
                                    let specular_factor = light.strength * angle.powf(SPECULAR_EXPONENT);
                                    pixel = pixel.saturating_add((255.0 * specular_factor * SPECULAR) as u8);
                                }
                            }
                            pixels[y * width + x] = pixel;
                        }
                    }
                }

                let image_path =
                    base_directory.join(format!("{}_{}", item.name, rotation_index + 1)).with_extension("png");
                let image_file = std::fs::File::create(image_path)?;
                let w = std::io::BufWriter::new(image_file);

                let mut encoder = png::Encoder::new(w, width.try_into()?, height.try_into()?);
                encoder.set_color(png::ColorType::Grayscale);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&pixels)?;
            }
        }
    }

    Ok(())
}
