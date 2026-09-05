use crate::adjacent_track;
use crate::file_watcher;
use crate::render;
use crate::render::{RenderArgs, RenderMessage, SharedTrackImage, TrackImage, UpdateModelArgs};
use crate::settings;
use crate::sprites;
use crate::ui;
use crate::ui::panels;
use crate::ui::widgets;
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub enum TrackEditorMessage {
    NewFrame,
    ModelFileChanged,
    Error(Vec<String>),
}

#[derive(Clone, Copy, Default)]
pub struct Changes {
    pub directory: bool,
    pub model_settings: bool,
    pub load_models: bool,
    pub masks: bool,
    pub offsets: bool,
    pub update_model: bool,
    pub render: bool,
    pub redraw: bool,
}

pub struct TrackEditor {
    render_thread: Option<std::thread::JoinHandle<()>>,
    editor_rx: Receiver<TrackEditorMessage>,
    render_tx: Sender<RenderMessage>,
    track_image: SharedTrackImage,
    changes: Changes,
    side_panel_tab: Option<panels::SidePanelTab>,
    track_desc_path: std::path::PathBuf,
    track_desc: make_track::track_desc::Desc,
    track_index: usize,
    track_section: &'static make_track::track_sections::TrackSection,
    drawing_options: crate::drawing::Options,
    rotation: usize,
    current_track_image: Option<TrackImage>,
    back_buffer: egui::TextureHandle,
    back_buffer_image: renderer::image::Image,
    colour_button_textures: Vec<widgets::colour_picker::ButtonTextures>,
    colour_picker_1: widgets::colour_picker::ColourPicker,
    colour_picker_2: widgets::colour_picker::ColourPicker,
    colour_picker_3: widgets::colour_picker::ColourPicker,
    adjacent_track_sections: adjacent_track::AdjacentTrackSections,
    file_watcher: file_watcher::FileWatcher,
    model_file_changed_time: Option<std::time::Instant>,
}

