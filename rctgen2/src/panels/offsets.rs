use crate::widgets;
use eframe::egui;

fn offsets_drag_values(ui: &mut egui::Ui, offsets: &mut [[f32; 2]]) -> bool {
    let mut changed = false;
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
                ui.label("Flat");
                if offsets_drag_values(ui, &mut offsets.flat) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Gentle");
                if offsets_drag_values(ui, &mut offsets.gentle) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Steep");
                if offsets_drag_values(ui, &mut offsets.steep) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Flat Banked");
                if offsets_drag_values(ui, &mut offsets.flat_banked) {
                    update_offsets = true;
                }
                ui.separator();
                if let Some(gentle_banked_right) = offsets.gentle_banked_right.as_mut() {
                    ui.label("Gentle Banked Left");
                    if offsets_drag_values(ui, &mut offsets.gentle_banked) {
                        update_offsets = true;
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Gentle Banked Right");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if widgets::buttons::remove_button(ui) {
                                remove_gentle_banked_right = true;
                            }
                        });
                    });
                    if offsets_drag_values(ui, gentle_banked_right) {
                        update_offsets = true;
                    }
                } else {
                    ui.label("Gentle Banked");
                    if offsets_drag_values(ui, &mut offsets.gentle_banked) {
                        update_offsets = true;
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Gentle Banked Right");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if widgets::buttons::add_button(ui) {
                                offsets.gentle_banked_right = Some(Default::default());
                            }
                        });
                    });
                }
                ui.separator();
                ui.label("Inverted");
                if offsets_drag_values(ui, &mut offsets.inverted) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal");
                if offsets_drag_values(ui, &mut offsets.diagonal) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal Gentle");
                if offsets_drag_values(ui, &mut offsets.diagonal_gentle) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal Steep");
                if offsets_drag_values(ui, &mut offsets.diagonal_steep) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal Banked");
                if offsets_drag_values(ui, &mut offsets.diagonal_banked) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Vertical");
                if offsets_drag_values(ui, &mut offsets.vertical) {
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
