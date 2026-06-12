use crate::app::AppMessage;
use eframe::egui;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct LoadTrackArgs {
    pub track_desc: make_track::track_desc::Desc,
    pub directory: std::path::PathBuf,
}

pub struct UpdateModelArgs {
    pub track_section: &'static make_track::track_sections::TrackSection,
    pub rotation: usize,
}

pub struct RenderArgs {
    pub egui_context: egui::Context,
    pub rotation: usize,
    pub samples: usize,
    pub dither: bool,
    pub indexed: bool,
}

pub enum RenderMessage {
    LoadTrack(Box<LoadTrackArgs>),
    UpdateModel(UpdateModelArgs),
    Render(RenderArgs),
    Exit,
}

pub type SharedTexture = Arc<Mutex<Option<egui::TextureHandle>>>;

struct Track {
    track_desc: make_track::track_desc::Desc,
    track_models: make_track::track_desc::Models<renderer::model::Model>,
    lengths: make_track::track_model::ModelLengths,
}

struct Scene<'a> {
    scene: renderer::Scene<'a>,
    mesh_types: Vec<renderer::MeshType>,
}

fn load_track(args: LoadTrackArgs) -> anyhow::Result<Track> {
    use anyhow::Context as _;

    let track = args.track_desc.tracks.first().with_context(|| "No track found in track description")?;
    let track_models = track.models.load(&args.directory)?;
    let lengths = make_track::track_model::ModelLengths::calculate(track, &track_models);

    Ok(Track {
        track_desc: args.track_desc,
        track_models,
        lengths,
    })
}

fn update_model<'a>(
    args: &UpdateModelArgs,
    render_device: &'a renderer::Device,
    track: &'a Track,
) -> anyhow::Result<Scene<'a>> {
    use anyhow::Context as _;

    let model_desc = make_track::track_model::ModelDesc::new(
        track.track_desc.tracks.first().with_context(|| "No track found in track description")?,
        &track.track_models,
        &track.lengths,
        args.track_section,
        args.rotation,
    );
    let (offset_start, offset_end) = if let Some(offsets) = &track.track_desc.offsets {
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

fn render(track: &Track, scene: &Scene, args: &RenderArgs, render_texture: &SharedTexture) {
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
    let lights = track
        .track_desc
        .lights
        .iter()
        .map(|light| renderer::Light {
            diffuse_strength: light.diffuse_strength,
            specular_strength: light.specular_strength,
            direction: view_rotation_inverse.transform_vector3(light.direction.into()).normalize(),
            shadow: light.shadow,
        })
        .collect::<Vec<renderer::Light>>();
    let framebuffer = renderer::render_scene(
        &scene.scene,
        &scene.mesh_types,
        &camera,
        &lights,
        args.samples,
        args.samples,
        track.track_desc.edge_distance.unwrap_or(0.088388346),
    );
    let image = if args.indexed {
        let image = framebuffer.into_indexed_image(args.dither);
        let pixels: Vec<_> = image
            .as_raw()
            .iter()
            .flat_map(|x| {
                if *x == 0 {
                    [0; 4]
                } else {
                    let colour = renderer::palette::PALETTE[usize::from(*x)];
                    [colour[0], colour[1], colour[2], 255]
                }
            })
            .collect();
        renderer::image::Image::from_raw(usize::from(image.width()), usize::from(image.height()), pixels)
    } else {
        framebuffer.to_image()
    };

    let egui_image = egui::ColorImage::from_rgba_unmultiplied([image.width(), image.height()], image.as_raw());
    let texture = args.egui_context.load_texture("render", egui_image, egui::TextureOptions::default());

    if let Ok(mut render_texture) = render_texture.lock() {
        *render_texture = Some(texture);
    }

    args.egui_context.request_repaint();
}

fn report_error(tx: &Sender<AppMessage>, error: &anyhow::Error) {
    let errors = error.chain().map(|x| x.to_string()).collect();
    let _result = tx.send(AppMessage::Error(errors));
}

pub fn render_thread(render_rx: &Receiver<RenderMessage>, app_tx: &Sender<AppMessage>, render_texture: &SharedTexture) {
    let render_device = match renderer::Device::try_new() {
        Ok(render_device) => render_device,
        Err(_) => {
            let _result = app_tx.send(AppMessage::Error(vec!["Could not create render device".to_string()]));
            return;
        }
    };

    let mut current_track = None;
    let mut current_scene = None;

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
                RenderMessage::LoadTrack(args) => match load_track(*args) {
                    Ok(track) => {
                        current_scene = None;
                        current_track = Some(track);
                    }
                    Err(error) => report_error(app_tx, &error),
                },
                RenderMessage::UpdateModel(args) => {
                    if let Some(track) = &current_track {
                        match update_model(&args, &render_device, track) {
                            Ok(scene) => current_scene = Some(scene),
                            Err(error) => report_error(app_tx, &error),
                        }
                    }
                }
                RenderMessage::Render(_) => {}
                RenderMessage::Exit => break 'main_loop,
            }
        }

        if let Some(RenderMessage::Render(args)) = render_message
            && let Some(track) = &current_track
            && let Some(scene) = &current_scene
        {
            render(track, scene, &args, render_texture);
            let _result = app_tx.send(AppMessage::NewFrame);
        }
    }
}
