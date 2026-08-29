use crate::ui::widgets;
use eframe::egui;

pub fn render_panel(track_desc: &mut make_track::track_desc::Desc, ui: &mut egui::Ui) -> bool {
    let mut changed = false;

    egui::Panel::right("Render settings panel").resizable(false).exact_size(250.0).show(ui, |ui| {
        if ui.checkbox(&mut track_desc.dither, "Dither").changed() {
            changed = true;
        }

        ui.separator();

        let mut edge_distance_removed = false;
        if let Some(edge_distance) = track_desc.edge_distance.as_mut() {
            egui::Grid::new("Edge distance grid").show(ui, |ui| {
                if widgets::drag_value(ui, edge_distance, "Edge distance", None) {
                    changed = true;
                }
                if widgets::buttons::remove_button(ui) {
                    edge_distance_removed = true;
                    changed = true;
                }
                ui.end_row();
            });
        } else {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("Edge distance");
                if widgets::buttons::add_button(ui) {
                    track_desc.edge_distance = Some(100.0);
                    changed = true;
                }
            });
        }
        if edge_distance_removed {
            track_desc.edge_distance = None;
        }
    });

    changed
}
