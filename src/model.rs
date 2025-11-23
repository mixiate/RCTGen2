#[derive(Debug, PartialEq)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

#[derive(Debug)]
pub struct Model {
    pub positions: Vec<glam::Vec3>,
    pub uvs: Vec<glam::Vec2>,
    pub normals: Vec<glam::Vec3>,
    pub indices: Vec<(u32, u32, u32)>,
}

pub struct TransformedModel<'a> {
    pub model: &'a Model,
    pub positions: Vec<(f32, f32, f32)>,
    pub normals: Vec<glam::Vec3>,
}

impl Model {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Model> {
        use anyhow::Context;

        let mut obj = obj::Obj::load(path)?;

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<(u32, u32, u32)> = Vec::new();

        if obj.data.objects.is_empty() {
            anyhow::bail!("Obj does not have any objects {}", path.display());
        }
        for object in &obj.data.objects {
            if object.groups.is_empty() {
                anyhow::bail!("Obj does not have any groups {}", path.display());
            }
            for group in &object.groups {
                for poly in &group.polys {
                    if poly.0.len() != 3 {
                        anyhow::bail!("Obj meshes are not triangulated {}", path.display());
                    }
                    let mut new_indices = [0u32; 3];
                    for (indices, new_index) in poly.0.iter().zip(new_indices.iter_mut()) {
                        let position = *obj
                            .data
                            .position
                            .get(indices.0)
                            .context(format!("Invalid index in obj file {}", path.display()))?;

                        let uv = if let Some(uv_index) = indices.1 {
                            *obj.data
                                .texture
                                .get(uv_index)
                                .context(format!("Invalid index in obj file {}", path.display()))?
                        } else {
                            [0.0; 2]
                        };

                        let normal = if let Some(normal_index) = indices.2 {
                            *obj.data
                                .normal
                                .get(normal_index)
                                .context(format!("Invalid index in obj file {}", path.display()))?
                        } else {
                            [0.0; 3]
                        };

                        let vertex = Vertex { position, uv, normal };
                        *new_index = if let Some(index) = vertices.iter().position(|x| *x == vertex) {
                            u32::try_from(index)
                        } else {
                            let index = vertices.len();
                            vertices.push(vertex);
                            u32::try_from(index)
                        }
                        .context(format!("Invalid index in obj file {}", path.display()))?;
                    }

                    indices.push(new_indices.into());
                }
            }
        }

        obj.load_mtls()?;

        Ok(Model {
            positions: vertices.iter().map(|x| x.position.into()).collect(),
            uvs: vertices.iter().map(|x| x.uv.into()).collect(),
            normals: vertices.iter().map(|x| x.normal.into()).collect(),
            indices,
        })
    }

    pub fn transform(&'_ self, translation: &glam::Vec3, rotation: &glam::Quat) -> TransformedModel<'_> {
        let transform = glam::Mat4::from_translation(*translation) * glam::Mat4::from_quat(*rotation);
        TransformedModel {
            model: self,
            positions: self.positions.iter().map(|x| transform.transform_point3(*x).into()).collect(),
            normals: self.normals.iter().map(|x| transform.transform_vector3(*x)).collect(),
        }
    }
}
