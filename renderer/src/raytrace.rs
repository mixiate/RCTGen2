pub struct SceneModelDesc<'a> {
    pub model: &'a crate::model::Model,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub is_mask: Option<bool>,
    pub is_ghost: Option<bool>,
}

struct SceneMesh<'a> {
    mesh: &'a crate::model::Mesh,
    normals: Vec<glam::Vec3>,
    is_mask: bool,
    is_ghost: bool,
}

pub struct Scene<'a> {
    embree_scene: embree::CommittedScene<'a>,
    meshes: Vec<SceneMesh<'a>>,
}

pub struct RayHitMesh<'a> {
    pub u: f32,
    pub v: f32,
    pub depth: f32,
    pub ghost_depth: f32,
    pub mesh: &'a crate::model::Mesh,
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub indices: &'a [u32; 3],
}

pub enum RayHit<'a> {
    Mesh(RayHitMesh<'a>),
    Mask,
    Ghost(f32),
}

impl Scene<'_> {
    pub fn new<'a>(embree_device: &'a embree::Device, models: &[SceneModelDesc<'a>]) -> anyhow::Result<Scene<'a>> {
        use anyhow::Context as _;

        let embree_scene = embree::Scene::try_new(embree_device).context("Could not create embree scene")?;

        let meshes = {
            let mut meshes = Vec::new();
            for model_desc in models {
                let transform =
                    glam::Mat4::from_translation(model_desc.translation) * glam::Mat4::from_quat(model_desc.rotation);
                for mesh in &model_desc.model.meshes {
                    let mut geometry =
                        embree::TriangleGeometry::new(embree_device, mesh.positions.len(), &mesh.indices)?;
                    for (position, geom_position) in mesh.positions.iter().zip(geometry.positions().iter_mut()) {
                        *geom_position = transform.transform_point3(*position).into();
                    }
                    embree_scene.add_geometry(geometry)?;

                    meshes.push(SceneMesh {
                        mesh,
                        normals: mesh.normals.iter().map(|x| transform.transform_vector3(*x).normalize()).collect(),
                        is_mask: model_desc.is_mask.unwrap_or(mesh.is_mask),
                        is_ghost: model_desc.is_ghost.unwrap_or(mesh.is_ghost),
                    });
                }
            }
            meshes
        };

        let embree_scene = embree::commit_scene(embree_scene);

        Ok(Scene { embree_scene, meshes })
    }

    pub fn trace_ray(&'_ self, origin: &glam::Vec3, direction: &glam::Vec3) -> Option<RayHit<'_>> {
        let mut hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into(), 0.0)?;
        let ghost_depth = hit.distance;

        let mut scene_mesh = &self.meshes[usize::try_from(hit.geometry_id).unwrap()];

        while scene_mesh.is_ghost {
            let near = hit.distance + 0.0001;
            let next_hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into(), near);
            if next_hit.is_none() {
                return Some(RayHit::Ghost(ghost_depth));
            }
            hit = next_hit.unwrap();

            scene_mesh = &self.meshes[usize::try_from(hit.geometry_id).unwrap()];
        }

        if scene_mesh.is_mask {
            return Some(RayHit::Mask);
        }

        let indices = &scene_mesh.mesh.indices[usize::try_from(hit.primitive_id).unwrap()];

        let normals = [
            scene_mesh.normals[usize::try_from(indices[0]).unwrap()] * (1.0 - hit.u - hit.v),
            scene_mesh.normals[usize::try_from(indices[1]).unwrap()] * hit.u,
            scene_mesh.normals[usize::try_from(indices[2]).unwrap()] * hit.v,
        ];
        let normal = normals.iter().sum::<glam::Vec3>().normalize();

        Some(RayHit::Mesh(RayHitMesh {
            u: hit.u,
            v: hit.v,
            depth: hit.distance,
            ghost_depth,
            mesh: scene_mesh.mesh,
            position: hit.position.into(),
            normal,
            indices,
        }))
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
