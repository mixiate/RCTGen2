mod chain;
mod curves;
mod mask;
mod offset;
mod split;
mod sprites_json;
mod track_curves;
mod track_desc;
mod track_model;
mod track_sections;

const CLEARANCE_HEIGHT: f32 = 0.20412415; // 1.0 / (2.0 * 6.0.sqrt())

fn render_scene(
    scene: &renderer::Scene,
    mesh_types: &[renderer::MeshType],
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    edge_distance: f32,
    rotation: usize,
) -> renderer::Framebuffer {
    let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation as f32);
    let camera = camera * view_rotation;

    let view_rotation_inverse = view_rotation.inverse();
    let lights = lights.iter().map(|x| x.transform(&view_rotation_inverse)).collect::<Vec<_>>();

    renderer::render_scene(scene, mesh_types, &camera, &lights, 4, 4, edge_distance)
}

fn render_scene_depth(
    scene: &renderer::Scene,
    mesh_types: &[renderer::MeshType],
    camera: &glam::Mat4,
    rotation: usize,
) -> renderer::DepthBuffer {
    let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation as f32);
    let camera = camera * view_rotation;
    renderer::render_scene_depth(scene, mesh_types, &camera, 4, 4)
}

#[expect(clippy::too_many_arguments)]
fn render_track_section_view(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    edge_distance: f32,
    models: &track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &track_model::ModelDesc,
    view: &mask::View,
    rotation: usize,
    offset_start: &glam::Vec3,
    offset_end: &glam::Vec3,
) -> anyhow::Result<(renderer::Framebuffer, Option<renderer::DepthBuffer>)> {
    let mut scene = renderer::SceneBuilder::new(render_device)?;
    let track_model::TrackSectionMeshIds {
        extrude_behind_mesh_ids,
        extrude_ahead_mesh_ids,
    } = track_model::build(&mut scene, models, track_section, model_desc, offset_start, offset_end)?;
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

    let image = render_scene(&scene, &mesh_types, camera, lights, edge_distance, rotation);

    let mask_depth = if view.requires_track_mask {
        let (scene, mesh_types) = track_model::build_mask(
            render_device,
            models,
            track_section,
            model_desc,
            offset_start,
            offset_end,
        )?;
        Some(render_scene_depth(&scene, &mesh_types, camera, rotation))
    } else {
        None
    };

    Ok((image, mask_depth))
}

