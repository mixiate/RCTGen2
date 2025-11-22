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

    for item in &scene.items {
        let embree_device = embree4_rs::Device::try_new(None)?;
        let embree_scene = embree4_rs::Scene::try_new(embree_device, Default::default())?;

        if let Some(model) = models.get(usize::try_from(item.model.mesh_index)?) {
            raytrace::add_model(&embree_scene, model)?;
        } else {
            anyhow::bail!("Invalid mesh index in item");
        }

        let embree_scene = embree_scene.commit()?;

        for y in 0..120 {
            for x in 0..120 {
                let origin = [(3.3 / 120.0) * x as f32, 5.0, ((3.3 / 120.0) * y as f32) - (3.3 / 2.0)];
                let hit = raytrace::trace_ray(&embree_scene, &origin, &[0.0, -1.0, 0.0]);
                print!("{}", if hit { "x" } else { " " });
            }
        }
    }

    Ok(())
}
