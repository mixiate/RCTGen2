use eframe::egui;

pub enum Choice {
    NoSave,
    Save,
    Cancel,
}

pub fn save_confirm_modal(ui: &mut egui::Ui) -> Option<Choice> {
    let mut choice = None;
    let modal = egui::containers::modal::Modal::new(egui::Id::new("Save Confirm Modal")).show(ui.ctx(), |ui| {
        ui.label("Save changes?");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                choice = Some(Choice::Save);
            }
            if ui.button("Don't Save").clicked() {
                choice = Some(Choice::NoSave);
            }
            if ui.button("Cancel").clicked() {
                ui.close();
            }
        });
    });
    if modal.should_close() {
        Some(Choice::Cancel)
    } else {
        choice
    }
}
