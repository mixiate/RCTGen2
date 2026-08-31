use crate::app;
use crate::render::TrackImage;
use crate::settings;
use eframe::egui;
use make_track::track_sections::TrackSection;

fn load_track(
    file_path: std::path::PathBuf,
    current_track_image: &mut Option<TrackImage>,
    current_path: &mut Option<std::path::PathBuf>,
    current_track_desc: &mut Option<make_track::track_desc::Desc>,
    changes: &mut app::Changes,
) -> anyhow::Result<()> {
    let track_desc = make_track::track_desc::Desc::load(&file_path)?;

    changes.directory = true;
    changes.model_settings = true;
    changes.load_models = true;
    changes.masks = true;
    changes.offsets = true;

    *current_track_image = None;
    *current_path = Some(file_path);
    *current_track_desc = Some(track_desc);

    Ok(())
}

fn track_name(track: &make_track::track_desc::Track) -> String {
    if let Some(suffix) = &track.suffix {
        format!("{} {suffix}", track.name)
    } else {
        track.name.clone()
    }
}

#[expect(clippy::too_many_arguments)]
pub fn menu_bar(
    ui: &mut egui::Ui,
    current_track_image: &mut Option<TrackImage>,
    track_desc_path: &mut Option<std::path::PathBuf>,
    track_desc: &mut Option<make_track::track_desc::Desc>,
    current_track_index: &mut usize,
    current_track_section: &mut &TrackSection,
    settings: &mut settings::AppSettings,
    changes: &mut app::Changes,
    errors: &mut Vec<String>,
) {
    egui::Panel::top("Tracks Menu Bar").show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.add(egui::Button::new("Open...").min_size(egui::Vec2::new(200.0, 0.0))).clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    settings.settings.add_recent_file(&file_path);
                    if let Err(error) = load_track(file_path, current_track_image, track_desc_path, track_desc, changes)
                    {
                        errors.extend(error.chain().map(|x| x.to_string()));
                    }
                }

                ui.scope(|ui| {
                    if settings.settings.recent_files().is_empty() {
                        ui.disable();
                    }
                    egui::containers::menu::SubMenuButton::new("Open Recent").ui(ui, |ui| {
                        let mut clicked_index = None;
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (index, file_path) in settings.settings.recent_files().iter().enumerate() {
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
                            ui.separator();
                            if ui.button("Clear recent files").clicked() {
                                settings.settings.clear_recent_files();
                            }
                        });
                        if let Some(index) = clicked_index {
                            let file_path = settings.settings.recent_files()[index].clone();
                            settings.settings.add_recent_file(&file_path);
                            if let Err(error) =
                                load_track(file_path, current_track_image, track_desc_path, track_desc, changes)
                            {
                                errors.extend(error.chain().map(|x| x.to_string()));
                            }
                        }
                    });
                });

                if let Some(path) = &track_desc_path
                    && let Some(track_desc) = &track_desc
                {
                    if ui.button("Save").clicked()
                        && let Err(error) = track_desc.save(path)
                    {
                        errors.extend(error.chain().map(|x| x.to_string()));
                    }
                } else {
                    ui.add_enabled(false, egui::Button::new("Save"));
                }
                ui.separator();

                if ui.add_enabled(track_desc.is_some(), egui::Button::new("Import Lights...")).clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    match make_track::track_desc::Desc::load(&file_path) {
                        Ok(import_track_desc) => {
                            if let Some(track_desc) = track_desc {
                                track_desc.lights = import_track_desc.lights;
                                changes.render = true;
                            }
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                    }
                }
                if ui.add_enabled(track_desc.is_some(), egui::Button::new("Import Metal Supports...")).clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    match make_track::track_desc::Desc::load(&file_path) {
                        Ok(import_track_desc) => {
                            if let Some(track_desc) = track_desc {
                                track_desc.metal_supports = import_track_desc.metal_supports;
                                changes.redraw = true;
                            }
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
                let previous_track_index = *current_track_index;
                let mut combo_box = egui::ComboBox::from_id_salt("Track dropdown").width(180.0).height(500.0);
                if let Some(track_desc) = track_desc
                    && let Some(track) = track_desc.tracks.get(*current_track_index)
                {
                    combo_box = combo_box.selected_text(track_name(track));
                }
                combo_box.show_ui(ui, |ui| {
                    if let Some(track_desc) = &track_desc {
                        for (index, track) in track_desc.tracks.iter().enumerate() {
                            ui.selectable_value(current_track_index, index, track_name(track));
                        }
                    }
                });
                if *current_track_index != previous_track_index {
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
