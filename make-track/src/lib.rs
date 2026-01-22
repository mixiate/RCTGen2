mod curves;
mod mask;
mod offset;
mod split;
mod track_curves;
mod track_desc;
mod track_sections;

const CLEARANCE_HEIGHT: f32 = 0.204_124_15; // 1.0 / (2.0 * 6.0.sqrt())

#[expect(clippy::too_many_arguments)]
fn add_model_to_scene<'a>(
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
    track_model_desc: &TrackModelDesc,
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

struct TrackModelDesc {
    mesh_count: i32,
    scale: f32,
    length: f32,
    bank_angle: f32,
}

impl TrackModelDesc {
    fn new(track: &track_desc::Track, track_section: &track_sections::TrackSection) -> Self {
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
    fn new_alternating(track: &track_desc::Track, track_section: &track_sections::TrackSection) -> Self {
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
    track_model_desc: &TrackModelDesc,
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

    add_model_to_scene(
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

struct TrackSectionMeshIds {
    extrude_behind_mesh_ids: Vec<usize>,
    extrude_ahead_mesh_ids: Vec<usize>,
}

fn build_track_section<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    track_model_desc: &TrackModelDesc,
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

fn create_mask_scene<'a>(
    render_device: &'a renderer::Device,
    models: &'a track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    track_model_desc: &TrackModelDesc,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<renderer::Scene<'a>> {
    let mut scene = renderer::SceneBuilder::new(render_device)?;
    for i in -1..(track_model_desc.mesh_count + 1) {
        add_model_to_scene(
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

fn render_scene(
    scene: &renderer::Scene,
    mesh_types: &[renderer::MeshType],
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    rotation: usize,
) -> renderer::Framebuffer {
    let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation as f32);
    let camera = camera * view_rotation;

    let view_rotation_inverse = view_rotation.inverse();
    let lights = lights.iter().map(|x| x.transform(&view_rotation_inverse)).collect::<Vec<_>>();

    const EDGE_DISTANCE: f32 = 0.088_388_346;
    renderer::render_scene(scene, mesh_types, &camera, &lights, 4, 4, EDGE_DISTANCE)
}

fn render_scene_depth(scene: &renderer::Scene, camera: &glam::Mat4, rotation: usize) -> renderer::DepthBuffer {
    let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation as f32);
    let camera = camera * view_rotation;
    renderer::render_scene_depth(scene, &camera, 4, 4)
}

#[expect(clippy::too_many_arguments)]
fn render_track_section_view(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    models: &track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &TrackModelDesc,
    view: &mask::View,
    rotation: usize,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<(renderer::Framebuffer, Option<renderer::DepthBuffer>)> {
    let mut scene = renderer::SceneBuilder::new(render_device)?;
    let TrackSectionMeshIds {
        extrude_behind_mesh_ids,
        extrude_ahead_mesh_ids,
    } = build_track_section(&mut scene, models, track_section, model_desc, offset_start, offset_end)?;
    let (scene, mut mesh_types) = scene.build();

    if let Some(mesh_type) = view.extrude_behind_type {
        for mesh_type_index in &extrude_behind_mesh_ids {
            mesh_types[*mesh_type_index] = mesh_type;
        }
    }
    if let Some(mesh_type) = view.extrude_ahead_type {
        for mesh_type_index in &extrude_ahead_mesh_ids {
            mesh_types[*mesh_type_index] = mesh_type;
        }
    }

    let image = render_scene(&scene, &mesh_types, camera, lights, rotation);

    let mask_depth = if view.requires_track_mask {
        let scene = create_mask_scene(
            render_device,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
        )?;
        Some(render_scene_depth(&scene, camera, rotation))
    } else {
        None
    };

    Ok((image, mask_depth))
}

fn render_track_section_views(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    models: &track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &TrackModelDesc,
    views: &[mask::View],
) -> anyhow::Result<Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>> {
    use rayon::prelude::*;

    let offset = glam::Vec3::ZERO;

    let mut scene = renderer::SceneBuilder::new(render_device)?;
    let TrackSectionMeshIds {
        extrude_behind_mesh_ids,
        extrude_ahead_mesh_ids,
    } = build_track_section(&mut scene, models, track_section, model_desc, &offset, &offset)?;
    let (scene, mesh_types) = scene.build();

    let has_extrusions =
        views.iter().any(|x| x.extrude_behind_type.is_some()) || views.iter().any(|x| x.extrude_ahead_type.is_some());

    let images = if has_extrusions {
        let mut view_mesh_types = vec![mesh_types; views.len()];
        for (mesh_types, view) in view_mesh_types.iter_mut().zip(views.iter()) {
            if let Some(mesh_type) = view.extrude_behind_type {
                for mesh_type_index in &extrude_behind_mesh_ids {
                    mesh_types[*mesh_type_index] = mesh_type;
                }
            }
            if let Some(mesh_type) = view.extrude_ahead_type {
                for mesh_type_index in &extrude_ahead_mesh_ids {
                    mesh_types[*mesh_type_index] = mesh_type;
                }
            }
        }
        let view_mesh_types = view_mesh_types;

        view_mesh_types
            .into_par_iter()
            .enumerate()
            .map(|(rotation, mesh_types)| render_scene(&scene, &mesh_types, camera, lights, rotation))
            .collect::<Vec<_>>()
    } else {
        (0..views.len())
            .into_par_iter()
            .map(|rotation| render_scene(&scene, &mesh_types, camera, lights, rotation))
            .collect::<Vec<_>>()
    };

    let mask_depths = if views.iter().any(|x| x.requires_track_mask) {
        let scene = create_mask_scene(render_device, models, track_section, model_desc, &offset, &offset)?;
        views
            .into_par_iter()
            .enumerate()
            .map(|(rotation, view)| view.requires_track_mask.then(|| render_scene_depth(&scene, camera, rotation)))
            .collect::<Vec<_>>()
    } else {
        vec![None, None, None, None] // ehh
    };

    Ok(images.into_iter().zip(mask_depths).collect())
}

#[expect(clippy::too_many_arguments)]
fn render_track_section(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    models: &track_desc::Models<renderer::model::Model>,
    track: &track_desc::Track,
    offsets: Option<&track_desc::Offsets>,
    track_section: &track_sections::TrackSection,
    views: &[mask::View],
) -> anyhow::Result<Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>> {
    use rayon::prelude::*;

    let model_desc = if track.models.track_alt.is_some() {
        TrackModelDesc::new_alternating(track, track_section)
    } else {
        TrackModelDesc::new(track, track_section)
    };

    Ok(if let Some(offsets) = offsets {
        views
            .into_par_iter()
            .enumerate()
            .map(|(rotation, view)| {
                let offset_start = offset::calculate(offsets, track_section, model_desc.bank_angle, 0.0, rotation);
                let offset_end = offset::calculate(
                    offsets,
                    track_section,
                    model_desc.bank_angle,
                    track_section.length,
                    rotation,
                );
                render_track_section_view(
                    render_device,
                    camera,
                    lights,
                    models,
                    track_section,
                    &model_desc,
                    view,
                    rotation,
                    &offset_start,
                    &offset_end,
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?
    } else {
        render_track_section_views(render_device, camera, lights, models, track_section, &model_desc, views)?
    })
}

fn split_track_section(
    images: Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>,
    views: &[mask::View],
    dither: bool,
    track_section: &track_sections::TrackSection,
    track_z_offset: i32,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    for ((view_index, view), (image, mask_depth)) in views.iter().enumerate().zip(images) {
        let offset_offset = glam::IVec2::new(0, 16) + glam::IVec2::new(0, -track_z_offset);
        let mask_y_offset = if track_section.mask_offset_y {
            track_z_offset - 8 // offset masks are presumably made for track z_offset of 8 by default
        } else {
            0
        };

        let split_images = if let Some(mut mask_depth) = mask_depth {
            let track_depth = image.to_cropped_depth();
            let mut image = image.into_cropped_indexed_image(dither);

            image.offset += offset_offset;
            mask_depth.offset += offset_offset;

            split::split_image_depth(&image, view, mask_y_offset, &track_depth, &mask_depth)
        } else {
            let mut image = image.into_cropped_indexed_image(dither);
            image.offset += offset_offset;

            split::split_image(&image, view, mask_y_offset)
        };

        for (sprite_index, image) in split_images.iter().enumerate() {
            let image_name = if view.sprites.len() > 1 {
                format!("{}_{view_index}_{sprite_index}", track_section.name)
            } else {
                format!("{}_{view_index}", track_section.name)
            };
            image.save(&output_directory.join(image_name).with_extension("png"))?;
        }
    }

    Ok(())
}

fn list_track_sections(
    sections: &std::collections::HashSet<track_desc::TrackGroup>,
) -> Vec<&track_sections::TrackSection> {
    use track_desc::TrackGroup;

    let mut track_sections = Vec::new();

    if sections.contains(&TrackGroup::Flat) {
        track_sections.push(&track_sections::FLAT);
    }
    if sections.contains(&TrackGroup::GentleSlopes) {
        track_sections.push(&track_sections::FLAT_TO_GENTLE);
        track_sections.push(&track_sections::GENTLE_TO_FLAT);
        track_sections.push(&track_sections::GENTLE);
    }
    if sections.contains(&TrackGroup::SteepSlopes) {
        track_sections.push(&track_sections::GENTLE_TO_STEEP);
        track_sections.push(&track_sections::STEEP_TO_GENTLE);
        track_sections.push(&track_sections::STEEP);
    }
    if sections.contains(&TrackGroup::VerticalSlopes) {
        track_sections.push(&track_sections::STEEP_TO_VERTICAL);
        track_sections.push(&track_sections::VERTICAL_TO_STEEP);
        track_sections.push(&track_sections::VERTICAL);
    }
    if sections.contains(&TrackGroup::SmallSlopeTransitions) {
        track_sections.push(&track_sections::SMALL_FLAT_TO_STEEP);
        track_sections.push(&track_sections::SMALL_STEEP_TO_FLAT);
        track_sections.push(&track_sections::SMALL_FLAT_TO_STEEP_DIAG);
        track_sections.push(&track_sections::SMALL_STEEP_TO_FLAT_DIAG);
    }
    if sections.contains(&TrackGroup::LargeSlopeTransitions) {
        track_sections.push(&track_sections::FLAT_TO_STEEP);
        track_sections.push(&track_sections::STEEP_TO_FLAT);
        track_sections.push(&track_sections::FLAT_TO_STEEP_DIAG);
        track_sections.push(&track_sections::STEEP_TO_FLAT_DIAG);
    }
    if sections.contains(&TrackGroup::Turns) {
        track_sections.push(&track_sections::SMALL_TURN_LEFT);
        track_sections.push(&track_sections::MEDIUM_TURN_LEFT);
        track_sections.push(&track_sections::LARGE_TURN_LEFT_TO_DIAG);
        track_sections.push(&track_sections::LARGE_TURN_RIGHT_TO_DIAG);
    }
    if sections.contains(&TrackGroup::Diagonals) {
        track_sections.push(&track_sections::FLAT_DIAG);
    }
    if sections.contains(&TrackGroup::Diagonals) && sections.contains(&TrackGroup::GentleSlopes) {
        track_sections.push(&track_sections::FLAT_TO_GENTLE_DIAG);
        track_sections.push(&track_sections::GENTLE_TO_FLAT_DIAG);
        track_sections.push(&track_sections::GENTLE_DIAG);
    }
    if sections.contains(&TrackGroup::Diagonals) && sections.contains(&TrackGroup::SteepSlopes) {
        track_sections.push(&track_sections::GENTLE_TO_STEEP_DIAG);
        track_sections.push(&track_sections::STEEP_TO_GENTLE_DIAG);
        track_sections.push(&track_sections::STEEP_DIAG);
    }
    if sections.contains(&TrackGroup::BankedTurns) {
        track_sections.push(&track_sections::FLAT_TO_LEFT_BANK);
        track_sections.push(&track_sections::FLAT_TO_RIGHT_BANK);
        track_sections.push(&track_sections::LEFT_BANK_TO_GENTLE);
        track_sections.push(&track_sections::RIGHT_BANK_TO_GENTLE);
        track_sections.push(&track_sections::GENTLE_TO_LEFT_BANK);
        track_sections.push(&track_sections::GENTLE_TO_RIGHT_BANK);
        track_sections.push(&track_sections::LEFT_BANK);

        if sections.contains(&TrackGroup::Diagonals) {
            track_sections.push(&track_sections::FLAT_TO_LEFT_BANK_DIAG);
            track_sections.push(&track_sections::FLAT_TO_RIGHT_BANK_DIAG);
            track_sections.push(&track_sections::LEFT_BANK_TO_GENTLE_DIAG);
            track_sections.push(&track_sections::RIGHT_BANK_TO_GENTLE_DIAG);
            track_sections.push(&track_sections::GENTLE_TO_LEFT_BANK_DIAG);
            track_sections.push(&track_sections::GENTLE_TO_RIGHT_BANK_DIAG);
            track_sections.push(&track_sections::LEFT_BANK_DIAG);
        }

        track_sections.push(&track_sections::SMALL_TURN_LEFT_BANK);
        track_sections.push(&track_sections::MEDIUM_TURN_LEFT_BANK);
        track_sections.push(&track_sections::LARGE_TURN_LEFT_TO_DIAG_BANK);
        track_sections.push(&track_sections::LARGE_TURN_RIGHT_TO_DIAG_BANK);
    }
    if sections.contains(&TrackGroup::SlopedTurns) && sections.contains(&TrackGroup::GentleSlopes) {
        track_sections.push(&track_sections::SMALL_TURN_LEFT_GENTLE);
        track_sections.push(&track_sections::SMALL_TURN_RIGHT_GENTLE);
        track_sections.push(&track_sections::MEDIUM_TURN_LEFT_GENTLE);
        track_sections.push(&track_sections::MEDIUM_TURN_RIGHT_GENTLE);
    }
    if sections.contains(&TrackGroup::SlopedTurns) && sections.contains(&TrackGroup::SteepSlopes) {
        track_sections.push(&track_sections::VERY_SMALL_TURN_LEFT_STEEP);
        track_sections.push(&track_sections::VERY_SMALL_TURN_RIGHT_STEEP);
    }
    if sections.contains(&TrackGroup::SlopedTurns) && sections.contains(&TrackGroup::VerticalSlopes) {
        track_sections.push(&track_sections::VERTICAL_TWIST_LEFT);
        track_sections.push(&track_sections::VERTICAL_TWIST_RIGHT);
    }
    if sections.contains(&TrackGroup::BankedSlopedTurns) {
        track_sections.push(&track_sections::GENTLE_TO_GENTLE_LEFT_BANK);
        track_sections.push(&track_sections::GENTLE_TO_GENTLE_RIGHT_BANK);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_GENTLE);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_GENTLE);
        track_sections.push(&track_sections::LEFT_BANK_TO_GENTLE_LEFT_BANK);
        track_sections.push(&track_sections::RIGHT_BANK_TO_GENTLE_RIGHT_BANK);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_LEFT_BANK);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_RIGHT_BANK);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK);
        track_sections.push(&track_sections::FLAT_TO_GENTLE_LEFT_BANK);
        track_sections.push(&track_sections::FLAT_TO_GENTLE_RIGHT_BANK);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_FLAT);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_FLAT);
        track_sections.push(&track_sections::SMALL_TURN_LEFT_BANK_GENTLE);
        track_sections.push(&track_sections::SMALL_TURN_RIGHT_BANK_GENTLE);
        track_sections.push(&track_sections::MEDIUM_TURN_LEFT_BANK_GENTLE);
        track_sections.push(&track_sections::MEDIUM_TURN_RIGHT_BANK_GENTLE);
    }
    if sections.contains(&TrackGroup::SBends) {
        track_sections.push(&track_sections::S_BEND_LEFT);
        track_sections.push(&track_sections::S_BEND_RIGHT);
    }
    if sections.contains(&TrackGroup::Helices) {
        track_sections.push(&track_sections::SMALL_HELIX_LEFT);
        track_sections.push(&track_sections::SMALL_HELIX_RIGHT);
        track_sections.push(&track_sections::MEDIUM_HELIX_LEFT);
        track_sections.push(&track_sections::MEDIUM_HELIX_RIGHT);
    }
    if sections.contains(&TrackGroup::TurnBankTransitions) {
        track_sections.push(&track_sections::SMALL_TURN_LEFT_BANK_TO_GENTLE);
        track_sections.push(&track_sections::SMALL_TURN_RIGHT_BANK_TO_GENTLE);
    }
    if sections.contains(&TrackGroup::BarrelRolls) {
        track_sections.push(&track_sections::BARREL_ROLL_LEFT);
        track_sections.push(&track_sections::BARREL_ROLL_RIGHT);
    }
    if sections.contains(&TrackGroup::InlineTwists) {
        track_sections.push(&track_sections::INLINE_TWIST_LEFT);
        track_sections.push(&track_sections::INLINE_TWIST_RIGHT);
    }
    if sections.contains(&TrackGroup::HalfLoops) {
        track_sections.push(&track_sections::HALF_LOOP);
    }
    if sections.contains(&TrackGroup::VerticalLoops) {
        track_sections.push(&track_sections::VERTICAL_LOOP_LEFT);
        track_sections.push(&track_sections::VERTICAL_LOOP_RIGHT);
    }
    if sections.contains(&TrackGroup::QuarterLoops) {
        track_sections.push(&track_sections::QUARTER_LOOP);
    }
    if sections.contains(&TrackGroup::Corkscrews) {
        track_sections.push(&track_sections::CORKSCREW_LEFT);
        track_sections.push(&track_sections::CORKSCREW_RIGHT);
    }
    if sections.contains(&TrackGroup::LargeCorkscrews) {
        track_sections.push(&track_sections::LARGE_CORKSCREW_LEFT);
        track_sections.push(&track_sections::LARGE_CORKSCREW_RIGHT);
    }
    if sections.contains(&TrackGroup::MediumHalfLoops) {
        track_sections.push(&track_sections::MEDIUM_HALF_LOOP_LEFT);
        track_sections.push(&track_sections::MEDIUM_HALF_LOOP_RIGHT);
    }
    if sections.contains(&TrackGroup::LargeHalfLoops) {
        track_sections.push(&track_sections::LARGE_HALF_LOOP_LEFT);
        track_sections.push(&track_sections::LARGE_HALF_LOOP_RIGHT);
    }
    if sections.contains(&TrackGroup::ZeroGRolls) {
        track_sections.push(&track_sections::ZERO_G_ROLL_LEFT);
        track_sections.push(&track_sections::ZERO_G_ROLL_RIGHT);
        track_sections.push(&track_sections::LARGE_ZERO_G_ROLL_LEFT);
        track_sections.push(&track_sections::LARGE_ZERO_G_ROLL_RIGHT);
    }
    if sections.contains(&TrackGroup::DiveLoops) {
        track_sections.push(&track_sections::DIVE_LOOP_45_LEFT);
        track_sections.push(&track_sections::DIVE_LOOP_45_RIGHT);
    }
    if sections.contains(&TrackGroup::LargeSlopedTurns) {
        track_sections.push(&track_sections::LARGE_TURN_LEFT_TO_DIAG_GENTLE);
        track_sections.push(&track_sections::LARGE_TURN_RIGHT_TO_DIAG_GENTLE);
        track_sections.push(&track_sections::LARGE_TURN_LEFT_TO_ORTHOGONAL_GENTLE);
        track_sections.push(&track_sections::LARGE_TURN_RIGHT_TO_ORTHOGONAL_GENTLE);
    }
    if sections.contains(&TrackGroup::LargeBankedSlopedTurns) {
        track_sections.push(&track_sections::GENTLE_TO_GENTLE_LEFT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_TO_GENTLE_RIGHT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_GENTLE_DIAG);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_GENTLE_DIAG);
        track_sections.push(&track_sections::LEFT_BANK_TO_GENTLE_LEFT_BANK_DIAG);
        track_sections.push(&track_sections::RIGHT_BANK_TO_GENTLE_RIGHT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_LEFT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_RIGHT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_DIAG);
        track_sections.push(&track_sections::FLAT_TO_GENTLE_LEFT_BANK_DIAG);
        track_sections.push(&track_sections::FLAT_TO_GENTLE_RIGHT_BANK_DIAG);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_FLAT_DIAG);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_FLAT_DIAG);
        track_sections.push(&track_sections::LARGE_TURN_LEFT_BANK_TO_DIAG_GENTLE);
        track_sections.push(&track_sections::LARGE_TURN_RIGHT_BANK_TO_DIAG_GENTLE);
        track_sections.push(&track_sections::LARGE_TURN_LEFT_BANK_TO_ORTHOGONAL_GENTLE);
        track_sections.push(&track_sections::LARGE_TURN_RIGHT_BANK_TO_ORTHOGONAL_GENTLE);
    }

    track_sections
}

