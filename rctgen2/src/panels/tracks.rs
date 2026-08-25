use crate::widgets;
use eframe::egui;
use relative_path::RelativePathBuf;

fn length_widgets(ui: &mut egui::Ui, value: &mut Option<f32>) -> bool {
    let mut changed = false;
    let mut removed = false;
    if let Some(value) = value {
        if ui.add(egui::DragValue::new(value).speed(0.01).range(0.1..=1.0)).changed() {
            changed = true;
        }
        if widgets::buttons::remove_button(ui) {
            removed = true;
            changed = true;
        }
    } else {
        if widgets::buttons::add_button(ui) {
            *value = Some(0.1);
            changed = true;
        }
    }
    if removed {
        *value = None;
    }
    changed
}

fn model_file_dialog(directory: &std::path::Path) -> anyhow::Result<Option<RelativePathBuf>> {
    use anyhow::Context as _;

    let result = rfd::FileDialog::new().add_filter("obj", &["obj"]).set_directory(directory).pick_file();
    if let Some(file_path) = result {
        let file_path = file_path.strip_prefix(directory).context("Model file must be in the track directory.")?;
        let file_path = RelativePathBuf::from_path(file_path)?;
        Ok(Some(file_path))
    } else {
        Ok(None)
    }
}

fn model_widgets(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut RelativePathBuf,
    directory: &std::path::Path,
    errors: &mut Vec<String>,
    changed: &mut bool,
) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(label);
    });
    if ui.add(egui::Button::new("📁")).clicked() {
        match model_file_dialog(directory) {
            Ok(Some(file_path)) => {
                *value = file_path;
                *changed = true;
            }
            Err(error) => {
                errors.push(error.to_string());
            }
            _ => {}
        }
    }
    ui.label(value.as_str());
}

fn optional_model_widgets(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<RelativePathBuf>,
    directory: &std::path::Path,
    errors: &mut Vec<String>,
    changed: &mut bool,
) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(label);
    });
    if ui.add(egui::Button::new("📁")).clicked() {
        match model_file_dialog(directory) {
            Ok(Some(file_path)) => {
                *value = Some(file_path);
                *changed = true;
            }
            Err(error) => {
                errors.push(error.to_string());
            }
            _ => {}
        }
    }
    let mut removed = false;
    if let Some(value) = value {
        ui.label(value.as_str());
        if widgets::buttons::remove_button(ui) {
            removed = true;
            *changed = true;
        }
    }
    if removed {
        *value = None;
    }
}

fn models_collapsible(
    ui: &mut egui::Ui,
    models: &mut make_track::track_desc::Models<RelativePathBuf>,
    dir: &std::path::Path,
    errors: &mut Vec<String>,
) -> bool {
    let mut changed = false;
    egui::Grid::new("Track models grid").min_col_width(0.0).show(ui, |ui| {
        model_widgets(ui, "Track", &mut models.track, dir, errors, &mut changed);
        ui.end_row();

        model_widgets(ui, "Mask", &mut models.mask, dir, errors, &mut changed);
        ui.end_row();

        optional_model_widgets(ui, "Tie", &mut models.tie, dir, errors, &mut changed);
        ui.end_row();

        optional_model_widgets(ui, "Track Alt", &mut models.track_alt, dir, errors, &mut changed);
        ui.end_row();

        optional_model_widgets(ui, "Track Tie", &mut models.track_tie, dir, errors, &mut changed);
        ui.end_row();

        optional_model_widgets(ui, "Support Base", &mut models.support_base, dir, errors, &mut changed);
        ui.end_row();

        optional_model_widgets(ui, "Support Flat", &mut models.support_flat, dir, errors, &mut changed);
        ui.end_row();

        optional_model_widgets(
            ui,
            "Support Bank 1/6",
            &mut models.support_bank_sixth,
            dir,
            errors,
            &mut changed,
        );
        ui.end_row();

        optional_model_widgets(
            ui,
            "Support Bank 1/3",
            &mut models.support_bank_third,
            dir,
            errors,
            &mut changed,
        );
        ui.end_row();

        optional_model_widgets(
            ui,
            "Support Bank 1/2",
            &mut models.support_bank_half,
            dir,
            errors,
            &mut changed,
        );
        ui.end_row();

        optional_model_widgets(
            ui,
            "Support Bank 2/3",
            &mut models.support_bank_two_thirds,
            dir,
            errors,
            &mut changed,
        );
        ui.end_row();

        optional_model_widgets(
            ui,
            "Support Bank 5/6",
            &mut models.support_bank_five_sixths,
            dir,
            errors,
            &mut changed,
        );
        ui.end_row();

        optional_model_widgets(ui, "Support Bank", &mut models.support_bank, dir, errors, &mut changed);
        ui.end_row();
    });
    changed
}

