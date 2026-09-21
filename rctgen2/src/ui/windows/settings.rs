use crate::settings::Settings;
use eframe::egui;

fn path_buf_text(ui: &mut egui::Ui, path: Option<&std::path::PathBuf>, size: egui::Vec2) {
    if let Some(path) = path
        && let Some(mut path) = path.to_str()
    {
        ui.add_sized(size, egui::TextEdit::singleline(&mut path));
    }
}

pub fn settings_window(ui: &mut egui::Ui, open: &mut bool, settings: &mut Settings) -> bool {
    let mut changed = false;
    egui::Window::new("Settings")
        .open(open)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .resizable(false)
        .fixed_size(egui::Vec2::new(650.0, 250.0))
        .show(ui.ctx(), |ui| {
            let frame = egui::Frame::new().inner_margin(egui::Margin::same(24));
            egui::CentralPanel::default().frame(frame).show(ui, |ui| {
                egui::Grid::new("Settings Grid").min_col_width(0.0).show(ui, |ui| {
                    let path_text_size = egui::Vec2::new(400.0, ui.available_size().y);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Track Export Directory");
                    });
                    if ui.add(egui::Button::new("📁")).clicked() {
                        let file_result = rfd::FileDialog::new().pick_folder();
                        if let Some(file_path) = file_result {
                            settings.track_export_directory = Some(file_path);
                            changed = true;
                        }
                    }
                    path_buf_text(ui, settings.track_export_directory.as_ref(), path_text_size);
                    ui.end_row();

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Track Build Input Path");
                    });
                    if ui.add(egui::Button::new("📁")).clicked() {
                        let file_result = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file();
                        if let Some(file_path) = file_result {
                            settings.track_build_input_path = Some(file_path);
                            changed = true;
                        }
                    }
                    path_buf_text(ui, settings.track_build_input_path.as_ref(), path_text_size);
                    ui.end_row();

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Track Build Output Path");
                    });
                    if ui.add(egui::Button::new("📁")).clicked() {
                        let file_result = rfd::FileDialog::new().add_filter("dat", &["dat"]).pick_file();
                        if let Some(file_path) = file_result {
                            settings.track_build_output_path = Some(file_path);
                            changed = true;
                        }
                    }
                    path_buf_text(ui, settings.track_build_output_path.as_ref(), path_text_size);
                    ui.end_row();

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("g1.dat Path");
                    });
                    if ui.add(egui::Button::new("📁")).clicked() {
                        let file_result = rfd::FileDialog::new().add_filter("dat", &["dat"]).pick_file();
                        if let Some(file_path) = file_result {
                            settings.g1_dat_path = Some(file_path);
                            changed = true;
                        }
                    }
                    path_buf_text(ui, settings.g1_dat_path.as_ref(), path_text_size);
                    ui.end_row();
                });
            });
        });

    changed
}