fn render(
    track_desc: &track_desc::Desc,
    data_directory: &std::path::Path,
    base_directory: &std::path::Path,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    use anyhow::Context as _;
    use rayon::prelude::*;

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    let camera = glam::Mat4::from_mat3(
        glam::Mat3::from_cols(
            glam::Vec3::new(32.0, 0.0, 32.0),
            glam::Vec3::new(16.0, -16.0 * 6.0_f32.sqrt(), -16.0),
            glam::Vec3::new(-16.0 * 3.0_f32.sqrt(), -16.0 * 2.0_f32.sqrt(), 16.0 * 3.0_f32.sqrt()),
        )
        .transpose(),
    );

    let lights = track_desc.get_lights();

    for track in &track_desc.tracks {
        let models = track.models.load(base_directory)?;

        let masks = mask::Masks::load(&data_directory.join("masks").join(&track.masks).with_extension("json"))?;

        let output_directory = output_directory.join(&track.name);
        std::fs::create_dir_all(&output_directory)?;

        let track_sections = list_track_sections(&track.sections);

        track_sections
            .into_par_iter()
            .map(|track_section| {
                if let Some(views) = masks.get_views(track_section.name) {
                    render_track_section(
                        &render_device,
                        &camera,
                        &lights,
                        &models,
                        track,
                        track_desc.offsets.as_ref(),
                        track_section,
                        views,
                    )
                    .and_then(|images| {
                        split_track_section(
                            images,
                            views,
                            track_desc.dither,
                            track_section,
                            track.z_offset as i32,
                            &output_directory,
                        )
                    })
                } else {
                    Ok(())
                }
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
    }

    Ok(())
}

pub fn make_track(
    data_directory: &std::path::Path,
    track_description_file_path: &std::path::Path,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let desc = track_desc::Desc::load(track_description_file_path)?;

    let base_directory = track_description_file_path.parent().with_context(|| {
        format!(
            "Could not get parent directory of {}",
            track_description_file_path.display()
        )
    })?;

    render(&desc, data_directory, base_directory, output_directory)?;

    Ok(())
}
