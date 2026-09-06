use crate::ui::widgets;
use eframe::egui;

fn offsets_drag_values(ui: &mut egui::Ui, id_str: &str, offsets: &mut [[f32; 2]]) -> bool {
    let mut changed = false;
    egui::Grid::new(id_str).min_col_width(0.0).show(ui, |ui| {
        for offset in offsets.iter_mut() {
            ui.label("X");
            if ui.add(widgets::DragValueSpin::new(&mut offset[0], 0.01)).changed() {
                changed = true;
            }
            ui.label("Y");
            if ui.add(widgets::DragValueSpin::new(&mut offset[1], 0.01)).changed() {
                changed = true;
            }
            ui.end_row();
        }
    });
    changed
}

pub fn offsets_panel(offsets: &mut Option<make_track::track_desc::Offsets>, ui: &mut egui::Ui) -> bool {
    let mut update_offsets = false;

    egui::Panel::right("Offsets").resizable(false).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("Offsets");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.allocate_space(egui::Vec2::new(ui.style().spacing.scroll.floating_width, 0.0));
                if offsets.is_some() {
                    if widgets::buttons::remove_button(ui) {
                        *offsets = None;
                        update_offsets = true;
                    }
                } else if widgets::buttons::add_button(ui) {
                    *offsets = Some(make_track::track_desc::Offsets::default());
                }
            });
        });
        ui.add(egui::Separator::default().spacing(0.0));

        if let Some(offsets) = offsets.as_mut() {
            let mut remove_gentle_banked_right = false;

            ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
            let visibility = egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible;
            egui::ScrollArea::vertical().scroll_bar_visibility(visibility).show(ui, |ui| {
                ui.label("Flat");
                if offsets_drag_values(ui, "Flat offset", &mut offsets.flat) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Gentle");
                if offsets_drag_values(ui, "Gentle offset", &mut offsets.gentle) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Steep");
                if offsets_drag_values(ui, "Steep offset", &mut offsets.steep) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Flat Banked");
                if offsets_drag_values(ui, "Flat Banked offset", &mut offsets.flat_banked) {
                    update_offsets = true;
                }
                ui.separator();
                if let Some(gentle_banked_right) = offsets.gentle_banked_right.as_mut() {
                    ui.label("Gentle Banked Left");
                    if offsets_drag_values(ui, "Gentle Banked Left offset", &mut offsets.gentle_banked) {
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
                    if offsets_drag_values(ui, "Gentle Banked Right offset", gentle_banked_right) {
                        update_offsets = true;
                    }
                } else {
                    ui.label("Gentle Banked");
                    if offsets_drag_values(ui, "Gentle Banked offset", &mut offsets.gentle_banked) {
                        update_offsets = true;
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Gentle Banked Right");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if widgets::buttons::add_button(ui) {
                                offsets.gentle_banked_right = Some(Default::default());
                                update_offsets = true;
                            }
                        });
                    });
                }
                ui.separator();
                ui.label("Inverted");
                if offsets_drag_values(ui, "Inverted offset", &mut offsets.inverted) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal");
                if offsets_drag_values(ui, "Diagonal offset", &mut offsets.diagonal) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal Gentle");
                if offsets_drag_values(ui, "Diagonal Gentle offset", &mut offsets.diagonal_gentle) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal Steep");
                if offsets_drag_values(ui, "Diagonal Steep offset", &mut offsets.diagonal_steep) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Diagonal Banked");
                if offsets_drag_values(ui, "Diagonal Banked offset", &mut offsets.diagonal_banked) {
                    update_offsets = true;
                }
                ui.separator();
                ui.label("Vertical");
                if offsets_drag_values(ui, "Vertical offset", &mut offsets.vertical) {
                    update_offsets = true;
                }
            });
            if remove_gentle_banked_right {
                offsets.gentle_banked_right = None;
                update_offsets = true;
            }
        }
    });

    update_offsets
}
