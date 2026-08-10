use crate::widgets;
use eframe::egui;

pub struct Response {
    pub changed: bool,
    pub removed: bool,
}

pub fn collapsible_with_remove(ui: &mut egui::Ui, label: &str, body: impl FnOnce(&mut egui::Ui) -> bool) -> Response {
    let mut changed = false;
    let mut removed = false;

    let id = ui.make_persistent_id(label);
    let mut toggle = false;
    let mut header =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false).show_header(ui, |ui| {
            let scope = ui.scope_builder(egui::UiBuilder::new().sense(egui::Sense::click()), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    let label = egui::Label::new(label).selectable(false).sense(egui::Sense::click());
                    if ui.add(label).clicked() {
                        toggle = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if widgets::buttons::remove_button(ui) {
                            removed = true;
                        }
                    });
                });
            });
            if scope.response.clicked() {
                toggle = true;
            }
        });

    if toggle {
        header.set_open(!header.is_open());
    }

    header.body(|ui| {
        if body(ui) {
            changed = true;
        }
    });
    Response { changed, removed }
}
