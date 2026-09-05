use crate::app;
use crate::settings;
use crate::track_editor;
use crate::ui::widgets;
use eframe::egui;

pub fn ui(
    ui: &mut egui::Ui,
    data_directory: &std::path::Path,
    settings: &mut settings::AppSettings,
    errors: &mut Vec<String>,
) -> Option<app::State> {
    let mut new_state = None;
    egui::Area::new(egui::Id::new("Start screen track area"))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .movable(false)
        .show(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("New Track...").clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).save_file()
                {
                    match track_editor::Track::try_new(file_path) {
                        Ok(track) => {
                            settings.settings.recent_track_files.add(&track.file_path);
                            if let Err(error) = track.desc.save(&track.file_path) {
                                errors.extend(error.chain().map(|x| x.to_string()));
                            }
                            new_state = Some(app::State::TrackEditor(Box::new(track_editor::TrackEditor::new(
                                ui.ctx(),
                                data_directory,
                                track,
                                errors,
                            ))));
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                    }
                }

                if ui.button("Load Track...").clicked() {
                    match track_editor::Track::open() {
                        Ok(Some(track)) => {
                            settings.settings.recent_track_files.add(&track.file_path);
                            new_state = Some(app::State::TrackEditor(Box::new(track_editor::TrackEditor::new(
                                ui.ctx(),
                                data_directory,
                                track,
                                errors,
                            ))));
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                        _ => {}
                    }
                }
            });

            ui.separator();
            ui.vertical_centered_justified(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(2.0, 0.0);
                ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
                ui.style_mut().visuals.widgets.open.bg_stroke = egui::Stroke::NONE;
                ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;

                let response = egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                    widgets::recent_files(ui, &mut settings.settings.recent_track_files)
                });
                if let Some(index) = response.inner {
                    let file_path = settings.settings.recent_track_files.get()[index].clone();
                    match track_editor::Track::load(file_path) {
                        Ok(track) => {
                            settings.settings.recent_track_files.add(&track.file_path);
                            new_state = Some(app::State::TrackEditor(Box::new(track_editor::TrackEditor::new(
                                ui.ctx(),
                                data_directory,
                                track,
                                errors,
                            ))));
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                    }
                }
            });
            ui.separator();

            ui.vertical_centered_justified(|ui| {
                if ui.button("Settings").clicked() {
                    settings.window_open = true;
                }
            });

            ui.allocate_space(egui::Vec2::new(250.0, 1.0));
        });
    new_state
}
