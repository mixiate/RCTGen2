use eframe::egui;

pub struct TrackSectionSelectionModal {
    modal_id: egui::Id,
    selection_id: egui::Id,
    open: bool,
    selected_track_section: &'static make_track::track_sections::TrackSection,
}

impl TrackSectionSelectionModal {
    pub fn new() -> Self {
        TrackSectionSelectionModal {
            modal_id: egui::Id::new("track section selection modal"),
            selection_id: egui::Id::new("track section selection list"),
            open: false,
            selected_track_section: &make_track::track_sections::FLAT,
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Option<&'static make_track::track_sections::TrackSection> {
        if !self.open {
            return None;
        }

        let mut result = None;
        let mut frame = egui::Frame::popup(ui.style());
        frame.inner_margin = egui::Margin::symmetric(10, 10);

        let modal = egui::containers::modal::Modal::new(self.modal_id).frame(frame).show(ui.ctx(), |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2::new(10.0, 10.0);

            egui::Grid::new("Track selection modal grid").show(ui, |ui| {
                egui::ComboBox::from_id_salt(self.selection_id)
                    .selected_text(self.selected_track_section.name)
                    .width(300.0)
                    .height(500.0)
                    .show_ui(ui, |ui| {
                        for track_section in make_track::track_sections::TRACK_SECTIONS {
                            ui.selectable_value(&mut self.selected_track_section, track_section, track_section.name);
                        }
                    });
                ui.end_row();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        self.open = false;
                    }
                    if ui.button("Ok").clicked() {
                        self.open = false;
                        result = Some(self.selected_track_section);
                    }
                });
                ui.end_row();
            });
        });

        if modal.should_close() {
            self.open = false;
        }

        result
    }

    pub fn open(&mut self) {
        self.open = true;
    }
}
