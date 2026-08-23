use crate::widgets;
use eframe::egui;

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

pub fn tracks_panel(tracks: &mut [make_track::track_desc::Track], ui: &mut egui::Ui) -> bool {
    let mut changed = false;
    egui::Panel::right("Tracks side panel").resizable(false).show(ui, |ui| {
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

                    egui::CollapsingHeader::new("Sections").id_salt(index).show(ui, |ui| {
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
