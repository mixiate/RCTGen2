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

#[derive(Debug, serde::Deserialize)]
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

    let scene = {
        let json = std::fs::read_to_string(&scene_description_path)
            .context(format!("Could not read file {}", scene_description_path.display()))?;
        serde_json::from_str::<SceneDesc>(&json).context(format!(
            "Could not parse json in file {}",
            scene_description_path.display()
        ))?
    };

    let models = &scene
        .meshes
        .iter()
        .map(|x| {
            let x = std::path::PathBuf::from(x);
            let file_path = if x.is_absolute() { x } else { base_directory.join(x) };
            model::Model::load(&file_path)
        })
        .collect::<anyhow::Result<Vec<model::Model>>>()?;

    let camera_matrix = glam::Mat4::from_euler(glam::EulerRot::XYZ, -60.0_f32.to_radians(), 45.0_f32.to_radians(), 0.0);
    let camera_matrix = camera_matrix.inverse();

    for (item_index, item) in scene.items.iter().enumerate() {
        let frame_count: usize = item
            .frames
            .try_into()
            .context(format!("Invalid frame count {} in item {item_index}", item.frames))?;
        for frame_index in 0..frame_count {
            let embree_device = embree4_rs::Device::try_new(None)?;
            let embree_scene = embree4_rs::Scene::try_new(embree_device, Default::default())?;

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

            raytrace::add_model(&embree_scene, model, &translation, &rotation)?;

            let embree_scene = embree_scene.commit()?;

            for rotation_index in 0..item.rotations {
                let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation_index as f32);
                let camera_matrix = view_rotation * camera_matrix;

                for y in 0..120 {
                    for x in 0..120 {
                        let origin =
                            glam::Vec3::new((3.3 / 120.0) * x as f32, 5.0, ((3.3 / 120.0) * y as f32) - (3.3 / 2.0));
                        let origin = camera_matrix.transform_point3(origin);
                        let direction = glam::Vec3::new(0.0, -1.0, 0.0);
                        let direction = camera_matrix.transform_vector3(direction);
                        let hit = raytrace::trace_ray(&embree_scene, &origin, &direction);
                        print!("{}", if hit { "x" } else { " " });
                    }
                }
            }
        }
    }

    Ok(())
}
