use eframe::egui;

pub fn remove_button(ui: &mut egui::Ui) -> bool {
    let mut clicked = false;
    ui.scope(|ui| {
        ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
        clicked = ui.add(egui::Button::new("✖").fill(egui::Color32::LIGHT_RED)).clicked();
    });
    clicked
}

pub fn add_button(ui: &mut egui::Ui) -> bool {
    let mut clicked = false;
    ui.scope(|ui| {
        ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
        clicked = ui.add(egui::Button::new("➕").fill(egui::Color32::LIGHT_GREEN)).clicked();
    });
    clicked
}
