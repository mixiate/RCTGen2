use crate::track_editor::TrackEditorMessage;
use eframe::egui;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct UpdateModelArgs {
    pub track_section: &'static make_track::track_sections::TrackSection,
    pub rotation: usize,
}

pub struct RenderArgs {
    pub egui_context: egui::Context,
    pub rotation: usize,
    pub samples: usize,
    pub dither: bool,
    pub edge_distance: Option<f32>,
    pub lights: Vec<renderer::Light>,
}

pub enum RenderMessage {
    SetDirectory(std::path::PathBuf),
    UpdateModelSettings(make_track::track_desc::ModelSettings),
    LoadModels(Box<make_track::track_desc::Models<relative_path::RelativePathBuf>>),
    LoadMasks(String),
    UpdateOffsets(Box<Option<make_track::track_desc::Offsets>>),
    UpdateModel(UpdateModelArgs),
    Render(RenderArgs),
    Exit,
}

pub struct Images {
    pub unindexed: renderer::image::Image,
    pub indexed: renderer::image::IndexedImage,
}

pub struct TrackImage {
    pub images: Images,
    pub track_section: &'static make_track::track_sections::TrackSection,
    pub rotation: usize,
}

pub type SharedTrackImage = Arc<Mutex<Option<TrackImage>>>;

fn load_masks(
    name: &str,
    data_directory: &std::path::Path,
) -> anyhow::Result<HashMap<String, [make_track::mask::View; 4]>> {
    let masks_directory = data_directory.join("masks");
    let masks_file_path = masks_directory.join(name).with_extension("json");
    let masks = make_track::mask::load_masks(&masks_file_path)?;

    let mut output_masks = HashMap::new();
    for (track_section_name, views) in masks {
        if let Some(track_section) =
            make_track::track_sections::TRACK_SECTIONS.iter().find(|x| x.name == track_section_name)
        {
            let views = views.load(&masks_directory, &track_section.tiles)?;
            output_masks.insert(track_section_name, views);
        }
    }
    Ok(output_masks)
}

struct Scene<'a> {
    scene: renderer::Scene<'a>,
    mesh_types: Vec<renderer::MeshType>,
}

fn update_model<'a>(
    args: &UpdateModelArgs,
    render_device: &'a renderer::Device,
    model_settings: &make_track::track_desc::ModelSettings,
    models: &'a make_track::track_desc::Models<renderer::model::Model>,
    lengths: &make_track::track_model::ModelLengths,
    offsets: Option<&make_track::track_desc::Offsets>,
    views: Option<&[make_track::mask::View; 4]>,
) -> anyhow::Result<Scene<'a>> {
    let model_desc =
        make_track::track_model::ModelDesc::new(model_settings, models, lengths, args.track_section, args.rotation);
    let (offset_start, offset_end) = if let Some(offsets) = offsets {
        let offset_start =
            make_track::offset::calculate(offsets, args.track_section, model_desc.bank_angle, 0.0, args.rotation);
        let offset_end = make_track::offset::calculate(
            offsets,
            args.track_section,
            model_desc.bank_angle,
            args.track_section.length,
            args.rotation,
        );
        (offset_start, offset_end)
    } else {
        (glam::Vec3::ZERO, glam::Vec3::ZERO)
    };
    let mut scene = renderer::SceneBuilder::new(render_device)?;
    let make_track::track_model::TrackSectionMeshIds {
        extrude_behind_mesh_ids,
        extrude_ahead_mesh_ids,
    } = make_track::track_model::build(
        &mut scene,
        models,
        args.track_section,
        &model_desc,
        &offset_start,
        &offset_end,
    )?;
    let (scene, mut mesh_types) = scene.build();

    if let Some(view) = views.as_ref().and_then(|x| x.get(args.rotation)) {
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

    Ok(Scene { scene, mesh_types })
}

fn render(scene: &Scene, args: &mut RenderArgs, views: Option<&[make_track::mask::View; 4]>) -> Images {
    let camera = glam::Mat4::from_mat3(
        glam::Mat3::from_cols(
            glam::Vec3::new(32.0, 0.0, 32.0),
            glam::Vec3::new(16.0, -16.0 * 6.0_f32.sqrt(), -16.0),
            glam::Vec3::new(-16.0 * 3.0_f32.sqrt(), -16.0 * 2.0_f32.sqrt(), 16.0 * 3.0_f32.sqrt()),
        )
        .transpose(),
    );

    let view_rotation = glam::Mat4::from_rotation_y(args.rotation as f32 * 90.0_f32.to_radians());
    let camera = camera * view_rotation;

    let view_rotation_inverse = view_rotation.inverse();
    for light in &mut args.lights {
        light.direction = view_rotation_inverse.transform_vector3(light.direction).normalize();
    }
    let framebuffer = renderer::render_scene(
        &scene.scene,
        &scene.mesh_types,
        &camera,
        &args.lights,
        args.samples,
        args.samples,
        args.edge_distance.unwrap_or(0.088388346),
    );

    let unindexed = framebuffer.to_image();
    let mut indexed = framebuffer.into_indexed_image(args.dither);

    if let Some(view) = views.as_ref().and_then(|x| x.get(args.rotation)) {
        for y in 0..indexed.height() {
            for x in 0..indexed.width() {
                let mask_x = indexed.offset.x + i32::from(x);
                let mask_y = indexed.offset.y + i32::from(y);

                if view.sample_primary(mask_x, mask_y, 0) {
                    indexed.set_pixel(x.into(), y.into(), 0);
                }
            }
        }
    }

    Images { unindexed, indexed }
}