impl TrackEditor {
    pub fn new(
        egui_context: &egui::Context,
        data_directory: &std::path::Path,
        track_desc_path: std::path::PathBuf,
        track_desc: make_track::track_desc::Desc,
        errors: &mut Vec<String>,
    ) -> Self {
        let (render_tx, render_rx) = std::sync::mpsc::channel();
        let (editor_tx, editor_rx) = std::sync::mpsc::channel();
        let track_image = Arc::new(Mutex::new(None));

        let changes = Changes {
            directory: true,
            model_settings: true,
            load_models: true,
            masks: true,
            offsets: true,
            update_model: true,
            render: true,
            redraw: true,
        };

        let render_thread = {
            let editor_tx = editor_tx.clone();
            let track_image = track_image.clone();
            let data_directory = data_directory.to_path_buf();
            std::thread::spawn(move || render::render_thread(&render_rx, &editor_tx, &track_image, &data_directory))
        };

        let adjacent_track_sections = data_directory.join("adjacent_track_sections").with_extension("json");
        let adjacent_track_sections = match adjacent_track::load_adjacent_track_sections(&adjacent_track_sections) {
            Ok(sections) => sections,
            Err(error) => {
                errors.extend(error.chain().map(|x| x.to_string()));
                Default::default()
            }
        };

        let back_buffer_size = 512;
        let back_buffer = egui::ColorImage::filled([back_buffer_size, back_buffer_size], egui::Color32::TRANSPARENT);
        let back_buffer = egui_context.load_texture("back buffer", back_buffer, egui::TextureOptions::default());
        let mut back_buffer_image = renderer::image::Image::new(back_buffer_size, back_buffer_size);
        back_buffer_image.offset = glam::IVec2::new(back_buffer_size as i32 / 2, back_buffer_size as i32 / 2);

        let file_watcher = file_watcher::FileWatcher::try_new(editor_tx, egui_context.clone()).unwrap();

        TrackEditor {
            render_thread: Some(render_thread),
            editor_rx,
            render_tx,
            track_image,
            changes,
            side_panel_tab: None,
            track_desc_path,
            track_desc,
            track_index: 0,
            track_section: &make_track::track_sections::FLAT,
            drawing_options: Default::default(),
            rotation: 0,
            current_track_image: None,
            back_buffer,
            back_buffer_image,
            colour_button_textures: widgets::colour_picker::create_colour_button_textures(egui_context),
            colour_picker_1: widgets::colour_picker::ColourPicker::new(),
            colour_picker_2: widgets::colour_picker::ColourPicker::new(),
            colour_picker_3: widgets::colour_picker::ColourPicker::new(),
            adjacent_track_sections,
            file_watcher,
            model_file_changed_time: None,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut settings::AppSettings,
        rct2_sprites: Option<&mut sprites::Sprites>,
        errors: &mut Vec<String>,
    ) {
        let mut fetch_frame = false;
        for message in self.editor_rx.try_iter() {
            match message {
                TrackEditorMessage::NewFrame => fetch_frame = true,
                TrackEditorMessage::ModelFileChanged => self.model_file_changed_time = Some(std::time::Instant::now()),
                TrackEditorMessage::Error(error) => errors.extend(error),
            }
        }

        if let Some(time) = self.model_file_changed_time {
            if let Some(time_left) = std::time::Duration::from_millis(250).checked_sub(time.elapsed()) {
                ui.ctx().request_repaint_after(time_left);
            } else {
                self.model_file_changed_time = None;
                self.changes.load_models = true;
            }
        }

        ui::menu_bars::menu_bar(
            ui,
            &mut self.current_track_image,
            &mut self.track_desc_path,
            &mut self.track_desc,
            &mut self.track_index,
            &mut self.track_section,
            settings,
            &mut self.changes,
            errors,
        );

        panels::side_panel_tabs(ui, &mut self.side_panel_tab);

        if let Some(tab) = self.side_panel_tab {
            panels::side_panel(
                ui,
                tab,
                &self.track_desc_path,
                &mut self.track_desc,
                self.track_index,
                self.track_section,
                self.rotation,
                &mut self.changes,
                errors,
            );
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(23, 35, 35));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            let frame = egui::Frame::popup(ui.style()).outer_margin(egui::Margin::same(10)).shadow(egui::Shadow::NONE);
            frame.show(ui, |ui| {
                if ui.checkbox(&mut self.drawing_options.indexed, "Indexed").changed() {
                    self.changes.redraw = true;
                }
            });
            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                    if self.colour_picker_1.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_1)
                    {
                        self.changes.redraw = true;
                    }
                    if self.colour_picker_2.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_2)
                    {
                        self.changes.redraw = true;
                    }
                    if self.colour_picker_3.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_3)
                    {
                        self.changes.redraw = true;
                    }
                });
            });
            frame.show(ui, |ui| {
                let supports_checkbox_enabled = if let Some(metal_supports) = &self.track_desc.metal_supports {
                    metal_supports.sections.contains_key(self.track_section.name)
                } else {
                    false
                };
                if ui
                    .add_enabled(
                        supports_checkbox_enabled && rct2_sprites.is_some(),
                        egui::Checkbox::new(&mut self.drawing_options.supports, "Supports"),
                    )
                    .changed()
                {
                    self.changes.redraw = true;
                }
            });
            frame.show(ui, |ui| {
                let original_track_checkbox_enabled = if let Some(track) = self.track_desc.tracks.get(self.track_index)
                {
                    track.original_sprites.contains_key(self.track_section.name)
                } else {
                    false
                };
                if ui
                    .add_enabled(
                        original_track_checkbox_enabled && rct2_sprites.is_some(),
                        egui::Checkbox::new(&mut self.drawing_options.original_track, "Original"),
                    )
                    .changed()
                {
                    self.changes.redraw = true;
                }
                let adjacent_track_checkbox_enabled = if let Some(track) = self.track_desc.tracks.get(self.track_index)
                {
                    !track.original_sprites.is_empty()
                } else {
                    false
                };
                if ui
                    .add_enabled(
                        adjacent_track_checkbox_enabled && rct2_sprites.is_some(),
                        egui::Checkbox::new(&mut self.drawing_options.adjacent_track, "Adjacent"),
                    )
                    .changed()
                {
                    self.changes.redraw = true;
                }
            });
            frame.show(ui, |ui| {
                if ui
                    .add_sized(
                        egui::Vec2::new(35.0, 35.0),
                        egui::Button::new(egui::RichText::new("↻").size(25.0)),
                    )
                    .clicked()
                {
                    self.rotation += 1;
                    if self.rotation == 4 {
                        self.rotation = 0;
                    }
                    self.changes.update_model = true;
                }
            });

            if self.changes.directory
                && let Some(directory) = self.track_desc_path.parent()
            {
                if let Err(error) = self.file_watcher.set_directory(directory) {
                    errors.push(error.to_string());
                }
                let _result = self.render_tx.send(RenderMessage::SetDirectory(directory.to_path_buf()));
            }
            if let Some(track) = self.track_desc.tracks.get(self.track_index) {
                if self.changes.model_settings {
                    let _result = self.render_tx.send(RenderMessage::UpdateModelSettings(track.model_settings));
                    self.changes.update_model = true;
                }
                if self.changes.load_models {
                    let _result = self.render_tx.send(RenderMessage::LoadModels(Box::new(track.models.clone())));
                    self.changes.update_model = true;
                }
                if self.changes.masks {
                    let _result = self.render_tx.send(RenderMessage::LoadMasks(track.masks.clone()));
                    self.changes.update_model = true;
                }
            }
            if self.changes.offsets {
                let _result = self.render_tx.send(RenderMessage::UpdateOffsets(Box::new(self.track_desc.offsets)));
                self.changes.update_model = true;
            }
            if self.changes.update_model {
                let _result = self.render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
                    track_section: self.track_section,
                    rotation: self.rotation,
                }));
                self.changes.render = true;
            }
            if self.changes.render {
                let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
                    egui_context: ui.ctx().clone(),
                    rotation: self.rotation,
                    samples: self.track_desc.samples.into(),
                    dither: self.track_desc.dither,
                    edge_distance: self.track_desc.edge_distance,
                    lights: self.track_desc.get_lights(),
                }));
            }

            if fetch_frame
                && let Ok(mut track_image) = self.track_image.lock()
                && track_image.is_some()
            {
                self.current_track_image = track_image.take();
                self.changes.redraw = true;
            }

            if self.changes.redraw
                && let Some(track) = self.track_desc.tracks.get(self.track_index)
                && let Some(track_image) = &self.current_track_image
            {
                let max_tile_height = track_image
                    .track_section
                    .tiles
                    .iter()
                    .max_by(|a, b| a[2].cmp(&b[2]))
                    .map(|x| i32::from(x[2]))
                    .unwrap_or(0);
                self.back_buffer_image.offset.y = (self.back_buffer_image.height() as i32 / 2) + (max_tile_height / 2);

                self.back_buffer_image.pixels_mut().fill(0);
                crate::drawing::draw(
                    track,
                    self.track_desc.metal_supports.as_ref(),
                    track_image,
                    &self.drawing_options,
                    &self.adjacent_track_sections,
                    rct2_sprites,
                    &mut self.back_buffer_image,
                );
                let image =
                    egui::ColorImage::from_rgba_unmultiplied(self.back_buffer.size(), self.back_buffer_image.pixels());
                self.back_buffer.set(image, egui::TextureOptions::default());
            }

            let texture_size = self.back_buffer.size_vec2();
            let image = egui::Image::from_texture((self.back_buffer.id(), texture_size));
            let image_pos = ui.max_rect().center() - (texture_size / egui::Vec2::new(2.0, 2.0));
            let image_rect = egui::Rect::from_min_size(image_pos, texture_size);
            ui.place(image_rect, image);
        });

        self.changes = Changes::default();
    }

    pub fn on_exit(&mut self) {
        let _result = self.render_tx.send(RenderMessage::Exit);
        self.render_thread.take().map(std::thread::JoinHandle::join);
    }
}
