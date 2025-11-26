#[derive(Debug, PartialEq)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

pub struct Material {
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub specular_exponent: f32,
}

pub struct Mesh {
    pub positions: Vec<glam::Vec3>,
    pub normals: Vec<glam::Vec3>,
    pub indices: Vec<(u32, u32, u32)>,
    pub material: Material,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub struct TransformedMesh<'a> {
    pub mesh: &'a Mesh,
    pub positions: Vec<(f32, f32, f32)>,
    pub normals: Vec<glam::Vec3>,
}

pub struct TransformedModel<'a> {
    pub meshes: Vec<TransformedMesh<'a>>,
}

impl Model {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Model> {
        use anyhow::Context;

        let mut obj = obj::Obj::load(path)?;
        obj.load_mtls()?;

        let mut meshes: Vec<Mesh> = Vec::new();

        if obj.data.objects.is_empty() {
            anyhow::bail!("Obj does not have any objects {}", path.display());
        }
        for object in &obj.data.objects {
            if object.groups.is_empty() {
                anyhow::bail!("Obj does not have any groups {}", path.display());
            }
            for group in &object.groups {
                let material = group.material.as_ref().context(format!(
                    "No material found for object {} in {} ",
                    object.name,
                    path.display(),
                ))?;
                let material = if let obj::ObjMaterial::Mtl(mtl) = material {
                    Material {
                        diffuse: mtl.kd.unwrap_or_default(),
                        specular: mtl.ks.unwrap_or_default(),
                        specular_exponent: mtl.ns.unwrap_or_default(),
                    }
                } else {
                    anyhow::bail!(format!(
                        "No material found for object {} in {} ",
                        object.name,
                        path.display(),
                    ))
                };

                let mut vertices: Vec<Vertex> = Vec::new();
                let mut indices: Vec<(u32, u32, u32)> = Vec::new();

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

                meshes.push(Mesh {
                    positions: vertices.iter().map(|x| x.position.into()).collect(),
                    normals: vertices.iter().map(|x| x.normal.into()).collect(),
                    indices,
                    material,
                });
            }
        }

        Ok(Model { meshes })
    }

    pub fn transform(&'_ self, translation: &glam::Vec3, rotation: &glam::Quat) -> TransformedModel<'_> {
        let transform = glam::Mat4::from_translation(*translation) * glam::Mat4::from_quat(*rotation);
        let meshes = self
            .meshes
            .iter()
            .map(|x| TransformedMesh {
                mesh: x,
                positions: x.positions.iter().map(|x| transform.transform_point3(*x).into()).collect(),
                normals: x.normals.iter().map(|x| transform.transform_vector3(*x)).collect(),
            })
            .collect();
        TransformedModel { meshes }
    }
}
