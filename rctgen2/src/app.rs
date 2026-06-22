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
    track_desc_path: Option<std::path::PathBuf>,
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
            track_desc_path: None,
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
            self.track_desc_path = Some(file_path);
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

    fn queue_render(&self, egui_context: egui::Context) {
        if let Some(track_desc) = &self.track_desc {
            let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
                egui_context,
                rotation: self.rotation,
                samples: self.samples,
                dither: self.dither,
                indexed: self.indexed,
                lights: track_desc.lights.clone(),
            }));
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
                });
                if ui.add(egui::DragValue::new(&mut self.samples).prefix("Samples: ").range(1..=4)).changed() {
                    queue_render = true;
                }
                if ui.checkbox(&mut self.indexed, "Indexed").changed() {
                    queue_render = true;
                }
                if ui.checkbox(&mut self.dither, "Dithered").changed() {
                    queue_render = true;
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

        if update_model {
            self.update_model();
        }
        if queue_render {
            self.queue_render(ui.ctx().clone());
        }

        if fetch_frame
            && let Ok(mut render_texture) = self.render_texture.lock()
            && render_texture.is_some()
        {
            self.texture = render_texture.take();
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(34, 33, 39));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
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
