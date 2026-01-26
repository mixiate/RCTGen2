use crate::track_desc;
use crate::track_sections;

#[expect(clippy::too_many_arguments)]
fn scene_add_track_model_transformed<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    model: &'a renderer::model::Model,
    mesh_type: renderer::MeshType,
    track_section: &track_sections::TrackSection,
    scale: f32,
    bank_angle: f32,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    distance: f32,
    mesh_ids: Option<&mut Vec<usize>>,
) -> anyhow::Result<()> {
    let transform = |(position, normal): (&glam::Vec3, &glam::Vec3)| {
        let distance = (position.z * scale) + distance;
        let point = track_section.sample_curve(distance, bank_angle, offset_start, offset_end);

        let position = point.position + (point.normal * position.y) + (point.binormal * position.x);
        let normal = (point.tangent * normal.z) + (point.normal * normal.y) + (point.binormal * normal.x);

        (position, normal)
    };
    scene.add_model_transform(model, transform, mesh_type, mesh_ids)
}

#[expect(clippy::too_many_arguments)]
fn scene_add_track_model<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    track_section: &track_sections::TrackSection,
    tie_model: &'a renderer::model::Model,
    mesh_type: renderer::MeshType,
    track_model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    distance: f32,
    mesh_ids: Option<&mut Vec<usize>>,
) -> anyhow::Result<()> {
    let point = track_section.sample_curve(distance, track_model_desc.bank_angle, offset_start, offset_end);
    let rotation =
        glam::Quat::from_mat3(&glam::Mat3::from_cols(point.binormal, point.normal, point.tangent)).normalize();
    scene.add_model(tie_model, point.position, rotation, mesh_type, mesh_ids)
}

pub struct ModelDesc {
    pub mesh_count: i32,
    pub scale: f32,
    pub length: f32,
    pub bank_angle: f32,
}

impl ModelDesc {
    pub fn new(track: &track_desc::Track, track_section: &track_sections::TrackSection) -> Self {
        let mesh_count = (0.5 + track_section.length / track.length).floor() as i32;
        let scale = track_section.length / (mesh_count as f32 * track.length);

        Self {
            mesh_count,
            scale,
            length: scale * track.length,
            bank_angle: track.bank_angle(),
        }
    }

    /// Attempts to use an even number of alternating track meshes if it doesn't cause too much distortion
    pub fn new_alternating(track: &track_desc::Track, track_section: &track_sections::TrackSection) -> Self {
        let mesh_count = if track_section.prefer_odd_alt_mesh_count {
            (track_section.length / (track.length * 2.0)).floor() as i32 * 2 + 1
        } else {
            (0.5 + track_section.length / (track.length * 2.0)).floor() as i32 * 2
        };
        let scale = track_section.length / (mesh_count as f32 * track.length);

        if scale > 0.9 && scale < 1.11111 {
            Self {
                mesh_count,
                scale,
                length: scale * track.length,
                bank_angle: track.bank_angle(),
            }
        } else {
            Self::new(track, track_section)
        }
    }
}

#[expect(clippy::too_many_arguments)]
fn build_track_segment<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    track_model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    segment_index: i32,
    mesh_type: renderer::MeshType,
    mut mesh_ids: Option<&mut Vec<usize>>,
) -> anyhow::Result<()> {
    let distance = segment_index as f32 * track_model_desc.length;

    let remainder = if track_section.invert_alt_mesh { 0 } else { 1 };
    let track_model = if segment_index % 2 == remainder
        && let Some(track_alt) = &models.track_alt
    {
        track_alt
    } else {
        &models.track
    };

    scene_add_track_model_transformed(
        scene,
        track_model,
        mesh_type,
        track_section,
        track_model_desc.scale,
        track_model_desc.bank_angle,
        offset_start,
        offset_end,
        distance,
        mesh_ids.as_deref_mut(),
    )?;

    if let Some(tie_model) = &models.tie {
        scene_add_track_model(
            scene,
            track_section,
            tie_model,
            mesh_type,
            track_model_desc,
            offset_start,
            offset_end,
            distance + (track_model_desc.length / 2.0),
            mesh_ids,
        )?;
    }

    Ok(())
}

pub struct TrackSectionMeshIds {
    pub extrude_behind_mesh_ids: Vec<usize>,
    pub extrude_ahead_mesh_ids: Vec<usize>,
}

pub fn build<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    track_model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<TrackSectionMeshIds> {
    let mut extrude_behind_mesh_ids = Vec::new();
    build_track_segment(
        scene,
        models,
        track_section,
        track_model_desc,
        offset_start,
        offset_end,
        -1,
        renderer::MeshType::Ghost,
        Some(&mut extrude_behind_mesh_ids),
    )?;

    let mut extrude_ahead_mesh_ids = Vec::new();
    build_track_segment(
        scene,
        models,
        track_section,
        track_model_desc,
        offset_start,
        offset_end,
        track_model_desc.mesh_count,
        renderer::MeshType::Ghost,
        Some(&mut extrude_ahead_mesh_ids),
    )?;

    for i in 0..track_model_desc.mesh_count {
        build_track_segment(
            scene,
            models,
            track_section,
            track_model_desc,
            offset_start,
            offset_end,
            i,
            renderer::MeshType::Normal,
            None,
        )?;
    }

    Ok(TrackSectionMeshIds {
        extrude_behind_mesh_ids,
        extrude_ahead_mesh_ids,
    })
}

pub fn build_mask<'a>(
    render_device: &'a renderer::Device,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    track_model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<renderer::Scene<'a>> {
    let mut scene = renderer::SceneBuilder::new(render_device)?;
    for i in -1..(track_model_desc.mesh_count + 1) {
        scene_add_track_model_transformed(
            &mut scene,
            &models.mask,
            renderer::MeshType::Normal,
            track_section,
            track_model_desc.scale,
            track_model_desc.bank_angle,
            offset_start,
            offset_end,
            i as f32 * track_model_desc.length,
            None,
        )?;
    }
    Ok(scene.build().0)
}
