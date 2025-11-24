pub struct SceneModel<'a> {
    pub model: &'a crate::model::Model,
    pub normals: Vec<glam::Vec3>,
}

pub struct Scene<'a> {
    embree_scene: embree::CommittedScene<'a>,
    models: Vec<SceneModel<'a>>,
}

pub struct RayHit {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
}

impl Scene<'_> {
    pub fn new<'a>(
        embree_device: &'a embree::Device,
        models: Vec<crate::model::TransformedModel<'a>>,
    ) -> anyhow::Result<Scene<'a>> {
        use anyhow::Context;
        let embree_scene = embree::Scene::try_new(embree_device).context("Could not create embree scene")?;

        let models = models
            .into_iter()
            .map(|x| add_model(&embree_scene, x))
            .collect::<anyhow::Result<Vec<SceneModel>>>()?;

        let embree_scene = embree::commit_scene(embree_scene);

        Ok(Scene { embree_scene, models })
    }

    pub fn trace_ray(&self, origin: &glam::Vec3, direction: &glam::Vec3) -> Option<RayHit> {
        let hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into())?;

        let scene_model = self.models.get(usize::try_from(hit.geometry_id).unwrap()).unwrap();
        let indices = scene_model.model.indices.get(usize::try_from(hit.primitive_id).unwrap()).unwrap();

        let normals = [
            scene_model.normals[usize::try_from(indices.0).unwrap()] * (1.0 - hit.u - hit.v),
            scene_model.normals[usize::try_from(indices.1).unwrap()] * hit.u,
            scene_model.normals[usize::try_from(indices.2).unwrap()] * hit.v,
        ];
        let normal: glam::Vec3 = normals.iter().sum::<glam::Vec3>().normalize();

        Some(RayHit {
            position: hit.position.into(),
            normal,
        })
    }

    pub fn trace_occlusion_ray(&self, origin: &glam::Vec3, direction: &glam::Vec3) -> bool {
        self.embree_scene.occluded_1(&(*origin).into(), &(*direction).into())
    }

    pub fn get_scene_screen_bounds(&self, camera: &glam::Mat4) -> anyhow::Result<[i32; 4]> {
        let scene_bounds = self.embree_scene.bounds();
        let scene_bounds = [
            glam::Vec3::new(scene_bounds.lower_x, scene_bounds.lower_y, scene_bounds.lower_z),
            glam::Vec3::new(scene_bounds.upper_x, scene_bounds.lower_y, scene_bounds.lower_z),
            glam::Vec3::new(scene_bounds.lower_x, scene_bounds.upper_y, scene_bounds.lower_z),
            glam::Vec3::new(scene_bounds.upper_x, scene_bounds.upper_y, scene_bounds.lower_z),
            glam::Vec3::new(scene_bounds.lower_x, scene_bounds.lower_y, scene_bounds.upper_z),
            glam::Vec3::new(scene_bounds.upper_x, scene_bounds.lower_y, scene_bounds.upper_z),
            glam::Vec3::new(scene_bounds.lower_x, scene_bounds.upper_y, scene_bounds.upper_z),
            glam::Vec3::new(scene_bounds.upper_x, scene_bounds.upper_y, scene_bounds.upper_z),
        ];

        let mut screen_bounds = {
            let screen_bound = camera.transform_point3(scene_bounds[0]);
            [
                screen_bound.x.floor() as i32,
                screen_bound.y.floor() as i32,
                screen_bound.x.ceil() as i32,
                screen_bound.y.ceil() as i32,
            ]
        };

        for scene_bound in &scene_bounds[1..] {
            let screen_bound = camera.transform_point3(*scene_bound);
            screen_bounds = [
                screen_bounds[0].min(screen_bound.x.floor() as i32),
                screen_bounds[1].min(screen_bound.y.floor() as i32),
                screen_bounds[2].max(screen_bound.x.ceil() as i32),
                screen_bounds[3].max(screen_bound.y.ceil() as i32),
            ];
        }

        Ok(screen_bounds)
    }
}

fn add_model<'a>(scene: &embree::Scene, model: crate::model::TransformedModel<'a>) -> anyhow::Result<SceneModel<'a>> {
    scene.add_geometry(&model.positions, &model.model.indices)?;

    Ok(SceneModel {
        model: model.model,
        normals: model.normals,
    })
}
