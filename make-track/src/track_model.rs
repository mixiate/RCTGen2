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

#[derive(Clone, Copy, Debug)]
pub struct ModelDesc {
    pub mesh_count: i32,
    pub scale: f32,
    pub length: f32,
    pub tie_length: f32,
    pub bank_angle: f32,
    pub extrusion_count: i32,
    pub track_even: bool,
}

impl ModelDesc {
    pub fn new(
        track: &track_desc::Track,
        models: &track_desc::Models<renderer::model::Model>,
        track_section: &track_sections::TrackSection,
        rotation: usize,
    ) -> Self {
        let model_desc = if models.track_alt.is_some() {
            ModelDesc::new_alternating(track, track_section)
        } else {
            ModelDesc::new_non_alternating(track, track_section)
        };

        if models.track_tie.is_some() {
            model_desc.boundary_tie(track, track_section, rotation)
        } else {
            model_desc
        }
    }

    fn new_non_alternating(track: &track_desc::Track, track_section: &track_sections::TrackSection) -> Self {
        let mesh_count = (0.5 + track_section.length / track.length).floor() as i32;
        let scale = track_section.length / (mesh_count as f32 * track.length);
        let length = scale * track.length;

        Self {
            mesh_count,
            scale,
            length,
            tie_length: scale * track.tie_length,
            bank_angle: track.bank_angle(),
            extrusion_count: ((0.25 / length).round() as i32).clamp(1, 4),
            track_even: false,
        }
    }

    /// Attempts to use an even number of alternating track meshes if it doesn't cause too much distortion
    fn new_alternating(track: &track_desc::Track, track_section: &track_sections::TrackSection) -> Self {
        let mesh_count = if track_section.prefer_odd_alt_mesh_count {
            (track_section.length / (track.length * 2.0)).floor() as i32 * 2 + 1
        } else {
            (0.5 + track_section.length / (track.length * 2.0)).floor() as i32 * 2
        };
        let scale = track_section.length / (mesh_count as f32 * track.length);
        let length = scale * track.length;

        if scale > 0.9 && scale < 1.11111 {
            Self {
                mesh_count,
                scale,
                length,
                tie_length: scale * track.tie_length,
                bank_angle: track.bank_angle(),
                extrusion_count: ((0.25 / length).round() as i32).clamp(1, 4),
                track_even: false,
            }
        } else {
            Self::new_non_alternating(track, track_section)
        }
    }

    fn boundary_tie(
        &self,
        track: &track_desc::Track,
        track_section: &track_sections::TrackSection,
        rotation: usize,
    ) -> Self {
        let tie_start = (rotation + usize::from(track_section.entry_angle_offset)) % 4 < 2;
        let tie_end = (rotation + usize::from(track_section.exit_angle_offset)) % 4 >= 2;

        let (full_length, mesh_count) = {
            let mut full_length = track.length * self.mesh_count as f32;
            let mut mesh_count = self.mesh_count * 2;
            if !tie_start {
                full_length -= track.tie_length;
                mesh_count -= 1;
            }
            if tie_end {
                full_length += track.tie_length;
                mesh_count += 1;
            }
            (full_length, mesh_count)
        };
        let scale = track_section.length / full_length;

        Self {
            mesh_count,
            scale,
            length: scale * track.length,
            tie_length: scale * track.tie_length,
            bank_angle: self.bank_angle,
            extrusion_count: self.extrusion_count * 2,
            track_even: !tie_start,
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

#[expect(clippy::too_many_arguments)]
fn build_track_segment_boundary_tie<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    segment_index: i32,
    mesh_type: renderer::MeshType,
    mut mesh_ids: Option<&mut Vec<usize>>,
) -> anyhow::Result<()> {
    let segment_index_halved = segment_index.div_euclid(2);
    let distance = segment_index_halved as f32 * model_desc.length;

    let remainder = if model_desc.track_even { 1 } else { 0 };
    if segment_index.rem_euclid(2) == remainder {
        let distance = if model_desc.track_even {
            distance + model_desc.length - model_desc.tie_length
        } else {
            distance
        };

        if let Some(track_tie_model) = &models.track_tie {
            scene_add_track_model_transformed(
                scene,
                track_tie_model,
                mesh_type,
                track_section,
                model_desc.scale,
                model_desc.bank_angle,
                offset_start,
                offset_end,
                distance,
                mesh_ids.as_deref_mut(),
            )?;
        }

        if let Some(tie_model) = &models.tie {
            scene_add_track_model(
                scene,
                track_section,
                tie_model,
                mesh_type,
                model_desc,
                offset_start,
                offset_end,
                distance + (model_desc.tie_length / 2.0),
                mesh_ids,
            )?;
        }
    } else {
        let remainder = if track_section.invert_alt_mesh { 0 } else { 1 };
        let track_model = if segment_index_halved.rem_euclid(2) == remainder
            && let Some(track_alt) = &models.track_alt
        {
            track_alt
        } else {
            &models.track
        };

        let distance = if model_desc.track_even {
            distance
        } else {
            distance + model_desc.tie_length
        };
        scene_add_track_model_transformed(
            scene,
            track_model,
            mesh_type,
            track_section,
            model_desc.scale,
            model_desc.bank_angle,
            offset_start,
            offset_end,
            distance,
            mesh_ids,
        )?;
    }

    Ok(())
}

pub struct TrackSectionMeshIds {
    pub extrude_behind_mesh_ids: Vec<usize>,
    pub extrude_ahead_mesh_ids: Vec<usize>,
}

pub fn build_track<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<TrackSectionMeshIds> {
    let mut extrude_behind_mesh_ids = Vec::new();
    for i in (-model_desc.extrusion_count)..0 {
        build_track_segment(
            scene,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
            i,
            renderer::MeshType::Ghost,
            Some(&mut extrude_behind_mesh_ids),
        )?;
    }

    let mut extrude_ahead_mesh_ids = Vec::new();
    for i in model_desc.mesh_count..(model_desc.mesh_count + model_desc.extrusion_count) {
        build_track_segment(
            scene,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
            i,
            renderer::MeshType::Ghost,
            Some(&mut extrude_ahead_mesh_ids),
        )?;
    }

    for i in 0..model_desc.mesh_count {
        build_track_segment(
            scene,
            models,
            track_section,
            model_desc,
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

pub fn build_track_boundary_tie<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<TrackSectionMeshIds> {
    let mut extrude_behind_mesh_ids = Vec::new();
    for i in -model_desc.extrusion_count..0 {
        build_track_segment_boundary_tie(
            scene,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
            i,
            renderer::MeshType::Ghost,
            Some(&mut extrude_behind_mesh_ids),
        )?;
    }

    let mut extrude_ahead_mesh_ids = Vec::new();
    for i in model_desc.mesh_count..(model_desc.mesh_count + model_desc.extrusion_count) {
        build_track_segment_boundary_tie(
            scene,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
            i,
            renderer::MeshType::Ghost,
            Some(&mut extrude_ahead_mesh_ids),
        )?;
    }

    for i in 0..model_desc.mesh_count {
        build_track_segment_boundary_tie(
            scene,
            models,
            track_section,
            model_desc,
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

pub fn build<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<TrackSectionMeshIds> {
    if models.track_tie.is_some() {
        build_track_boundary_tie(scene, models, track_section, model_desc, offset_start, offset_end)
    } else {
        build_track(scene, models, track_section, model_desc, offset_start, offset_end)
    }
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
