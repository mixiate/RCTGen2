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
    if ui.button("Load Track...").clicked()
        && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
    {
        settings.settings.recent_track_files.add(&file_path);
        match make_track::track_desc::Desc::load(&file_path) {
            Ok(track_desc) => Some(app::State::TrackEditor(Box::new(track_editor::TrackEditor::new(
                ui.ctx(),
                data_directory,
                file_path,
                track_desc,
                errors,
            )))),
            Err(error) => {
                errors.extend(error.chain().map(|x| x.to_string()));
                None
            }
        }
    } else {
        None
    }
}
