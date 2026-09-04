use crate::adjacent_track;
use crate::app::AppMessage;
use crate::file_watcher;
use crate::render::{RenderArgs, RenderMessage, SharedTrackImage, TrackImage, UpdateModelArgs};
use crate::settings;
use crate::sprites;
use crate::ui;
use crate::ui::panels;
use crate::ui::widgets;
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};

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
    track_image: SharedTrackImage,
    current_track_image: Option<TrackImage>,
    side_panel_tab: Option<panels::SidePanelTab>,
    track_desc_path: Option<std::path::PathBuf>,
    track_desc: Option<make_track::track_desc::Desc>,
    track_index: usize,
    track_section: &'static make_track::track_sections::TrackSection,
    drawing_options: crate::drawing::Options,
    rotation: usize,
    back_buffer: egui::TextureHandle,
    back_buffer_image: renderer::image::Image,
    colour_button_textures: Vec<widgets::colour_picker::ButtonTextures>,
    colour_picker_1: widgets::colour_picker::ColourPicker,
    colour_picker_2: widgets::colour_picker::ColourPicker,
    colour_picker_3: widgets::colour_picker::ColourPicker,
    adjacent_track_sections: adjacent_track::AdjacentTrackSections,
    model_file_changed_time: Option<std::time::Instant>,
}

impl TrackEditor {
    pub fn new(
        egui_context: &egui::Context,
        track_image: SharedTrackImage,
        data_directory: &std::path::Path,
        errors: &mut Vec<String>,
    ) -> Self {
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

        TrackEditor {
            track_image,
            current_track_image: None,
            side_panel_tab: None,
            track_desc_path: None,
            track_desc: None,
            track_index: 0,
            track_section: &make_track::track_sections::FLAT,
            drawing_options: Default::default(),
            rotation: 0,
            back_buffer,
            back_buffer_image,
            colour_button_textures: widgets::colour_picker::create_colour_button_textures(egui_context),
            colour_picker_1: widgets::colour_picker::ColourPicker::new(),
            colour_picker_2: widgets::colour_picker::ColourPicker::new(),
            colour_picker_3: widgets::colour_picker::ColourPicker::new(),
            adjacent_track_sections,
            model_file_changed_time: None,
        }
    }

    #[expect(clippy::too_many_arguments)]
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        app_rx: &Receiver<AppMessage>,
        render_tx: &Sender<RenderMessage>,
        settings: &mut settings::AppSettings,
        rct2_sprites: Option<&mut sprites::Sprites>,
        file_watcher: &mut file_watcher::FileWatcher,
        errors: &mut Vec<String>,
    ) {
        let mut fetch_frame = false;
        for message in app_rx.try_iter() {
            match message {
                AppMessage::NewFrame => fetch_frame = true,
                AppMessage::ModelFileChanged => self.model_file_changed_time = Some(std::time::Instant::now()),
                AppMessage::Error(error) => errors.extend(error),
            }
        }

        let mut changes = Changes::default();

        if let Some(time) = self.model_file_changed_time {
            if let Some(time_left) = std::time::Duration::from_millis(250).checked_sub(time.elapsed()) {
                ui.ctx().request_repaint_after(time_left);
            } else {
                self.model_file_changed_time = None;
                changes.load_models = true;
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
            &mut changes,
            errors,
        );

        panels::side_panel_tabs(ui, &mut self.side_panel_tab);

        if let Some(track_desc) = self.track_desc.as_mut()
            && let Some(tab) = self.side_panel_tab
        {
            panels::side_panel(
                ui,
                tab,
                &mut self.track_desc_path,
                track_desc,
                self.track_index,
                self.track_section,
                self.rotation,
                &mut changes,
                errors,
            );
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(23, 35, 35));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            let frame = egui::Frame::popup(ui.style()).outer_margin(egui::Margin::same(10)).shadow(egui::Shadow::NONE);
            frame.show(ui, |ui| {
                if ui.checkbox(&mut self.drawing_options.indexed, "Indexed").changed() {
                    changes.redraw = true;
                }
            });
            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                    if self.colour_picker_1.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_1)
                    {
                        changes.redraw = true;
                    }
                    if self.colour_picker_2.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_2)
                    {
                        changes.redraw = true;
                    }
                    if self.colour_picker_3.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_3)
                    {
                        changes.redraw = true;
                    }
                });
            });
            frame.show(ui, |ui| {
                let supports_checkbox_enabled = if let Some(track_desc) = &self.track_desc
                    && let Some(metal_supports) = &track_desc.metal_supports
                {
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
                    changes.redraw = true;
                }
            });
            frame.show(ui, |ui| {
                let original_track_checkbox_enabled = if let Some(track_desc) = &self.track_desc
                    && let Some(track) = track_desc.tracks.get(self.track_index)
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
                    changes.redraw = true;
                }
                let adjacent_track_checkbox_enabled = if let Some(track_desc) = &self.track_desc
                    && let Some(track) = track_desc.tracks.get(self.track_index)
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
                    changes.redraw = true;
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
                    changes.update_model = true;
                }
            });

            if let Some(track_desc) = &self.track_desc
                && let Some(track) = track_desc.tracks.get(self.track_index)
            {
                if changes.directory
                    && let Some(track_desc_path) = &self.track_desc_path
                    && let Some(directory) = track_desc_path.parent()
                {
                    if let Err(error) = file_watcher.set_directory(directory) {
                        errors.push(error.to_string());
                    }
                    let _result = render_tx.send(RenderMessage::SetDirectory(directory.to_path_buf()));
                }
                if changes.model_settings {
                    let _result = render_tx.send(RenderMessage::UpdateModelSettings(track.model_settings));
                    changes.update_model = true;
                }
                if changes.load_models {
                    let _result = render_tx.send(RenderMessage::LoadModels(Box::new(track.models.clone())));
                    changes.update_model = true;
                }
                if changes.masks {
                    let _result = render_tx.send(RenderMessage::LoadMasks(track.masks.clone()));
                    changes.update_model = true;
                }
                if changes.offsets {
                    let _result = render_tx.send(RenderMessage::UpdateOffsets(Box::new(track_desc.offsets)));
                    changes.update_model = true;
                }
                if changes.update_model {
                    let _result = render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
                        track_section: self.track_section,
                        rotation: self.rotation,
                    }));
                    changes.render = true;
                }
                if changes.render {
                    let _result = render_tx.send(RenderMessage::Render(RenderArgs {
                        egui_context: ui.ctx().clone(),
                        rotation: self.rotation,
                        samples: track_desc.samples.into(),
                        dither: track_desc.dither,
                        edge_distance: track_desc.edge_distance,
                        lights: track_desc.get_lights(),
                    }));
                }
            }

            if fetch_frame
                && let Ok(mut track_image) = self.track_image.lock()
                && track_image.is_some()
            {
                self.current_track_image = track_image.take();
                changes.redraw = true;
            }

            if changes.redraw
                && let Some(track_desc) = &self.track_desc
                && let Some(track) = track_desc.tracks.get(self.track_index)
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
                    track_desc.metal_supports.as_ref(),
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
    }
}
