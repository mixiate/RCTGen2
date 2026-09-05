use crate::settings;
use eframe::egui;

pub fn recent_files(ui: &mut egui::Ui, recent_files: &mut settings::RecentFiles) -> Option<usize> {
    let mut clicked_index = None;
    for (index, file_path) in recent_files.get().iter().enumerate() {
        if let Some(file_name) = file_path.file_name()
            && let Some(file_name) = file_name.to_str()
        {
            let response = ui.button(file_name);
            if response.clicked() {
                clicked_index = Some(index);
            }
            if let Some(file_path) = file_path.to_str() {
                response.on_hover_text(file_path);
            }
        }
    }
    clicked_index
}
