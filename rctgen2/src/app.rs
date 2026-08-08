use crate::adjacent_track;
use crate::modals;
use crate::panels;
use crate::render::{LoadTrackArgs, RenderArgs, RenderMessage, SharedTrackImage, TrackImage, UpdateModelArgs};
use crate::settings;
use crate::sprites;
use crate::widgets;
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};

pub enum AppMessage {
    NewFrame,
    Error(Vec<String>),
}

pub struct RctGen2App {
    app_rx: Receiver<AppMessage>,
    render_tx: Sender<RenderMessage>,
    track_image: SharedTrackImage,
    errors: Vec<String>,
    settings: settings::AppSettings,
    adjacent_track_sections: adjacent_track::AdjacentTrackSections,
    rct2_sprites: Option<sprites::Sprites>,
    side_panel_tab: Option<panels::SidePanelTab>,
    sprites_track_selection_modal: modals::TrackSectionSelectionModal,
    track_desc_path: Option<std::path::PathBuf>,
    track_desc: Option<make_track::track_desc::Desc>,
    track_section: &'static make_track::track_sections::TrackSection,
    samples: usize,
    drawing_options: crate::drawing::Options,
    rotation: usize,
    current_track_image: Option<TrackImage>,
    back_buffer: egui::TextureHandle,
    back_buffer_image: renderer::image::Image,
    colour_button_textures: Vec<widgets::colour_picker::ButtonTextures>,
    colour_picker_1: widgets::colour_picker::ColourPicker,
    colour_picker_2: widgets::colour_picker::ColourPicker,
}

impl RctGen2App {
    pub fn new(
        egui_context: &egui::Context,
        app_rx: Receiver<AppMessage>,
        render_tx: Sender<RenderMessage>,
        track_image: SharedTrackImage,
        data_directory: &std::path::Path,
        config_dir: std::path::PathBuf,
    ) -> Self {
        let mut errors = Vec::new();

        let settings = settings::AppSettings::new(config_dir);

        let adjacent_track_sections = data_directory.join("adjacent_track_sections").with_extension("json");
        let adjacent_track_sections = match adjacent_track::load_adjacent_track_sections(&adjacent_track_sections) {
            Ok(sections) => sections,
            Err(error) => {
                errors.extend(error.chain().map(|x| x.to_string()));
                Default::default()
            }
        };

        let rct2_sprites = if let Some(g1_dat_path) = &settings.settings.g1_dat_path {
            match sprites::Sprites::try_new(g1_dat_path) {
                Ok(sprites) => Some(sprites),
                Err(error) => {
                    errors.extend(error.chain().map(|x| x.to_string()));
                    None
                }
            }
        } else {
            None
        };

        let back_buffer_size = 512;
        let back_buffer = egui::ColorImage::filled([back_buffer_size, back_buffer_size], egui::Color32::TRANSPARENT);
        let back_buffer = egui_context.load_texture("back buffer", back_buffer, egui::TextureOptions::default());
        let mut back_buffer_image = renderer::image::Image::new(back_buffer_size, back_buffer_size);
        back_buffer_image.offset = glam::IVec2::new(back_buffer_size as i32 / 2, back_buffer_size as i32 / 2);

        Self {
            app_rx,
            render_tx,
            track_image,
            errors,
            settings,
            adjacent_track_sections,
            rct2_sprites,
            side_panel_tab: None,
            sprites_track_selection_modal: modals::TrackSectionSelectionModal::new(),
            track_desc_path: None,
            track_desc: None,
            track_section: &make_track::track_sections::FLAT,
            samples: 4,
            drawing_options: Default::default(),
            rotation: 0,
            current_track_image: None,
            back_buffer,
            back_buffer_image,
            colour_button_textures: widgets::colour_picker::create_colour_button_textures(egui_context),
            colour_picker_1: widgets::colour_picker::ColourPicker::new(),
            colour_picker_2: widgets::colour_picker::ColourPicker::new(),
        }
    }

    fn load_track(&mut self, egui_context: egui::Context) -> anyhow::Result<()> {
        use anyhow::Context as _;

        let file_result = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file();
        if let Some(file_path) = file_result {
            let directory = file_path
                .parent()
                .with_context(|| format!("Could not get parent directory of {}", file_path.display()))?
                .to_path_buf();
            let track_desc = make_track::track_desc::Desc::load(&file_path)?;

            let _result = self.render_tx.send(RenderMessage::LoadTrack(Box::new(LoadTrackArgs {
                track_desc: track_desc.clone(),
                directory,
            })));
            self.update_model();
            self.current_track_image = None;
            self.track_desc_path = Some(file_path);
            self.track_desc = Some(track_desc);
            self.queue_render(egui_context);
        }
        Ok(())
    }

    fn update_offsets(&self) {
        if let Some(track_desc) = &self.track_desc {
            let _result = self.render_tx.send(RenderMessage::UpdateOffsets(Box::new(track_desc.offsets)));
        }
    }

