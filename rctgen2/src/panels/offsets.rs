use crate::widgets;
use eframe::egui;

fn offsets_widget(ui: &mut egui::Ui, name: &str, offsets: &mut [[f32; 2]]) -> bool {
    let mut changed = false;
    ui.label(name);
    ui.columns_const(|[col_0, col_1]| {
        for offset in offsets.iter_mut() {
            if widgets::drag_value(col_0, &mut offset[0], "X", None) {
                changed = true;
            }
            if widgets::drag_value(col_1, &mut offset[1], "Y", None) {
                changed = true;
            }
        }
    });
    changed
}

pub fn offsets_panel(offsets: &mut Option<make_track::track_desc::Offsets>, ui: &mut egui::Ui) -> bool {
    let mut removed_offsets = false;
    let mut update_offsets = false;

    egui::Panel::right("Offsets").resizable(false).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            if offsets.is_some() {
                if ui.button("Remove offsets").clicked() {
                    *offsets = None;
                    removed_offsets = true;
                    update_offsets = true;
                }
                ui.separator();
            }
        });
        if let Some(offsets) = offsets.as_mut() {
            let mut remove_gentle_banked_right = false;

            ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
            let visibility = egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible;
            egui::ScrollArea::vertical().scroll_bar_visibility(visibility).show(ui, |ui| {
                if offsets_widget(ui, "Flat", &mut offsets.flat) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Gentle", &mut offsets.gentle) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Steep", &mut offsets.steep) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Flat Banked", &mut offsets.flat_banked) {
                    update_offsets = true;
                }
                ui.separator();
                if let Some(gentle_banked_right) = offsets.gentle_banked_right.as_mut() {
                    if offsets_widget(ui, "Gentle Banked Left", &mut offsets.gentle_banked) {
                        update_offsets = true;
                    }
                    ui.separator();
                    if offsets_widget(ui, "Gentle Banked Right", gentle_banked_right) {
                        update_offsets = true;
                    }
                    ui.vertical_centered(|ui| {
                        if ui.button("Remove Gentle Banked Right").clicked() {
                            remove_gentle_banked_right = true;
                        }
                    });
                } else {
                    if offsets_widget(ui, "Gentle Banked", &mut offsets.gentle_banked) {
                        update_offsets = true;
                    }
                    ui.separator();
                    ui.vertical_centered(|ui| {
                        if ui.button("Add Gentle Banked Right").clicked() {
                            offsets.gentle_banked_right = Some(Default::default());
                        }
                    });
                }
                ui.separator();
                if offsets_widget(ui, "Inverted", &mut offsets.inverted) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Diagonal", &mut offsets.diagonal) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Diagonal Gentle", &mut offsets.diagonal_gentle) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Diagonal Steep", &mut offsets.diagonal_steep) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Diagonal Banked", &mut offsets.diagonal_banked) {
                    update_offsets = true;
                }
                ui.separator();
                if offsets_widget(ui, "Vertical", &mut offsets.vertical) {
                    update_offsets = true;
                }
            });
            if remove_gentle_banked_right {
                offsets.gentle_banked_right = None;
            }
        } else if !removed_offsets {
            ui.vertical_centered(|ui| {
                if ui.button("Add offsets").clicked() {
                    *offsets = Some(make_track::track_desc::Offsets::default());
                }
            });
        }
    });

    update_offsets
}
