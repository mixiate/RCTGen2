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
    current_track_section: &TrackSection,
    export_settings: &mut track_editor::ExportSettings,
    settings: &mut settings::AppSettings,
    changes: &mut track_editor::TrackChanges,
    errors: &mut Vec<String>,
) -> Option<track_editor::Action> {
    let mut action = None;
    egui::Panel::top("Tracks Menu Bar").show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.add(egui::Button::new("Open...").min_size(egui::Vec2::new(200.0, 0.0))).clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    action = Some(track_editor::Action::Open(file_path));
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
                            action = Some(track_editor::Action::Open(file_path));
                        }
                        ui.separator();
                        if ui.button("Clear Recent Files").clicked() {
                            settings.settings.recent_track_files.clear();
                        }
                    });
                });

                if ui.button("Save").clicked() {
                    action = Some(track_editor::Action::Save);
                }
                ui.separator();

                if ui.button("Import Lights...").clicked()
                    && let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file()
                {
                    match make_track::track_desc::Desc::load(&file_path) {
                        Ok(import_track_desc) => {
                            track.desc.lights = import_track_desc.lights;
                            *changes |= track_editor::TrackChanges::Lights;
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
                            *changes |= track_editor::TrackChanges::MetalSupports;
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
            ui.separator();

            egui::ComboBox::from_id_salt("Track dropdown")
                .width(180.0)
                .height(500.0)
                .selected_text(track_name(&track.desc.tracks[track.track_index]))
                .show_ui(ui, |ui| {
                    for (index, sub_track) in track.desc.tracks.iter().enumerate() {
                        if ui.selectable_label(index == track.track_index, track_name(sub_track)).clicked() {
                            action = Some(track_editor::Action::ChangeTrack(index));
                        }
                    }
                });
            ui.separator();

            egui::ComboBox::from_id_salt("Track section")
                .selected_text(current_track_section.name)
                .width(300.0)
                .height(500.0)
                .show_ui(ui, |ui| {
                    for track_section in make_track::track_sections::TRACK_SECTIONS {
                        if ui.selectable_label(track_section == current_track_section, track_section.name).clicked() {
                            action = Some(track_editor::Action::ChangeSection(track_section));
                        }
                    }
                });
            ui.separator();

            if ui.add_enabled(export_settings.export_enabled, egui::Button::new("Export")).clicked() {
                action = Some(track_editor::Action::Export);
            }
            ui.add_enabled(
                export_settings.export_enabled,
                egui::Checkbox::new(&mut export_settings.skip_empty_sprites, "Skip Empty"),
            );
            ui.add_enabled(
                export_settings.build_enabled,
                egui::Checkbox::new(&mut export_settings.build, "Build"),
            );
            ui.separator();
        });
    });
    action
}
