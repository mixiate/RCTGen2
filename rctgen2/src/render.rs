use crate::app::AppMessage;
use eframe::egui;
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
    LoadTrack(Box<make_track::track_desc::Track>),
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

struct Track {
    track: make_track::track_desc::Track,
    track_models: make_track::track_desc::Models<renderer::model::Model>,
    lengths: make_track::track_model::ModelLengths,
}

struct Scene<'a> {
    scene: renderer::Scene<'a>,
    mesh_types: Vec<renderer::MeshType>,
}

fn load_track(track: make_track::track_desc::Track, directory: &std::path::Path) -> anyhow::Result<Track> {
    let track_models = track.models.load(directory)?;
    let lengths = make_track::track_model::ModelLengths::calculate(&track, &track_models);

    Ok(Track {
        track,
        track_models,
        lengths,
    })
}

fn update_model<'a>(
    args: &UpdateModelArgs,
    render_device: &'a renderer::Device,
    track: &'a Track,
    offsets: Option<&make_track::track_desc::Offsets>,
) -> anyhow::Result<Scene<'a>> {
    let model_desc = make_track::track_model::ModelDesc::new(
        &track.track,
        &track.track_models,
        &track.lengths,
        args.track_section,
        args.rotation,
    );
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
    make_track::track_model::build(
        &mut scene,
        &track.track_models,
        args.track_section,
        &model_desc,
        &offset_start,
        &offset_end,
    )?;
    let (scene, mesh_types) = scene.build();
    Ok(Scene { scene, mesh_types })
}

fn render(track: &Track, scene: &Scene, args: &mut RenderArgs) -> Images {
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

    let mut unindexed_image = framebuffer.to_image();
    let mut indexed_image = framebuffer.into_indexed_image(args.dither);
    unindexed_image.offset.y += -track.track.z_offset + 16;
    indexed_image.offset.y += -track.track.z_offset + 16;

    Images {
        unindexed: unindexed_image,
        indexed: indexed_image,
    }
}

fn report_error(tx: &Sender<AppMessage>, error: &anyhow::Error) {
    let errors = error.chain().map(|x| x.to_string()).collect();
    let _result = tx.send(AppMessage::Error(errors));
}

pub fn render_thread(render_rx: &Receiver<RenderMessage>, app_tx: &Sender<AppMessage>, track_image: &SharedTrackImage) {
    let render_device = match renderer::Device::try_new() {
        Ok(render_device) => render_device,
        Err(_) => {
            let _result = app_tx.send(AppMessage::Error(vec!["Could not create render device".to_string()]));
            return;
        }
    };

    let mut current_directory = None;
    let mut current_track = None;
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
                RenderMessage::LoadTrack(track_desc) => {
                    if let Some(directory) = current_directory.as_ref() {
                        match load_track(*track_desc, directory) {
                            Ok(track) => {
                                current_scene = None;
                                current_track = Some(track);
                            }
                            Err(error) => report_error(app_tx, &error),
                        }
                    }
                }
                RenderMessage::UpdateOffsets(offsets) => {
                    current_scene = None;
                    current_offsets = *offsets;
                }
                RenderMessage::UpdateModel(args) => {
                    if let Some(track) = &current_track {
                        match update_model(&args, &render_device, track, current_offsets.as_ref()) {
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
            && let Some(track) = &current_track
            && let Some(scene) = &current_scene
        {
            let images = render(track, scene, &mut args);

            if let Ok(mut track_image) = track_image.lock() {
                *track_image = Some(TrackImage {
                    images,
                    track_section: current_track_section,
                    rotation: args.rotation,
                });
            }

            args.egui_context.request_repaint();
            let _result = app_tx.send(AppMessage::NewFrame);
        }
    }
}
