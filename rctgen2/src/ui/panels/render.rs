use crate::ui::widgets;
use eframe::egui;

pub fn render_panel(track_desc: &mut make_track::track_desc::Desc, ui: &mut egui::Ui) -> bool {
    let mut changed = false;

    egui::Panel::right("Render settings panel").resizable(false).exact_size(250.0).show(ui, |ui| {
        egui::Grid::new("Render settings grid").min_col_width(0.0).show(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Samples")
            });
            if ui.add(widgets::DragValueSpin::new(&mut track_desc.samples, 1).range(1..=4)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Edge Distance")
            });
            let mut edge_distance_removed = false;
            if let Some(edge_distance) = track_desc.edge_distance.as_mut() {
                if ui.add(widgets::DragValueSpin::new(edge_distance, 0.01)).changed() {
                    changed = true;
                }
                if widgets::buttons::remove_button(ui) {
                    edge_distance_removed = true;
                    changed = true;
                }
            } else {
                if widgets::buttons::add_button(ui) {
                    track_desc.edge_distance = Some(100.0);
                    changed = true;
                }
            }
            if edge_distance_removed {
                track_desc.edge_distance = None;
            }
            ui.end_row();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Dither")
            });
            if ui.add(egui::Checkbox::without_text(&mut track_desc.dither)).changed() {
                changed = true;
            }
            ui.end_row();
        });
    });

    changed
}
