use crate::render::{LoadTrackArgs, RenderArgs, RenderMessage, SharedTexture, UpdateModelArgs};
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};

pub enum AppMessage {
    NewFrame,
    Error(Vec<String>),
}

pub struct RctGen2App {
    app_rx: Receiver<AppMessage>,
    render_tx: Sender<RenderMessage>,
    render_texture: SharedTexture,
    errors: Vec<String>,
    track_desc: Option<make_track::track_desc::Desc>,
    track_section: &'static make_track::track_sections::TrackSection,
    samples: usize,
    indexed: bool,
    dither: bool,
    rotation: usize,
    texture: Option<egui::TextureHandle>,
}

impl RctGen2App {
    pub fn new(app_rx: Receiver<AppMessage>, render_tx: Sender<RenderMessage>, render_texture: SharedTexture) -> Self {
        Self {
            app_rx,
            render_tx,
            render_texture,
            errors: Vec::new(),
            track_desc: None,
            track_section: &make_track::track_sections::FLAT,
            samples: 4,
            indexed: true,
            dither: true,
            rotation: 0,
            texture: None,
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
            self.dither = track_desc.dither;
            self.track_desc = Some(track_desc);
            self.queue_render(egui_context);
        }
        Ok(())
    }

    fn update_model(&self) {
        let _result = self.render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
            track_section: self.track_section,
            rotation: self.rotation,
        }));
    }

    fn queue_render(&mut self, egui_context: egui::Context) {
        let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
            egui_context,
            rotation: self.rotation,
            samples: self.samples,
            dither: self.dither,
            indexed: self.indexed,
        }));
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

        let previous_track_section = self.track_section;

        egui::Panel::top("Settings").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.add(egui::Button::new("Open")).clicked()
                    && let Err(error) = self.load_track(ui.ctx().clone())
                {
                    self.errors.extend(error.chain().map(|x| x.to_string()));
                }
                if ui.add(egui::DragValue::new(&mut self.samples).prefix("Samples: ").range(1..=4)).changed() {
                    self.queue_render(ui.ctx().clone());
                }
                if ui.checkbox(&mut self.indexed, "Indexed").changed() {
                    self.queue_render(ui.ctx().clone());
                }
                if ui.checkbox(&mut self.dither, "Dithered").changed() {
                    self.queue_render(ui.ctx().clone());
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

                if ui.add(egui::Button::new("↻")).clicked() {
                    self.rotation += 1;
                    if self.rotation == 4 {
                        self.rotation = 0;
                    }
                    self.update_model();
                    self.queue_render(ui.ctx().clone());
                }
            });
        });

        if self.track_section != previous_track_section {
            self.update_model();
            self.queue_render(ui.ctx().clone());
        }

        if fetch_frame
            && let Ok(mut render_texture) = self.render_texture.lock()
            && render_texture.is_some()
        {
            self.texture = render_texture.take();
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(34, 33, 39));
        egui::CentralPanel::default().frame(frame).show_inside(ui, |ui| {
            if let Some(texture) = &self.texture {
                ui.centered_and_justified(|ui| ui.image((texture.id(), texture.size_vec2())));
            }
        });

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
