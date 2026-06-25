use crate::track_desc;
use crate::track_sections;

#[expect(clippy::too_many_arguments)]
fn scene_add_track_model_transformed<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    model: &'a renderer::model::Model,
    mesh_type: renderer::MeshType,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    distance: f32,
    mesh_ids: Option<&mut Vec<usize>>,
) -> anyhow::Result<()> {
    let transform = |position: &glam::Vec3, normal: &glam::Vec3, semi_flat_shaded: bool| {
        let vertex_distance = (position.z * model_desc.scale) + distance;
        let point = track_section.sample_curve(vertex_distance, model_desc.bank_angle, offset_start, offset_end);

        let position = point.position + (point.normal * position.y) + (point.binormal * position.x);

        let normal = {
            let point = if semi_flat_shaded {
                let mid_distance = distance + (model_desc.length / 2.0);
                track_section.sample_curve(mid_distance, model_desc.bank_angle, offset_start, offset_end)
            } else {
                point
            };
            (point.tangent * normal.z) + (point.normal * normal.y) + (point.binormal * normal.x)
        };

        (position, normal)
    };
    scene.add_model_transform(model, transform, mesh_type, mesh_ids)
}

#[expect(clippy::too_many_arguments)]
fn scene_add_support_base_model_transformed<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    model: &'a renderer::model::Model,
    mesh_type: renderer::MeshType,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    distance: f32,
    mesh_ids: Option<&mut Vec<usize>>,
) -> anyhow::Result<()> {
    let transform = |position: &glam::Vec3, normal: &glam::Vec3, _semi_flat_shaded: bool| {
        let vertex_distance = (position.z * model_desc.scale) + distance;
        let track_sections::TrackPoint {
            position: track_position,
            tangent: track_tangent,
            normal: _,
            binormal: _,
        } = track_section.sample_curve(vertex_distance, model_desc.bank_angle, offset_start, offset_end);

        let track_binormal = glam::Vec3::new(0.0, 1.0, 0.0).cross(track_tangent).normalize();
        let track_normal = track_tangent.cross(track_binormal).normalize();

        let position = track_position + (glam::Vec3::new(0.0, 1.0, 0.0) * position.y) + (track_binormal * position.x);
        let normal = (track_tangent * normal.z) + (track_normal * normal.y) + (track_binormal * normal.x);

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
    let rotation = glam::Quat::from_rotation_axes(point.binormal, point.normal, point.tangent).normalize();
    scene.add_model(
        tie_model,
        &glam::Affine3::from_rotation_translation(rotation, point.position),
        mesh_type,
        mesh_ids,
    )
}

pub struct ModelLengths {
    track: f32,
    track_tie: f32,
}

impl ModelLengths {
    pub fn calculate(track: &track_desc::Track, models: &track_desc::Models<renderer::model::Model>) -> ModelLengths {
        let track_tie = track.tie_length.unwrap_or_else(|| {
            if let Some(track_tie) = &models.track_tie {
                ModelLengths::calculate_model_length(track_tie)
            } else {
                0.0
            }
        });
        let track = track.length.unwrap_or_else(|| ModelLengths::calculate_model_length(&models.track)) + track_tie;

        ModelLengths { track, track_tie }
    }

    fn calculate_model_length(model: &renderer::model::Model) -> f32 {
        let mut max_z = 0.0;
        for mesh in &model.meshes {
            for vertex in &mesh.positions {
                if vertex.z > max_z {
                    max_z = vertex.z;
                }
            }
        }
        max_z
    }
}

fn calculate_track_point_angle(track_section: &track_sections::TrackSection, distance: f32) -> usize {
    let track_point = track_section.sample_curve(distance, 0.0, &glam::Vec3::ZERO, &glam::Vec3::ZERO);
    let angle = if track_point.tangent == glam::Vec3::Y {
        // quick and dirty hack for vertical pieces
        track_point.binormal.x.atan2(track_point.binormal.z) - std::f32::consts::FRAC_PI_2
    } else {
        track_point.tangent.x.atan2(track_point.tangent.z)
    };
    let entry_angle_offset = (angle / std::f32::consts::FRAC_PI_4).round().rem_euclid(8.0);
    (entry_angle_offset / 2.0).ceil() as usize
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
    pub support_spacing: f32,
    pub support_pivot: f32,
}

impl ModelDesc {
    pub fn new(
        track: &track_desc::Track,
        models: &track_desc::Models<renderer::model::Model>,
        lengths: &ModelLengths,
        track_section: &track_sections::TrackSection,
        rotation: usize,
    ) -> Self {
        let model_desc = if models.track_alt.is_some() {
            ModelDesc::new_alternating(track, lengths, track_section)
        } else {
            ModelDesc::new_non_alternating(track, lengths, track_section)
        };

        if models.track_tie.is_some() {
            model_desc.boundary_tie(track, lengths, track_section, rotation)
        } else {
            model_desc
        }
    }

    fn calculate_extrusion_count(length: f32) -> i32 {
        const MINIMUM_LENGTH: f32 = 0.4;
        ((MINIMUM_LENGTH / length).round() as i32).clamp(1, 4)
    }

    fn new_non_alternating(
        track: &track_desc::Track,
        lengths: &ModelLengths,
        track_section: &track_sections::TrackSection,
    ) -> Self {
        let mesh_count = (0.5 + track_section.length / lengths.track).floor() as i32;
        let scale = track_section.length / (mesh_count as f32 * lengths.track);
        let length = scale * lengths.track;

        Self {
            mesh_count,
            scale,
            length,
            tie_length: scale * lengths.track_tie,
            bank_angle: track.bank_angle(),
            extrusion_count: ModelDesc::calculate_extrusion_count(length),
            track_even: false,
            support_spacing: track.support_spacing,
            support_pivot: track.support_pivot,
        }
    }

    /// Attempts to use an even number of alternating track meshes if it doesn't cause too much distortion
    fn new_alternating(
        track: &track_desc::Track,
        lengths: &ModelLengths,
        track_section: &track_sections::TrackSection,
    ) -> Self {
        let mesh_count = if track_section.prefer_odd_alt_mesh_count {
            (track_section.length / (lengths.track * 2.0)).floor() as i32 * 2 + 1
        } else {
            (0.5 + track_section.length / (lengths.track * 2.0)).floor() as i32 * 2
        };
        let scale = track_section.length / (mesh_count as f32 * lengths.track);
        let length = scale * lengths.track;

        if scale > 0.9 && scale < 1.11111 {
            Self {
                mesh_count,
                scale,
                length,
                tie_length: scale * lengths.track_tie,
                bank_angle: track.bank_angle(),
                extrusion_count: ModelDesc::calculate_extrusion_count(length),
                track_even: false,
                support_spacing: track.support_spacing,
                support_pivot: track.support_pivot,
            }
        } else {
            Self::new_non_alternating(track, lengths, track_section)
        }
    }

    fn boundary_tie(
        &self,
        track: &track_desc::Track,
        lengths: &ModelLengths,
        track_section: &track_sections::TrackSection,
        rotation: usize,
    ) -> Self {
        let entry_angle_offset = calculate_track_point_angle(track_section, 0.0);
        let exit_angle_offset = calculate_track_point_angle(track_section, track_section.length);

        let tie_start = (rotation + entry_angle_offset) % 4 < 2;
        let tie_end = (rotation + exit_angle_offset) % 4 >= 2;

        let (full_length, mesh_count) = {
            let mut full_length = lengths.track * self.mesh_count as f32;
            let mut mesh_count = self.mesh_count * 2;
            if !tie_start {
                full_length -= lengths.track_tie;
                mesh_count -= 1;
            }
            if tie_end {
                full_length += lengths.track_tie;
                mesh_count += 1;
            }
            (full_length, mesh_count)
        };
        let scale = track_section.length / full_length;

        Self {
            mesh_count,
            scale,
            length: scale * lengths.track,
            tie_length: scale * lengths.track_tie,
            bank_angle: self.bank_angle,
            extrusion_count: self.extrusion_count * 2,
            track_even: !tie_start,
            support_spacing: track.support_spacing,
            support_pivot: track.support_pivot,
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
        track_model_desc,
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
                model_desc,
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
            model_desc,
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

fn build_supports<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
    mesh_ids: &mut TrackSectionMeshIds,
) -> anyhow::Result<()> {
    if let Some(support_base_model) = &models.support_base {
        for i in -model_desc.extrusion_count..0 {
            scene_add_support_base_model_transformed(
                scene,
                support_base_model,
                renderer::MeshType::Ghost,
                track_section,
                model_desc,
                offset_start,
                offset_end,
                i as f32 * model_desc.length,
                Some(&mut mesh_ids.extrude_behind_mesh_ids),
            )?;
        }
        for i in model_desc.mesh_count..(model_desc.mesh_count + model_desc.extrusion_count) {
            scene_add_support_base_model_transformed(
                scene,
                support_base_model,
                renderer::MeshType::Ghost,
                track_section,
                model_desc,
                offset_start,
                offset_end,
                i as f32 * model_desc.length,
                Some(&mut mesh_ids.extrude_ahead_mesh_ids),
            )?;
        }
        for i in 0..model_desc.mesh_count {
            scene_add_support_base_model_transformed(
                scene,
                support_base_model,
                renderer::MeshType::Normal,
                track_section,
                model_desc,
                offset_start,
                offset_end,
                i as f32 * model_desc.length,
                None,
            )?;
        }
    }

    let support_count = (0.5 + track_section.length / model_desc.support_spacing).floor() as usize;
    let support_step = track_section.length / support_count as f32;

    for i in 0..(support_count + 1) {
        let distance = i as f32 * support_step;
        let point_unbanked = track_section.sample_curve(distance, 0.0, offset_start, offset_end);
        let point = track_section.sample_curve(distance, model_desc.bank_angle, offset_start, offset_end);

        let support_index = {
            let angle = point_unbanked.normal.angle_between(point.normal);
            let support_angle_interval = model_desc.bank_angle / 4.0;
            (angle / support_angle_interval).round() as usize
        };

        let support_model = match support_index {
            0 => models.support_flat.as_ref(),
            1 => models.support_bank_third.as_ref(),
            2 => models.support_bank_half.as_ref(),
            3 => models.support_bank_two_thirds.as_ref(),
            4 => models.support_bank.as_ref(),
            _ => None,
        };

        if let Some(support_model) = &support_model {
            let position = {
                let y_offset = model_desc.support_pivot
                    / (point.tangent.x * point.tangent.x + point.tangent.z * point.tangent.z).sqrt()
                    - model_desc.support_pivot;
                point.position + glam::Vec3::new(0.0, -y_offset, 0.0)
            };
            let banked_right = point.binormal.y.is_sign_negative();
            let rotation = {
                let point = point.only_yaw();
                let rotation =
                    glam::Quat::from_mat3(&glam::Mat3::from_cols(point.binormal, point.normal, point.tangent))
                        .normalize();
                if banked_right {
                    rotation * glam::Quat::from_rotation_y(180.0_f32.to_radians())
                } else {
                    rotation
                }
            };
            scene.add_model(
                support_model,
                &glam::Affine3::from_rotation_translation(rotation, position),
                renderer::MeshType::Normal,
                None,
            )?;
        }
    }

    Ok(())
}

pub fn build<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<TrackSectionMeshIds> {
    let mut mesh_ids = if models.track_tie.is_some() {
        build_track_boundary_tie(scene, models, track_section, model_desc, offset_start, offset_end)?
    } else {
        build_track(scene, models, track_section, model_desc, offset_start, offset_end)?
    };

    if track_section.has_supports {
        build_supports(
            scene,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
            &mut mesh_ids,
        )?;
    }

    if let Some(model) = models.additional.get(track_section.name) {
        let scale = if model.mirror {
            glam::Vec3::new(-1.0, 1.0, 1.0)
        } else {
            glam::Vec3::ONE
        };
        scene.add_model(
            &model.model,
            &glam::Affine3::from_scale_rotation_translation(scale, glam::Quat::IDENTITY, track_section.position_offset),
            renderer::MeshType::Normal,
            None,
        )?;
    }

    Ok(mesh_ids)
}

pub fn build_mask<'a>(
    render_device: &'a renderer::Device,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    track_model_desc: &ModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<(renderer::Scene<'a>, Vec<renderer::MeshType>)> {
    let mut scene = renderer::SceneBuilder::new(render_device)?;
    for i in -1..(track_model_desc.mesh_count + 1) {
        scene_add_track_model_transformed(
            &mut scene,
            &models.mask,
            renderer::MeshType::Normal,
            track_section,
            track_model_desc,
            offset_start,
            offset_end,
            i as f32 * track_model_desc.length,
            None,
        )?;
    }
    Ok(scene.build())
}
