use crate::settings;
use crate::track_editor;
use crate::ui::widgets;
use eframe::egui;
use make_track::track_sections::TrackSection;

fn track_name(track: &make_track::track_desc::Track) -> String {
    if let Some(suffix) = &track.suffix {
        format!("{} {suffix}", track.name)
    } else {
        track.name.clone()
    }
}

pub fn menu_bar(
    ui: &mut egui::Ui,
    track: &mut track_editor::Track,
    current_track_section: &mut &TrackSection,
    settings: &mut settings::AppSettings,
    changes: &mut track_editor::Changes,
    errors: &mut Vec<String>,
) {
    egui::Panel::top("Tracks Menu Bar").show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.add(egui::Button::new("Open...").min_size(egui::Vec2::new(200.0, 0.0))).clicked() {
                    match track_editor::Track::open() {
                        Ok(Some(new_track)) => {
                            *track = new_track;
                            changes.load_track();
                            settings.settings.recent_track_files.add(&track.file_path);
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                        _ => {}
                    }
                }

                ui.scope(|ui| {
                    if settings.settings.recent_track_files.is_empty() {
                        ui.disable();
                    }
                    egui::containers::menu::SubMenuButton::new("Open Recent").ui(ui, |ui| {
                        let response = egui::ScrollArea::vertical().show(ui, |ui| {
                            widgets::recent_files(ui, &mut settings.settings.recent_track_files)
                        });
                        if let Some(index) = response.inner {
                            let file_path = settings.settings.recent_track_files.get()[index].clone();
                            match track_editor::Track::load(file_path) {
                                Ok(new_track) => {
                                    *track = new_track;
                                    changes.load_track();
                                    settings.settings.recent_track_files.add(&track.file_path);
                                }
                                Err(error) => {
                                    settings.settings.recent_track_files.remove(index);
                                    errors.extend(error.chain().map(|x| x.to_string()));
                                }
                            }
                        }
                        ui.separator();
                        if ui.button("Clear Recent Files").clicked() {
                            settings.settings.recent_track_files.clear();
                        }
                    });
                });

                if ui.button("Save").clicked()
                    && let Err(error) = track.desc.save(&track.file_path)
                {
                    errors.extend(error.chain().map(|x| x.to_string()));
                }
                ui.separator();

                if ui.button("Import Lights...").clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    match make_track::track_desc::Desc::load(&file_path) {
                        Ok(import_track_desc) => {
                            track.desc.lights = import_track_desc.lights;
                            changes.render = true;
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                    }
                }
                if ui.button("Import Metal Supports...").clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    match make_track::track_desc::Desc::load(&file_path) {
                        Ok(import_track_desc) => {
                            track.desc.metal_supports = import_track_desc.metal_supports;
                            changes.redraw = true;
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                    }
                }
                ui.separator();

                if ui.button("Settings").clicked() {
                    settings.window_open = true;
                }
                ui.separator();

                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.add_space(100.0);
            ui.separator();

            {
                let previous_track_index = track.track_index;
                let mut combo_box = egui::ComboBox::from_id_salt("Track dropdown").width(180.0).height(500.0);
                if let Some(track) = track.desc.tracks.get(track.track_index) {
                    combo_box = combo_box.selected_text(track_name(track));
                }
                combo_box.show_ui(ui, |ui| {
                    for (index, sub_track) in track.desc.tracks.iter().enumerate() {
                        ui.selectable_value(&mut track.track_index, index, track_name(sub_track));
                    }
                });
                if track.track_index != previous_track_index {
                    changes.model_settings = true;
                    changes.load_models = true;
                    changes.masks = true;
                }
            }
            ui.separator();

            let previous_track_section = *current_track_section;
            egui::ComboBox::from_id_salt("Track section")
                .selected_text(current_track_section.name)
                .width(300.0)
                .height(500.0)
                .show_ui(ui, |ui| {
                    for track_section in make_track::track_sections::TRACK_SECTIONS {
                        ui.selectable_value(current_track_section, track_section, track_section.name);
                    }
                });
            if *current_track_section != previous_track_section {
                changes.update_model = true;
            }
            ui.separator();
        });
    });
}