#[expect(clippy::too_many_arguments)]
fn render_track_section_views(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    edge_distance: f32,
    models: &track_desc::Models<renderer::model::Model>,
    track_section: &track_sections::TrackSection,
    model_desc: &track_model::ModelDesc,
    views: &[mask::View],
) -> anyhow::Result<Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>> {
    use rayon::prelude::*;

    let offset = glam::Vec3::ZERO;

    let mut scene = renderer::SceneBuilder::new(render_device)?;
    let track_model::TrackSectionMeshIds {
        extrude_behind_mesh_ids,
        extrude_ahead_mesh_ids,
    } = track_model::build(&mut scene, models, track_section, model_desc, &offset, &offset)?;
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
            .map(|(rotation, mesh_types)| render_scene(&scene, &mesh_types, camera, lights, edge_distance, rotation))
            .collect::<Vec<_>>()
    } else {
        (0..views.len())
            .into_par_iter()
            .map(|rotation| render_scene(&scene, &mesh_types, camera, lights, edge_distance, rotation))
            .collect::<Vec<_>>()
    };

    let mask_depths = if views.iter().any(|x| x.requires_track_mask) {
        let (scene, mesh_types) =
            track_model::build_mask(render_device, models, track_section, model_desc, &offset, &offset)?;
        views
            .into_par_iter()
            .enumerate()
            .map(|(rotation, view)| {
                view.requires_track_mask.then(|| render_scene_depth(&scene, &mesh_types, camera, rotation))
            })
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
    edge_distance: f32,
    models: &track_desc::Models<renderer::model::Model>,
    model_lengths: &track_model::ModelLengths,
    track: &track_desc::Track,
    offsets: Option<&track_desc::Offsets>,
    track_section: &track_sections::TrackSection,
    views: &[mask::View],
) -> anyhow::Result<Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>> {
    use rayon::prelude::*;

    Ok(if offsets.is_some() || models.track_tie.is_some() {
        views
            .into_par_iter()
            .enumerate()
            .map(|(rotation, view)| {
                let model_desc = track_model::ModelDesc::new(track, models, model_lengths, track_section, rotation);
                let (offset_start, offset_end) = if let Some(offsets) = offsets {
                    let offset_start = offset::calculate(offsets, track_section, model_desc.bank_angle, 0.0, rotation);
                    let offset_end = offset::calculate(
                        offsets,
                        track_section,
                        model_desc.bank_angle,
                        track_section.length,
                        rotation,
                    );
                    (offset_start, offset_end)
                } else {
                    (glam::Vec3::ZERO, glam::Vec3::ZERO)
                };
                render_track_section_view(
                    render_device,
                    camera,
                    lights,
                    edge_distance,
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
        let model_desc = track_model::ModelDesc::new(track, models, model_lengths, track_section, 0);
        render_track_section_views(
            render_device,
            camera,
            lights,
            edge_distance,
            models,
            track_section,
            &model_desc,
            views,
        )?
    })
}

fn is_sprite_empty(image: &renderer::image::IndexedImage) -> bool {
    image.width() == 1 && image.height() == 1 && image.get_pixel(0, 0) == 0
}

fn output_track_section(
    images: Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>,
    dither: bool,
    track_section: &track_sections::TrackSection,
    track: &track_desc::Track,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    for (index, (image, _)) in images.into_iter().enumerate() {
        let image = image.into_cropped_indexed_image(dither);

        let image_name = if let Some(suffix) = &track.suffix {
            format!("{}_{suffix}_{index}", track_section.name)
        } else {
            format!("{}_{index}", track_section.name)
        };
        image.save(&output_directory.join(&image_name).with_extension("png"))?;
    }

    Ok(())
}

fn split_track_section(
    images: Vec<(renderer::Framebuffer, Option<renderer::DepthBuffer>)>,
    views: &[mask::View],
    dither: bool,
    track_section: &track_sections::TrackSection,
    track: &track_desc::Track,
    skip_empty_sprites: bool,
    output_directory: &std::path::Path,
) -> anyhow::Result<Vec<openrct2::objects::image::ImageFile>> {
    let mut sprite_descs = Vec::new();
    for ((view_index, view), (image, mask_depth)) in
        views.iter().filter(|x| track.lift || !x.optional).enumerate().zip(images)
    {
        let offset_offset = glam::IVec2::new(0, 16) + glam::IVec2::new(0, -track.z_offset);
        let mask_y_offset = if track_section.mask_offset_y {
            track.z_offset - 8 // offset masks are presumably made for track z_offset of 8 by default
        } else {
            0
        };

        let split_images = if let Some(mut mask_depth) = mask_depth {
            let track_depth = image.to_cropped_depth();
            let mut image = image.into_cropped_indexed_image(dither);
            if track.lift
                && let Some(chain_type) = track_section.chain_type
            {
                chain::apply_chain(&mut image, chain_type, view_index);
            }

            image.offset += offset_offset;
            mask_depth.offset += offset_offset;

            split::split_image_depth(&image, view, mask_y_offset, &track_depth, &mask_depth)
        } else {
            let mut image = image.into_cropped_indexed_image(dither);
            if track.lift
                && let Some(chain_type) = track_section.chain_type
            {
                chain::apply_chain(&mut image, chain_type, view_index);
            }
            image.offset += offset_offset;

            split::split_image(&image, view, mask_y_offset)
        };

        for (sprite_index, image) in split_images
            .iter()
            .filter(|x| if skip_empty_sprites { !is_sprite_empty(x) } else { true })
            .enumerate()
        {
            let image_name = if let Some(suffix) = &track.suffix {
                format!("{}_{suffix}_{view_index}", track_section.name)
            } else {
                format!("{}_{view_index}", track_section.name)
            };
            let image_name = if view.sprites.len() > 1 {
                format!("{image_name}_{sprite_index}")
            } else {
                image_name
            };
            image.save(&output_directory.join(&image_name).with_extension("png"))?;

            let relative_file_path = format!("track/{}/{image_name}.png", track.name);
            sprite_descs.push(openrct2::objects::image::ImageFile {
                path: relative_file_path.clone(),
                x: Some(image.offset.x),
                y: Some(image.offset.y),
                palette: Some(openrct2::objects::image::PaletteType::Keep),
                ..Default::default()
            });
        }
    }

    Ok(sprite_descs)
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
    }
    if sections.contains(&TrackGroup::SmallSlopeTransitionsDiagonal) {
        track_sections.push(&track_sections::SMALL_FLAT_TO_STEEP_DIAG);
        track_sections.push(&track_sections::SMALL_STEEP_TO_FLAT_DIAG);
    }
    if sections.contains(&TrackGroup::LargeSlopeTransitions) {
        track_sections.push(&track_sections::FLAT_TO_STEEP);
        track_sections.push(&track_sections::STEEP_TO_FLAT);
    }
    if sections.contains(&TrackGroup::LargeSlopeTransitionsDiagonal) {
        track_sections.push(&track_sections::FLAT_TO_STEEP_DIAG);
        track_sections.push(&track_sections::STEEP_TO_FLAT_DIAG);
    }
    if sections.contains(&TrackGroup::VerySmallTurns) {
        track_sections.push(&track_sections::VERY_SMALL_TURN_LEFT);
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
    if sections.contains(&TrackGroup::BankedSBends) {
        track_sections.push(&track_sections::S_BEND_LEFT_BANK);
        track_sections.push(&track_sections::S_BEND_RIGHT_BANK);
    }
    if sections.contains(&TrackGroup::Helices) {
        track_sections.push(&track_sections::SMALL_HELIX_LEFT);
        track_sections.push(&track_sections::SMALL_HELIX_RIGHT);
        track_sections.push(&track_sections::MEDIUM_HELIX_LEFT);
        track_sections.push(&track_sections::MEDIUM_HELIX_RIGHT);
    }
    if sections.contains(&TrackGroup::MediumQuarterHelices) {
        track_sections.push(&track_sections::MEDIUM_QUARTER_HELIX_LEFT);
        track_sections.push(&track_sections::MEDIUM_QUARTER_HELIX_RIGHT);
    }
    if sections.contains(&TrackGroup::MediumBankedQuarterHelices) {
        track_sections.push(&track_sections::MEDIUM_QUARTER_HELIX_LEFT_BANK);
        track_sections.push(&track_sections::MEDIUM_QUARTER_HELIX_RIGHT_BANK);
    }
    if sections.contains(&TrackGroup::TurnBankTransitions) {
        track_sections.push(&track_sections::SMALL_TURN_LEFT_BANK_TO_GENTLE);
        track_sections.push(&track_sections::SMALL_TURN_RIGHT_BANK_TO_GENTLE);
    }
    if sections.contains(&TrackGroup::SteepBankTransitions) {
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_STEEP);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_STEEP);
        track_sections.push(&track_sections::STEEP_TO_GENTLE_LEFT_BANK);
        track_sections.push(&track_sections::STEEP_TO_GENTLE_RIGHT_BANK);
        track_sections.push(&track_sections::GENTLE_LEFT_BANK_TO_STEEP_DIAG);
        track_sections.push(&track_sections::GENTLE_RIGHT_BANK_TO_STEEP_DIAG);
        track_sections.push(&track_sections::STEEP_TO_GENTLE_LEFT_BANK_DIAG);
        track_sections.push(&track_sections::STEEP_TO_GENTLE_RIGHT_BANK_DIAG);
    }
    if sections.contains(&TrackGroup::LargeSteepSlopedTurns) {
        track_sections.push(&track_sections::SMALL_TURN_LEFT_STEEP);
        track_sections.push(&track_sections::SMALL_TURN_RIGHT_STEEP);
    }
    if sections.contains(&TrackGroup::BarrelRolls) {
        track_sections.push(&track_sections::BARREL_ROLL_LEFT);
        track_sections.push(&track_sections::BARREL_ROLL_RIGHT);
    }
    if sections.contains(&TrackGroup::BankedBarrelRolls) {
        track_sections.push(&track_sections::BANKED_BARREL_ROLL_LEFT);
        track_sections.push(&track_sections::BANKED_BARREL_ROLL_RIGHT);
    }
    if sections.contains(&TrackGroup::InlineTwists) {
        track_sections.push(&track_sections::INLINE_TWIST_LEFT);
        track_sections.push(&track_sections::INLINE_TWIST_RIGHT);
    }
    if sections.contains(&TrackGroup::BankedInlineTwists) {
        track_sections.push(&track_sections::BANKED_INLINE_TWIST_LEFT);
        track_sections.push(&track_sections::BANKED_INLINE_TWIST_RIGHT);
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
    if sections.contains(&TrackGroup::BankedZeroGRolls) {
        track_sections.push(&track_sections::BANKED_ZERO_G_ROLL_LEFT);
        track_sections.push(&track_sections::BANKED_ZERO_G_ROLL_RIGHT);
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

struct TrackSectionSprites {
    track_name: String,
    track_section_name: String,
    sprites: Vec<openrct2::objects::image::ImageFile>,
}

fn render(
    track_desc: &track_desc::Desc,
    data_directory: &std::path::Path,
    base_directory: &std::path::Path,
    output_directory: &std::path::Path,
    skip_empty_sprites: bool,
) -> anyhow::Result<Vec<TrackSectionSprites>> {
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

    let edge_distance = track_desc.edge_distance.unwrap_or(0.088388346);

    let mut sprite_descs = Vec::new();
    for track in &track_desc.tracks {
        let models = track.models.load(base_directory)?;
        let model_lengths = track_model::ModelLengths::calculate(track, &models);

        let masks = mask::Masks::load(&data_directory.join("masks").join(&track.masks).with_extension("json"))?;

        let output_directory = output_directory.join("track").join(&track.name);
        std::fs::create_dir_all(&output_directory)?;

        let track_sections = list_track_sections(&track.sections);

        let track_section_sprite_descs = track_sections
            .into_par_iter()
            .map(|track_section| {
                let track_section_name = if let Some(suffix) = &track.suffix {
                    format!("{}_{suffix}", track_section.name)
                } else {
                    track_section.name.to_owned()
                };
                if let Some(views) = masks.get_views(track_section.name) {
                    let images = render_track_section(
                        &render_device,
                        &camera,
                        &lights,
                        edge_distance,
                        &models,
                        &model_lengths,
                        track,
                        track_desc.offsets.as_ref(),
                        track_section,
                        views,
                    )?;
                    let sprites = split_track_section(
                        images,
                        views,
                        track_desc.dither,
                        track_section,
                        track,
                        skip_empty_sprites,
                        &output_directory,
                    )?;
                    Ok(TrackSectionSprites {
                        track_name: track.name.clone(),
                        track_section_name,
                        sprites,
                    })
                } else {
                    println!("No mask found for {}.", track_section.name);
                    let views = [
                        mask::View::default(),
                        mask::View::default(),
                        mask::View::default(),
                        mask::View::default(),
                    ];
                    let images = render_track_section(
                        &render_device,
                        &camera,
                        &lights,
                        edge_distance,
                        &models,
                        &model_lengths,
                        track,
                        track_desc.offsets.as_ref(),
                        track_section,
                        &views,
                    )?;
                    output_track_section(images, track_desc.dither, track_section, track, &output_directory)?;
                    Ok(TrackSectionSprites {
                        track_name: String::new(),
                        track_section_name: String::new(),
                        sprites: Vec::new(),
                    })
                }
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        for sprite_desc in track_section_sprite_descs {
            sprite_descs.push(sprite_desc);
        }
    }

    Ok(sprite_descs)
}

fn is_track_section_sprite(sprite_path: &str, path_prefix: &str, track_section_name: &str) -> bool {
    if let Some(sprite_name) = sprite_path.strip_prefix(path_prefix)
        && sprite_name.starts_with(track_section_name)
        && let Some(character) = sprite_name.get((track_section_name.len() + 1)..(track_section_name.len() + 2))
        && character.parse::<i32>().is_ok()
    {
        true
    } else {
        false
    }
}

fn output_sprites_json(sprites: Vec<TrackSectionSprites>, output_directory: &std::path::Path) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let sprites_json_file_path = output_directory.join("sprites").with_extension("json");
    if let Ok(json_string) = std::fs::read_to_string(&sprites_json_file_path) {
        let mut sprites_json = serde_json::from_str::<sprites_json::Sprites>(&json_string)
            .with_context(|| format!("Could not parse json in file {}", sprites_json_file_path.display()))?;

        for track_section_sprites in sprites {
            if track_section_sprites.sprites.is_empty() {
                continue;
            }

            let path_prefix = format!("track/{}/", track_section_sprites.track_name);
            let track_section_name = &track_section_sprites.track_section_name;

            if let Some(index) = sprites_json
                .sprites
                .iter()
                .position(|sprite| is_track_section_sprite(&sprite.path, &path_prefix, track_section_name))
            {
                sprites_json
                    .sprites
                    .retain(|sprite| !is_track_section_sprite(&sprite.path, &path_prefix, track_section_name));
                sprites_json.sprites.splice(index..index, track_section_sprites.sprites);
            } else if let Some(index) =
                sprites_json.sprites.iter().rev().position(|sprite| sprite.path.starts_with(&path_prefix))
            {
                let index = sprites_json.sprites.len() - index;
                sprites_json.sprites.splice(index..index, track_section_sprites.sprites);
            } else {
                sprites_json.sprites.extend(track_section_sprites.sprites);
            }
        }
        sprites_json.save(&sprites_json_file_path)?;
    } else {
        let sprites = sprites.into_iter().flat_map(|x| x.sprites).collect();
        let sprites_json = sprites_json::Sprites { sprites };
        sprites_json.save(&sprites_json_file_path)?;
    }
    Ok(())
}

pub fn make_track(
    data_directory: &std::path::Path,
    track_description_file_path: &std::path::Path,
    output_directory: &std::path::Path,
    skip_empty_sprites: bool,
) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let desc = track_desc::Desc::load(track_description_file_path)?;

    let base_directory = track_description_file_path.parent().with_context(|| {
        format!(
            "Could not get parent directory of {}",
            track_description_file_path.display()
        )
    })?;

    let sprites = render(
        &desc,
        data_directory,
        base_directory,
        output_directory,
        skip_empty_sprites,
    )?;
    output_sprites_json(sprites, output_directory)?;

    Ok(())
}
