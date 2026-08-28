use crate::widgets;
use eframe::egui;
use make_track::track_desc::AdditionalModel;
use make_track::track_sections::TRACK_SECTIONS;
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

fn add_additional_model_button(
    ui: &mut egui::Ui,
    additional_models: &mut indexmap::IndexMap<String, AdditionalModel<RelativePathBuf>>,
    directory: &std::path::Path,
    errors: &mut Vec<String>,
    current_track_section: &make_track::track_sections::TrackSection,
) -> bool {
    let mut changed = false;
    let enabled = !additional_models.contains_key(current_track_section.name);
    if ui.add_enabled(enabled, egui::Button::new("Add current section")).clicked() {
        match model_file_dialog(directory) {
            Ok(Some(file_path)) => {
                let name = current_track_section.name.to_string();
                if let indexmap::map::Entry::Vacant(entry) = additional_models.entry(name) {
                    let model = AdditionalModel {
                        model: file_path,
                        mirror: false,
                    };

                    entry.insert_sorted_by(model, |key_a, _, key_b, _| {
                        let a_index = TRACK_SECTIONS.iter().position(|x| x.name == key_a);
                        let b_index = TRACK_SECTIONS.iter().position(|x| x.name == key_b);
                        if let Some(a_index) = a_index
                            && let Some(b_index) = b_index
                        {
                            a_index.cmp(&b_index)
                        } else {
                            std::cmp::Ordering::Less
                        }
                    });
                }
                changed = true;
            }
            Err(error) => {
                errors.push(error.to_string());
            }
            _ => {}
        }
    }
    changed
}

fn models_collapsible(
    ui: &mut egui::Ui,
    models: &mut make_track::track_desc::Models<RelativePathBuf>,
    dir: &std::path::Path,
    errors: &mut Vec<String>,
    current_track_section: &make_track::track_sections::TrackSection,
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

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Additional Models");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if add_additional_model_button(ui, &mut models.additional, dir, errors, current_track_section) {
                changed = true;
            }
        });
    });
    egui::Grid::new("Track additional models grid").min_col_width(0.0).show(ui, |ui| {
        let mut removed_index = None;
        for (index, (track_section, model)) in models.additional.iter_mut().enumerate() {
            model_widgets(ui, track_section, &mut model.model, dir, errors, &mut changed);
            if ui.checkbox(&mut model.mirror, "Mirror").changed() {
                changed = true;
            }
            if widgets::buttons::remove_button(ui) {
                removed_index = Some(index);
            }
            ui.end_row();
        }
        if let Some(index) = removed_index {
            models.additional.shift_remove_index(index);
            changed = true;
        }
    });

    changed
}

#[derive(Default)]
pub struct TracksPanelChanged {
    pub model_settings: bool,
    pub models: bool,
    pub masks: bool,
    pub redraw: bool,
}

pub fn tracks_panel(
    tracks: &mut [make_track::track_desc::Track],
    directory: &std::path::Path,
    errors: &mut Vec<String>,
    current_track_section: &make_track::track_sections::TrackSection,
    ui: &mut egui::Ui,
) -> TracksPanelChanged {
    let mut changed = TracksPanelChanged::default();
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
                            ui.add_sized(size, egui::TextEdit::singleline(&mut track.name));
                        }
                        ui.end_row();

                        {
                            let mut removed = false;
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label("Suffix");
                            });
                            if let Some(suffix) = track.suffix.as_mut() {
                                ui.add(egui::TextEdit::singleline(suffix));
                                if widgets::buttons::remove_button(ui) {
                                    removed = true;
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
                            ui.label("Z offset");
                        });
                        if ui.add(egui::DragValue::new(&mut track.z_offset).speed(0.1)).changed() {
                            changed.redraw = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Masks");
                        });
                        if ui.add(egui::TextEdit::singleline(&mut track.masks)).lost_focus() {
                            changed.masks = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Length");
                        });
                        if length_widgets(ui, &mut track.model_settings.length) {
                            changed.model_settings = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Tie length");
                        });
                        if length_widgets(ui, &mut track.model_settings.tie_length) {
                            changed.model_settings = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Support spacing");
                        });
                        if ui.add(egui::DragValue::new(&mut track.model_settings.support_spacing).speed(0.01)).changed()
                        {
                            changed.model_settings = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Support pivot");
                        });
                        if ui.add(egui::DragValue::new(&mut track.model_settings.support_pivot).speed(0.01)).changed() {
                            changed.model_settings = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Bank angle");
                        });
                        if ui.add(egui::DragValue::new(&mut track.model_settings.bank_angle).speed(0.1)).changed() {
                            changed.model_settings = true;
                        }
                        ui.end_row();

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Lift");
                        });
                        if ui.add(egui::Checkbox::without_text(&mut track.model_settings.lift)).changed() {
                            changed.model_settings = true;
                        }
                        ui.end_row();
                    });

                    egui::CollapsingHeader::new("Models").id_salt(index + 512).show(ui, |ui| {
                        if models_collapsible(ui, &mut track.models, directory, errors, current_track_section) {
                            changed.models = true;
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
