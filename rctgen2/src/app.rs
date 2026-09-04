use crate::settings;
use crate::sprites;
use crate::track_editor;
use eframe::egui;

pub struct RctGen2App {
    track_editor: track_editor::TrackEditor,
    errors: Vec<String>,
    settings: settings::AppSettings,
    rct2_sprites: Option<sprites::Sprites>,
}

impl RctGen2App {
    pub fn new(egui_context: &egui::Context, data_directory: &std::path::Path, config_dir: std::path::PathBuf) -> Self {
        let mut errors = Vec::new();

        let track_editor = track_editor::TrackEditor::new(egui_context, data_directory, &mut errors);

        let settings = settings::AppSettings::new(config_dir);

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

        Self {
            track_editor,
            errors,
            settings,
            rct2_sprites,
        }
    }
}

impl eframe::App for RctGen2App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.track_editor.ui(ui, &mut self.settings, self.rct2_sprites.as_mut(), &mut self.errors);

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
        self.track_editor.on_exit();
        let _result = self.settings.save();
    }
}
