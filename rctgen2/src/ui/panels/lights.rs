use crate::ui::widgets;
use eframe::egui;

fn inverted_checkbox(ui: &mut egui::Ui, value: &mut bool) -> bool {
    let mut inverse_value = !(*value);
    if ui.checkbox(&mut inverse_value, "Enabled").clicked() {
        *value = !inverse_value;
        true
    } else {
        false
    }
}

pub fn lights_panel(lights: &mut Vec<make_track::track_desc::Light>, ui: &mut egui::Ui) -> bool {
    let mut queue_render = false;
    egui::Panel::right("Lights").resizable(false).show(ui, |ui| {
        let mut deleted_index = None;
        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
        let visibility = egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible;
        egui::ScrollArea::vertical().scroll_bar_visibility(visibility).show(ui, |ui| {
            for (i, light) in lights.iter_mut().enumerate() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if widgets::buttons::remove_button(ui) {
                        deleted_index = Some(i);
                        queue_render = true;
                    }

                    if inverted_checkbox(ui, &mut light.disabled) {
                        queue_render = true;
                    }
                });
                egui::Grid::new(i).min_col_width(7.0).show(ui, |ui| {
                    ui.label("X");
                    if ui.add(widgets::DragValueSpin::new(&mut light.direction[0], 0.01)).changed() {
                        queue_render = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label("Diffuse");
                    });
                    if ui
                        .add(widgets::DragValueSpin::new(&mut light.diffuse_strength, 0.01).range(0.0..=10.0))
                        .changed()
                    {
                        queue_render = true;
                    }
                    ui.end_row();

                    ui.label("Y");
                    if ui.add(widgets::DragValueSpin::new(&mut light.direction[1], 0.01)).changed() {
                        queue_render = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label("Specular");
                    });
                    if ui
                        .add(widgets::DragValueSpin::new(&mut light.specular_strength, 0.01).range(0.0..=10.0))
                        .changed()
                    {
                        queue_render = true;
                    }
                    ui.end_row();

                    ui.label("Z");
                    if ui.add(widgets::DragValueSpin::new(&mut light.direction[2], 0.01)).changed() {
                        queue_render = true;
                    }
                    ui.add_visible(false, egui::Label::new(""));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.checkbox(&mut light.shadow, "Shadow").clicked() {
                            queue_render = true;
                        }
                    });
                    ui.end_row();
                });

                ui.separator();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if widgets::buttons::add_button(ui) {
                    lights.push(make_track::track_desc::Light {
                        direction: [1.0, 0.5, 1.0],
                        diffuse_strength: 1.0,
                        specular_strength: 1.0,
                        shadow: true,
                        disabled: false,
                    });
                    queue_render = true;
                }
            });
        });
        if let Some(i) = deleted_index {
            lights.remove(i);
            queue_render = true;
        }
    });
    queue_render
}
