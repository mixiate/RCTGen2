use crate::app;
use crate::track_editor;
use eframe::egui;

pub fn ui(ui: &mut egui::Ui, data_directory: &std::path::Path, errors: &mut Vec<String>) -> Option<app::State> {
    if ui.button("Tracks").clicked() {
        Some(app::State::TrackEditor(Box::new(track_editor::TrackEditor::new(
            ui.ctx(),
            data_directory,
            errors,
        ))))
    } else {
        None
    }
}
