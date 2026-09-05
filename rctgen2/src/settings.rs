use eframe::egui;
use std::collections::VecDeque;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct RecentFiles(VecDeque<std::path::PathBuf>);

impl RecentFiles {
    pub fn get(&self) -> &VecDeque<std::path::PathBuf> {
        &self.0
    }

    pub fn add(&mut self, file_path: &std::path::Path) {
        self.0.retain(|x| *x != file_path);
        self.0.push_front(file_path.to_path_buf());
        self.0.truncate(50);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g1_dat_path: Option<std::path::PathBuf>,
    #[serde(default, skip_serializing_if = "RecentFiles::is_empty")]
    pub recent_track_files: RecentFiles,
}

pub struct AppSettings {
    pub settings: Settings,
    pub config_dir: std::path::PathBuf,
    pub settings_file_path: std::path::PathBuf,
    pub window_open: bool,
}

impl AppSettings {
    pub fn new(config_dir: std::path::PathBuf) -> Self {
        let settings_file_path = config_dir.join("settings").with_extension("toml");
        let settings = if let Ok(string) = std::fs::read_to_string(&settings_file_path)
            && let Ok(settings) = toml::from_str(&string)
        {
            settings
        } else {
            Settings::default()
        };

        Self {
            settings,
            config_dir,
            settings_file_path,
            window_open: false,
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        use anyhow::Context as _;

        let _result = std::fs::create_dir_all(&self.config_dir);

        let toml = toml::to_string(&self.settings).unwrap();
        std::fs::write(&self.settings_file_path, toml)
            .with_context(|| format!("Could not save settings: {}", self.settings_file_path.display()))
    }

    pub fn window(&mut self, ui: &mut egui::Ui) -> bool {
        if !self.window_open {
            return false;
        }

        let mut changed = false;

        egui::Window::new("Settings")
            .open(&mut self.window_open)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .resizable(false)
            .fixed_size(egui::Vec2::new(650.0, 250.0))
            .show(ui.ctx(), |ui| {
                let mut frame = egui::Frame::new();
                frame.inner_margin = egui::Margin::same(24);

                egui::CentralPanel::default().frame(frame).show(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label("g1.dat path:");
                        if ui.add(egui::Button::new("📁")).clicked() {
                            let file_result = rfd::FileDialog::new().add_filter("dat", &["dat"]).pick_file();
                            if let Some(file_path) = file_result {
                                self.settings.g1_dat_path = Some(file_path);
                                changed = true;
                            }
                        }
                        if let Some(g1_dat_path) = &self.settings.g1_dat_path
                            && let Some(mut g1_dat_path) = g1_dat_path.to_str()
                        {
                            ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut g1_dat_path));
                        }
                    });
                });
            });

        changed
    }
}
