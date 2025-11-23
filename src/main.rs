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
    println!("{scene:?}");

    let models = scene
        .meshes
        .iter()
        .map(|x| {
            let x = std::path::PathBuf::from(x);
            let file_path = if x.is_absolute() { x } else { base_directory.join(x) };
            model::Model::load(&file_path)
        })
        .collect::<anyhow::Result<Vec<model::Model>>>()?;
    println!("{models:?}");

    let embree_device = embree4_rs::Device::try_new(None)?;
    let embree_scene = embree4_rs::Scene::try_new(embree_device, Default::default())?;

    for model in models {
        raytrace::add_model(&embree_scene, &model)?;
    }

    let _embree_scene = embree_scene.commit();

    Ok(())
}
