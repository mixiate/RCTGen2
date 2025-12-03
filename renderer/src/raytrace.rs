struct SceneMesh<'a> {
    mesh: &'a crate::model::Mesh,
    normals: Vec<glam::Vec3>,
}

pub struct Scene<'a> {
    embree_scene: embree::CommittedScene<'a>,
    meshes: Vec<SceneMesh<'a>>,
}

pub struct RayHit<'a> {
    pub u: f32,
    pub v: f32,
    pub mesh: &'a crate::model::Mesh,
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub indices: &'a (u32, u32, u32),
}

impl Scene<'_> {
    pub fn new<'a>(
        embree_device: &'a embree::Device,
        models: Vec<crate::model::TransformedModel<'a>>,
    ) -> anyhow::Result<Scene<'a>> {
        use anyhow::Context as _;
        let embree_scene = embree::Scene::try_new(embree_device).context("Could not create embree scene")?;

        let meshes = {
            let mut meshes: Vec<SceneMesh> = Vec::new();
            for model in models {
                for mesh in model.meshes {
                    embree_scene.add_geometry(&mesh.positions, &mesh.mesh.indices)?;

                    meshes.push(SceneMesh {
                        mesh: mesh.mesh,
                        normals: mesh.normals,
                    });
                }
            }
            meshes
        };

        let embree_scene = embree::commit_scene(embree_scene);

        Ok(Scene { embree_scene, meshes })
    }

    pub fn trace_ray(&'_ self, origin: &glam::Vec3, direction: &glam::Vec3) -> Option<RayHit<'_>> {
        let hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into())?;

        let scene_mesh = &self.meshes[usize::try_from(hit.geometry_id).unwrap()];
        let indices = &scene_mesh.mesh.indices[usize::try_from(hit.primitive_id).unwrap()];

        let normals = [
            scene_mesh.normals[usize::try_from(indices.0).unwrap()] * (1.0 - hit.u - hit.v),
            scene_mesh.normals[usize::try_from(indices.1).unwrap()] * hit.u,
            scene_mesh.normals[usize::try_from(indices.2).unwrap()] * hit.v,
        ];
        let normal = normals.iter().sum::<glam::Vec3>().normalize();

        Some(RayHit {
            u: hit.u,
            v: hit.v,
            mesh: scene_mesh.mesh,
            position: hit.position.into(),
            normal,
            indices,
        })
    }

    pub fn trace_occlusion_ray(&self, origin: &glam::Vec3, direction: &glam::Vec3) -> bool {
        self.embree_scene.occluded_1(&(*origin).into(), &(*direction).into())
    }

    pub fn get_scene_screen_bounds(&self, camera: &glam::Mat4) -> [i32; 4] {
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

        let mut screen_bounds = [i32::MAX, i32::MAX, i32::MIN, i32::MIN];

        for scene_bound in scene_bounds {
            let screen_bound = camera.transform_point3(scene_bound);
            screen_bounds = [
                screen_bounds[0].min(screen_bound.x.floor() as i32 - 1),
                screen_bounds[1].min(screen_bound.y.floor() as i32 - 1),
                screen_bounds[2].max(screen_bound.x.ceil() as i32 + 1),
                screen_bounds[3].max(screen_bound.y.ceil() as i32 + 1),
            ];
        }

        screen_bounds
    }
}