    fn update_model(&self) {
        let _result = self.render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
            track_section: self.track_section,
            rotation: self.rotation,
        }));
    }

    fn queue_render(&self, egui_context: egui::Context) {
        if let Some(track_desc) = &self.track_desc {
            let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
                egui_context,
                rotation: self.rotation,
                samples: self.samples,
                dither: track_desc.dither,
                edge_distance: track_desc.edge_distance,
                lights: track_desc.get_lights(),
            }));
        }
    }

    fn draw_side_panel(&mut self, ui: &mut egui::Ui) {
        if let Some(track_desc) = self.track_desc.as_mut() {
            match self.side_panel_tab {
                Some(panels::SidePanelTab::Lights) => {
                    let lights_changed = panels::lights::lights_panel(&mut track_desc.lights, ui);
                    if lights_changed {
                        self.queue_render(ui.ctx().clone());
                    }
                }
                Some(panels::SidePanelTab::Offsets) => {
                    let offsets_changed = panels::offsets::offsets_panel(&mut track_desc.offsets, ui);
                    if offsets_changed {
                        self.update_offsets();
                        self.update_model();
                        self.queue_render(ui.ctx().clone());
                    }
                }
                Some(panels::SidePanelTab::Render) => {
                    let changed = panels::render::render_panel(track_desc, ui);
                    if changed {
                        self.queue_render(ui.ctx().clone());
                    }
                }
                Some(panels::SidePanelTab::Sprites) => {
                    panels::sprites::sprites_panel(
                        &mut track_desc.original_sprites,
                        &mut self.sprites_track_selection_modal,
                        ui,
                    );
                }
                None => {}
            }
        }
    }
}

impl eframe::App for RctGen2App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut fetch_frame = false;
        for message in self.app_rx.try_iter() {
            match message {
                AppMessage::NewFrame => fetch_frame = true,
                AppMessage::Error(errors) => self.errors.extend(errors),
            }
        }

        let mut update_model = false;
        let mut queue_render = false;
        let mut redraw = false;
        let previous_track_section = self.track_section;

        egui::Panel::top("Top Menu").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.add(egui::Button::new("Open...").min_size(egui::Vec2::new(200.0, 0.0))).clicked()
                        && let Err(error) = self.load_track(ui.ctx().clone())
                    {
                        self.errors.extend(error.chain().map(|x| x.to_string()));
                    }

                    if let Some(path) = &self.track_desc_path
                        && let Some(track_desc) = &self.track_desc
                    {
                        if ui.button("Save").clicked()
                            && let Err(error) = track_desc.save(path)
                        {
                            self.errors.extend(error.chain().map(|x| x.to_string()));
                        }
                    } else {
                        ui.add_enabled(false, egui::Button::new("Save"));
                    }

                    ui.separator();
                    if ui.button("Settings").clicked() {
                        self.settings.window_open = true;
                    }

                    ui.separator();

                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                if ui.add(egui::DragValue::new(&mut self.samples).prefix("Samples: ").range(1..=4)).changed() {
                    queue_render = true;
                }
                if ui.checkbox(&mut self.drawing_options.indexed, "Indexed").changed() {
                    redraw = true;
                }

                let show_original_piece_enabled = if let Some(track_desc) = &self.track_desc {
                    track_desc.original_sprites.contains_key(self.track_section.name)
                } else {
                    true
                };
                if ui
                    .add_enabled(
                        show_original_piece_enabled,
                        egui::Checkbox::new(&mut self.drawing_options.original_track, "Original"),
                    )
                    .clicked()
                {
                    redraw = true;
                }
                if ui.checkbox(&mut self.drawing_options.adjacent_track, "Adjacent").clicked() {
                    redraw = true;
                }

                egui::ComboBox::from_id_salt("Track section")
                    .selected_text(self.track_section.name)
                    .width(300.0)
                    .height(500.0)
                    .show_ui(ui, |ui| {
                        for track_section in make_track::track_sections::TRACK_SECTIONS {
                            ui.selectable_value(&mut self.track_section, track_section, track_section.name);
                        }
                    });
                if self.track_section != previous_track_section {
                    update_model = true;
                    queue_render = true;
                }

                if ui.add(egui::Button::new("↻")).clicked() {
                    self.rotation += 1;
                    if self.rotation == 4 {
                        self.rotation = 0;
                    }
                    update_model = true;
                    queue_render = true;
                }
            });
        });

        panels::side_panel_tabs(ui, &mut self.side_panel_tab);

        self.draw_side_panel(ui);

        if update_model {
            self.update_model();
        }
        if queue_render {
            self.queue_render(ui.ctx().clone());
        }

        if fetch_frame
            && let Ok(mut track_image) = self.track_image.lock()
            && track_image.is_some()
        {
            self.current_track_image = track_image.take();
            redraw = true;
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(34, 33, 39));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                if self.colour_picker_1.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_1) {
                    redraw = true;
                }
                if self.colour_picker_2.button(ui, &self.colour_button_textures, &mut self.drawing_options.colour_2) {
                    redraw = true;
                }
            });

            if redraw
                && let Some(track_desc) = &self.track_desc
                && let Some(track_image) = &self.current_track_image
            {
                self.back_buffer_image.pixels_mut().fill(0);
                crate::drawing::draw(
                    track_desc,
                    track_image,
                    &self.drawing_options,
                    &self.adjacent_track_sections,
                    self.rct2_sprites.as_mut(),
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

        if self.settings.window(ui) {
            if let Some(g1_dat_path) = &self.settings.settings.g1_dat_path {
                match sprites::Sprites::try_new(g1_dat_path) {
                    Ok(sprites) => self.rct2_sprites = Some(sprites),
                    Err(error) => self.errors.extend(error.chain().map(|x| x.to_string())),
                }
            }
            if let Err(error) = self.settings.save() {
                self.errors.extend(error.chain().map(|x| x.to_string()));
            }
        }

        if !self.errors.is_empty() {
            let modal = egui::containers::modal::Modal::new(egui::Id::new("Error")).show(ui.ctx(), |ui| {
                ui.set_width(600.0);

                ui.vertical_centered(|ui| {
                    ui.heading("Error");
                    ui.separator();

                    for error in &self.errors {
                        ui.add(egui::Label::new(error));
                    }
                });
            });

            if modal.should_close() {
                self.errors.clear();
            }
        }
    }

    fn on_exit(&mut self) {
        let _result = self.render_tx.send(RenderMessage::Exit);
    }
}