pub fn tracks_panel(
    tracks: &mut [make_track::track_desc::Track],
    directory: &std::path::Path,
    errors: &mut Vec<String>,
    ui: &mut egui::Ui,
) -> bool {
    let mut changed = false;
    egui::Panel::right("Tracks side panel").show(ui, |ui| {
        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .auto_shrink(false)
            .show(ui, |ui| {
                for (index, track) in tracks.iter_mut().enumerate() {
                    if index != 0 {
                        ui.separator();
                    }
                    egui::Grid::new(index).show(ui, |ui| {
                        {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label("Name");
                            });
                            let mut size = ui.spacing().interact_size;
                            size.x = 150.0;
                            if ui.add_sized(size, egui::TextEdit::singleline(&mut track.name)).changed() {
                                changed = true;
                            }
                        }
                        ui.end_row();

                        {
                            let mut removed = false;
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label("Suffix");
                            });
                            if let Some(suffix) = track.suffix.as_mut() {
                                if ui.add(egui::TextEdit::singleline(suffix)).changed() {
                                    changed = true;
                                }
                                if widgets::buttons::remove_button(ui) {
                                    removed = true;
                                    changed = true;
                                }
                            } else {
                                if widgets::buttons::add_button(ui) {
                                    track.suffix = Some(String::new());
                                }
                            }
                            if removed {
                                track.suffix = None;
                            }
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Length");
                        });
                        if length_widgets(ui, &mut track.length) {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Tie length");
                        });
                        if length_widgets(ui, &mut track.tie_length) {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Z offset");
                        });
                        if ui.add(egui::DragValue::new(&mut track.z_offset).speed(0.1)).changed() {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Support spacing");
                        });
                        if ui.add(egui::DragValue::new(&mut track.support_spacing).speed(0.01)).changed() {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Support pivot");
                        });
                        if ui.add(egui::DragValue::new(&mut track.support_pivot).speed(0.01)).changed() {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Bank angle");
                        });
                        if ui.add(egui::DragValue::new(&mut track.bank_angle).speed(0.1)).changed() {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Lift");
                        });
                        if ui.add(egui::Checkbox::without_text(&mut track.lift)).changed() {
                            changed = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Masks");
                        });
                        if ui.add(egui::TextEdit::singleline(&mut track.masks)).changed() {
                            changed = true;
                        }
                        ui.end_row();
                    });

                    egui::CollapsingHeader::new("Models").id_salt(index + 512).show(ui, |ui| {
                        if models_collapsible(ui, &mut track.models, directory, errors) {
                            changed = true;
                        }
                    });

                    egui::CollapsingHeader::new("Sections").id_salt(index + 1024).show(ui, |ui| {
                        use strum::IntoEnumIterator as _;
                        for group in make_track::track_desc::TrackGroup::iter() {
                            let mut enabled = track.sections.contains(&group);
                            let label: &'static str = group.into();
                            if ui.checkbox(&mut enabled, label).changed() {
                                if enabled {
                                    track.sections.insert(group);
                                } else {
                                    track.sections.shift_remove(&group);
                                }
                            }
                        }
                    });
                }
            });
    });
    changed
}
