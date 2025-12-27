#[derive(Debug, PartialEq)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

pub(crate) enum MaterialColour {
    Colour(glam::Vec3),
    Texture(crate::texture::Texture),
}

pub struct Material {
    pub(crate) diffuse: MaterialColour,
    pub(crate) specular: MaterialColour,
    pub(crate) specular_exponent: f32,
    pub(crate) edge_type: Option<crate::renderer::EdgeType>,
    pub(crate) palette_region_type: crate::palette::RegionType,
    pub(crate) use_ao: bool,
}

impl Material {
    pub fn new(mtl: &obj::Material, mtl_file_directory: &std::path::Path) -> anyhow::Result<Self> {
        let mut palette_region_type = crate::palette::RegionType::NoRemaps;
        let mut use_ao = true;
        let mut edge_type = None;
        for segment in mtl.name.split('_') {
            match segment {
                "Remap1" => palette_region_type = crate::palette::RegionType::Remap1,
                "Remap2" => palette_region_type = crate::palette::RegionType::Remap2,
                "Remap3" => palette_region_type = crate::palette::RegionType::Remap3,
                "Greyscale" => palette_region_type = crate::palette::RegionType::Greyscale,
                "Peep" => palette_region_type = crate::palette::RegionType::Peep,
                "NoAO" => use_ao = false,
                "Edge" => edge_type = Some(crate::renderer::EdgeType::Light),
                "DarkEdge" => edge_type = Some(crate::renderer::EdgeType::Dark),
                _ => {}
            }
        }

        let diffuse = if let Some(file_path) = &mtl.map_kd {
            let file_path = mtl_file_directory.join(file_path);
            let texture = crate::texture::Texture::load(&file_path, palette_region_type.is_diffuse_greyscale())?;
            MaterialColour::Texture(texture)
        } else {
            let mut diffuse: glam::Vec3 = mtl.kd.unwrap_or_default().into();
            if palette_region_type.is_diffuse_greyscale() {
                diffuse = crate::palette::linear_rgb_to_luminence_rgb(&diffuse);
            }
            MaterialColour::Colour(diffuse)
        };

        let specular = if let Some(file_path) = &mtl.map_ks {
            let file_path = mtl_file_directory.join(file_path);
            let texture = crate::texture::Texture::load(&file_path, palette_region_type.is_diffuse_greyscale())?;
            MaterialColour::Texture(texture)
        } else {
            let mut specular: glam::Vec3 = mtl.ks.unwrap_or_default().into();
            if palette_region_type.is_diffuse_greyscale() {
                specular = crate::palette::linear_rgb_to_luminence_rgb(&specular);
            }
            MaterialColour::Colour(specular)
        };

        Ok(Material {
            diffuse,
            specular,
            specular_exponent: mtl.ns.unwrap_or_default(),
            edge_type,
            palette_region_type,
            use_ao,
        })
    }
}

pub struct Mesh {
    pub positions: Vec<glam::Vec3>,
    pub normals: Vec<glam::Vec3>,
    pub uvs: Vec<glam::Vec2>,
    pub indices: Vec<[u32; 3]>,
    pub material: Material,
    pub is_mask: bool,
    pub is_ghost: bool,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Model> {
        use anyhow::Context as _;

        let mut obj = obj::Obj::load(path).with_context(|| format!("Could not load obj file {}", path.display()))?;
        obj.load_mtls()
            .with_context(|| format!("Could not load mtl file referenced by {}", path.display()))?;

        let parent_directory =
            path.parent().with_context(|| format!("Could not get parent directory of {}", path.display()))?;

        let mut meshes: Vec<Mesh> = Vec::new();

        if obj.data.objects.is_empty() {
            anyhow::bail!("Obj does not have any objects {}", path.display());
        }
        for object in &obj.data.objects {
            if object.groups.is_empty() {
                anyhow::bail!("Obj does not have any groups {}", path.display());
            }
            for group in &object.groups {
                let material = group
                    .material
                    .as_ref()
                    .and_then(|x| {
                        if let obj::ObjMaterial::Mtl(mtl) = x {
                            Some(mtl)
                        } else {
                            None
                        }
                    })
                    .with_context(|| format!("No material found for object {} in {} ", object.name, path.display()))?;
                let is_mask = material.name.split("_").any(|x| x == "Mask");
                let is_ghost = material.name.split("_").any(|x| x == "Ghost");
                let material = Material::new(material, parent_directory)?;

                let mut vertices: Vec<Vertex> = Vec::new();
                let mut indices: Vec<[u32; 3]> = Vec::new();

                for poly in &group.polys {
                    if poly.0.len() != 3 {
                        anyhow::bail!("Obj meshes are not triangulated {}", path.display());
                    }
                    let mut new_indices = [0_u32; 3];
                    for (indices, new_index) in poly.0.iter().zip(new_indices.iter_mut()) {
                        let position = *obj
                            .data
                            .position
                            .get(indices.0)
                            .with_context(|| format!("Invalid index in obj file {}", path.display()))?;

                        let uv = if let Some(uv_index) = indices.1 {
                            *obj.data
                                .texture
                                .get(uv_index)
                                .with_context(|| format!("Invalid index in obj file {}", path.display()))?
                        } else {
                            [0.0; 2]
                        };

                        let normal = if let Some(normal_index) = indices.2 {
                            *obj.data
                                .normal
                                .get(normal_index)
                                .with_context(|| format!("Invalid index in obj file {}", path.display()))?
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
                        .with_context(|| format!("Invalid index in obj file {}", path.display()))?;
                    }

                    indices.push(new_indices);
                }

                meshes.push(Mesh {
                    positions: vertices.iter().map(|x| x.position.into()).collect(),
                    normals: vertices.iter().map(|x| x.normal.into()).collect(),
                    uvs: vertices.iter().map(|x| x.uv.into()).collect(),
                    indices,
                    material,
                    is_mask,
                    is_ghost,
                });
            }
        }

        Ok(Model { meshes })
    }
}
