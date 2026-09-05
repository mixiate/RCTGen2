use crate::app;
use crate::settings;
use crate::track_editor;
use eframe::egui;

pub fn ui(
    ui: &mut egui::Ui,
    data_directory: &std::path::Path,
    settings: &mut settings::AppSettings,
    errors: &mut Vec<String>,
) -> Option<app::State> {
    if ui.button("Load Track...").clicked() {
        match track_editor::Track::open() {
            Ok(Some(track)) => {
                settings.settings.recent_track_files.add(&track.file_path);
                Some(app::State::TrackEditor(Box::new(track_editor::TrackEditor::new(
                    ui.ctx(),
                    data_directory,
                    track,
                    errors,
                ))))
            }
            Err(error) => {
                errors.extend(error.chain().map(|x| x.to_string()));
                None
            }
            _ => None,
        }
    } else {
        None
    }
}
