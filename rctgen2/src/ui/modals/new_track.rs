use crate::ui::panels::tracks::track_widgets;
use eframe::egui;

struct NewTrack {
    directory: std::path::PathBuf,
    track: Box<make_track::track_desc::Track>,
}

enum State {
    Closed,
    Open(NewTrack),
}

pub struct NewTrackModal {
    id: egui::Id,
    state: State,
}

impl NewTrackModal {
    pub fn new() -> Self {
        NewTrackModal {
            id: egui::Id::new("New track modal"),
            state: State::Closed,
        }
    }

    pub fn open(&mut self, directory: std::path::PathBuf) {
        self.state = State::Open(NewTrack {
            directory,
            track: Box::default(),
        });
    }

    pub fn show(&mut self, ui: &mut egui::Ui, errors: &mut Vec<String>) -> Option<Box<make_track::track_desc::Track>> {
        let mut confirmed = false;
        let mut should_close = false;
        if let State::Open(new_track) = &mut self.state {
            let modal = egui::containers::modal::Modal::new(self.id).show(ui.ctx(), |ui| {
                ui.label("New Track");
                ui.separator();

                egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {
                    track_widgets(
                        ui,
                        0,
                        &mut new_track.track,
                        &new_track.directory,
                        errors,
                        &make_track::track_sections::FLAT,
                        &mut Default::default(),
                    );
                });
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() {
                        confirmed = true;
                    }
                    if ui.button("Cancel").clicked() {
                        ui.close();
                    }
                });
            });

            if modal.should_close() {
                should_close = true;
            }
        }
        if should_close {
            self.state = State::Closed;
        }
        if confirmed && let State::Open(new_track) = std::mem::replace(&mut self.state, State::Closed) {
            Some(new_track.track)
        } else {
            None
        }
    }
}
