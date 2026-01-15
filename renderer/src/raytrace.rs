struct SceneMesh<'a> {
    mesh: &'a crate::model::Mesh,
    normals: Vec<glam::Vec3>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MeshType {
    Normal,
    Mask,
    Ghost,
}

pub struct SceneBuilder<'a> {
    embree_device: &'a embree::Device,
    embree_scene: embree::Scene<'a>,
    meshes: Vec<SceneMesh<'a>>,
    mesh_types: Vec<MeshType>,
}

impl<'a> SceneBuilder<'a> {
    pub fn new(embree_device: &'a embree::Device) -> anyhow::Result<SceneBuilder<'a>> {
        use anyhow::Context as _;
        let scene = embree::Scene::try_new(embree_device).context("Could not create embree scene")?;
        Ok(SceneBuilder {
            embree_device,
            embree_scene: scene,
            meshes: Vec::new(),
            mesh_types: Vec::new(),
        })
    }

    pub fn add_model(
        &mut self,
        model: &'a crate::model::Model,
        translation: glam::Vec3,
        rotation: glam::Quat,
        mesh_type: MeshType,
        mesh_ids: Option<&mut Vec<usize>>,
    ) -> anyhow::Result<()> {
        let transform = glam::Mat4::from_translation(translation) * glam::Mat4::from_quat(rotation);
        for mesh in &model.meshes {
            let mut geometry = embree::TriangleGeometry::new(self.embree_device, mesh.positions.len(), &mesh.indices)?;
            for (position, geom_position) in mesh.positions.iter().zip(geometry.positions().iter_mut()) {
                *geom_position = transform.transform_point3(*position).into();
            }
            let normals = mesh.normals.iter().map(|x| transform.transform_vector3(*x).normalize()).collect();

            self.embree_scene.add_geometry(geometry, mesh_type == MeshType::Ghost)?;

            if let Some(&mut ref mut mesh_ids) = mesh_ids {
                mesh_ids.push(self.meshes.len());
            }

            self.meshes.push(SceneMesh { mesh, normals });

            self.mesh_types.push(mesh_type);
        }

        Ok(())
    }

    pub fn add_model_transform<F>(
        &mut self,
        model: &'a crate::model::Model,
        transform: F,
        mesh_type: MeshType,
        mesh_ids: Option<&mut Vec<usize>>,
    ) -> anyhow::Result<()>
    where
        F: Fn((&glam::Vec3, &glam::Vec3)) -> (glam::Vec3, glam::Vec3),
    {
        for mesh in &model.meshes {
            let mut geometry = embree::TriangleGeometry::new(self.embree_device, mesh.positions.len(), &mesh.indices)?;
            let mut normals = Vec::with_capacity(mesh.normals.len());
            for (vertex, geometry_position) in
                mesh.positions.iter().zip(mesh.normals.iter()).zip(geometry.positions().iter_mut())
            {
                let vertex = transform(vertex);
                *geometry_position = vertex.0.into();
                normals.push(vertex.1.normalize());
            }
            self.embree_scene.add_geometry(geometry, mesh_type == MeshType::Ghost)?;

            if let Some(&mut ref mut mesh_ids) = mesh_ids {
                mesh_ids.push(self.meshes.len());
            }

            self.meshes.push(SceneMesh { mesh, normals });

            self.mesh_types.push(mesh_type);
        }

        Ok(())
    }

    pub fn build(self) -> (Scene<'a>, Vec<MeshType>) {
        (
            Scene {
                embree_scene: embree::commit_scene(self.embree_scene),
                meshes: self.meshes,
            },
            self.mesh_types,
        )
    }
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
    pub fn trace_ray(
        &'_ self,
        mesh_types: &[MeshType],
        origin: &glam::Vec3,
        direction: &glam::Vec3,
    ) -> Option<RayHit<'_>> {
        let mut hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into(), 0.0)?;
        let ghost_depth = hit.distance;

        let mut geometry_id = usize::try_from(hit.geometry_id).unwrap();

        while mesh_types[geometry_id] == MeshType::Ghost {
            let near = hit.distance + 0.0001;
            let next_hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into(), near);
            if next_hit.is_none() {
                return Some(RayHit::Ghost(ghost_depth));
            }
            hit = next_hit.unwrap();

            geometry_id = usize::try_from(hit.geometry_id).unwrap();
        }

        if mesh_types[geometry_id] == MeshType::Mask {
            return Some(RayHit::Mask);
        }

        let scene_mesh = &self.meshes[geometry_id];

        let indices = &scene_mesh.mesh.indices[usize::try_from(hit.primitive_id).unwrap()];

        let position = self.embree_scene.interpolate(hit.geometry_id, hit.primitive_id, hit.u, hit.v);

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
            position: position.into(),
            normal,
            indices,
        }))
    }

    pub fn trace_occlusion_ray(&self, origin: &glam::Vec3, direction: &glam::Vec3) -> bool {
        self.embree_scene.occluded_1(&(*origin).into(), &(*direction).into())
    }

    pub fn trace_depth_ray(&self, origin: &glam::Vec3, direction: &glam::Vec3) -> f32 {
        let hit = self.embree_scene.intersect_1(&(*origin).into(), &(*direction).into(), 0.0);
        hit.map_or(f32::INFINITY, |x| x.distance)
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
