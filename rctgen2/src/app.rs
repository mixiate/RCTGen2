use crate::settings;
use crate::start_screen;
use crate::track_editor;
use eframe::egui;

pub enum State {
    Start(start_screen::StartScreen),
    TrackEditor(Box<track_editor::TrackEditor>),
}

pub struct RctGen2App {
    data_directory: std::path::PathBuf,
    errors: Vec<String>,
    settings: settings::AppSettings,
    rct2_sprites: Option<rct::csg::Archive>,
    state: State,
}

impl RctGen2App {
    pub fn new(data_directory: std::path::PathBuf, config_dir: std::path::PathBuf) -> Self {
        let mut errors = Vec::new();

        let settings = settings::AppSettings::new(config_dir);

        let rct2_sprites = if let Some(g1_dat_path) = &settings.settings.g1_dat_path {
            match rct::csg::Archive::load(g1_dat_path) {
                Ok(archive) => Some(archive),
                Err(error) => {
                    errors.push(error.to_string());
                    None
                }
            }
        } else {
            None
        };

        Self {
            data_directory,
            errors,
            settings,
            rct2_sprites,
            state: State::Start(start_screen::StartScreen::new()),
        }
    }
}

impl eframe::App for RctGen2App {
    fn logic(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        if let State::TrackEditor(track_editor) = &mut self.state {
            track_editor.logic(
                context,
                &self.data_directory,
                &mut self.settings.settings,
                self.rct2_sprites.as_ref(),
                &mut self.errors,
            );
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let new_state = match &mut self.state {
            State::Start(start_screen) => {
                start_screen.ui(ui, &self.data_directory, &mut self.settings, &mut self.errors)
            }
            State::TrackEditor(track_editor) => {
                track_editor.ui(ui, &mut self.settings, self.rct2_sprites.is_some(), &mut self.errors);
                None
            }
        };

        if self.settings.window(ui) {
            if let Some(g1_dat_path) = &self.settings.settings.g1_dat_path {
                match rct::csg::Archive::load(g1_dat_path) {
                    Ok(archive) => self.rct2_sprites = Some(archive),
                    Err(error) => self.errors.push(error.to_string()),
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

        if let Some(new_state) = new_state {
            self.state = new_state;
        }
    }

    fn on_exit(&mut self) {
        if let State::TrackEditor(track_editor) = &mut self.state {
            track_editor.on_exit();
        }
        let _result = self.settings.save();
    }
}