fn report_error(tx: &Sender<TrackEditorMessage>, error: &anyhow::Error) {
    let errors = error.chain().map(|x| x.to_string()).collect();
    let _result = tx.send(TrackEditorMessage::Error(errors));
}

pub fn render_thread(
    render_rx: &Receiver<RenderMessage>,
    app_tx: &Sender<TrackEditorMessage>,
    track_image: &SharedTrackImage,
    data_directory: &std::path::Path,
) {
    let render_device = match renderer::Device::try_new() {
        Ok(render_device) => render_device,
        Err(_) => {
            let _result = app_tx.send(TrackEditorMessage::Error(vec![
                "Could not create render device".to_string(),
            ]));
            return;
        }
    };

    let mut current_directory = None;
    let mut current_model_settings = None;
    let mut current_models = None;
    let mut current_model_lengths = None;
    let mut current_masks = None;
    let mut current_offsets = None;
    let mut current_scene = None;
    let mut current_track_section = &make_track::track_sections::FLAT;

    let mut messages = Vec::new();

    'main_loop: loop {
        match render_rx.recv() {
            Ok(message) => messages.push(message),
            Err(_) => break,
        }
        messages.extend(render_rx.try_iter());

        let render_message = messages.iter().rposition(|x| matches!(x, RenderMessage::Render(_)));
        let render_message = render_message.map(|index| messages.remove(index));

        for message in messages.drain(0..) {
            match message {
                RenderMessage::SetDirectory(directory) => current_directory = Some(directory),
                RenderMessage::UpdateModelSettings(settings) => {
                    current_model_settings = Some(settings);
                    if let Some(models) = &current_models {
                        current_model_lengths =
                            Some(make_track::track_model::ModelLengths::calculate(&settings, models));
                    }
                }
                RenderMessage::LoadModels(models) => {
                    if let Some(directory) = &current_directory
                        && let Some(settings) = &current_model_settings
                    {
                        match models.load(directory) {
                            Ok(models) => {
                                let lengths = make_track::track_model::ModelLengths::calculate(settings, &models);
                                current_scene = None;
                                current_models = Some(models);
                                current_model_lengths = Some(lengths);
                            }
                            Err(error) => report_error(app_tx, &error),
                        }
                    }
                }
                RenderMessage::LoadMasks(name) => match load_masks(&name, data_directory) {
                    Ok(masks) => {
                        current_masks = Some(masks);
                    }
                    Err(error) => {
                        current_masks = None;
                        report_error(app_tx, &error);
                    }
                },
                RenderMessage::UpdateOffsets(offsets) => {
                    current_scene = None;
                    current_offsets = *offsets;
                }
                RenderMessage::UpdateModel(args) => {
                    if let Some(settings) = &current_model_settings
                        && let Some(models) = &current_models
                        && let Some(lengths) = &current_model_lengths
                    {
                        match update_model(
                            &args,
                            &render_device,
                            settings,
                            models,
                            lengths,
                            current_offsets.as_ref(),
                            current_masks.as_ref().and_then(|x| x.get(args.track_section.name)),
                        ) {
                            Ok(scene) => {
                                current_scene = Some(scene);
                                current_track_section = args.track_section;
                            }
                            Err(error) => report_error(app_tx, &error),
                        }
                    }
                }
                RenderMessage::Render(_) => {}
                RenderMessage::Exit => break 'main_loop,
            }
        }

        if let Some(RenderMessage::Render(mut args)) = render_message
            && let Some(scene) = &current_scene
        {
            let images = render(
                scene,
                &mut args,
                current_masks.as_ref().and_then(|x| x.get(current_track_section.name)),
            );

            if let Ok(mut track_image) = track_image.lock() {
                *track_image = Some(TrackImage {
                    images,
                    track_section: current_track_section,
                    rotation: args.rotation,
                });
            }

            args.egui_context.request_repaint();
            let _result = app_tx.send(TrackEditorMessage::NewFrame);
        }
    }
}
