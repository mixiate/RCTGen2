mod track;
mod viewport;

pub use track::Track;

use crate::adjacent_track;
use crate::file_watcher;
use crate::render;
use crate::render::{RenderArgs, RenderMessage, SharedTrackImage, TrackImage, UpdateModelArgs};
use crate::settings;
use crate::sprite_cache;
use crate::ui;
use crate::ui::modals;
use crate::ui::panels;
use crate::ui::widgets;
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub enum TrackEditorMessage {
    NewFrame,
    ModelFileChanged,
    Error(Vec<String>),
}

#[derive(Clone, Copy, Default)]
pub struct Changes {
    pub directory: bool,
    pub model_settings: bool,
    pub load_models: bool,
    pub masks: bool,
    pub offsets: bool,
    pub update_model: bool,
    pub render: bool,
    pub clear_image: bool,
    pub redraw: bool,
}

impl Changes {
    pub fn load_track(&mut self) {
        self.directory = true;
        self.model_settings = true;
        self.load_models = true;
        self.masks = true;
        self.offsets = true;
        self.clear_image = true;
    }

    pub fn any(&self) -> bool {
        self.directory
            || self.model_settings
            || self.load_models
            || self.masks
            || self.offsets
            || self.update_model
            || self.render
            || self.clear_image
            || self.redraw
    }
}

pub struct TrackEditor {
    render_thread: Option<std::thread::JoinHandle<()>>,
    editor_rx: Receiver<TrackEditorMessage>,
    render_tx: Sender<RenderMessage>,
    track_image: SharedTrackImage,
    track: Track,
    changes: Changes,
    side_panel_tab: Option<panels::SidePanelTab>,
    track_section: &'static make_track::track_sections::TrackSection,
    colour_button_textures: Vec<widgets::colour_picker::ButtonTextures>,
    new_track_modal: modals::NewTrackModal,
    viewport: viewport::Viewport,
    adjacent_track_sections: adjacent_track::AdjacentTrackSections,
    file_watcher: file_watcher::FileWatcher,
    model_file_changed_time: Option<std::time::Instant>,
}

impl TrackEditor {
    pub fn new(
        egui_context: &egui::Context,
        data_directory: &std::path::Path,
        track: Track,
        errors: &mut Vec<String>,
    ) -> Self {
        let (render_tx, render_rx) = std::sync::mpsc::channel();
        let (editor_tx, editor_rx) = std::sync::mpsc::channel();
        let track_image = Arc::new(Mutex::new(None));

        let mut changes = Changes::default();
        changes.load_track();

        let render_thread = {
            let editor_tx = editor_tx.clone();
            let track_image = track_image.clone();
            let data_directory = data_directory.to_path_buf();
            std::thread::spawn(move || render::render_thread(&render_rx, &editor_tx, &track_image, &data_directory))
        };

        let adjacent_track_sections = data_directory.join("adjacent_track_sections").with_extension("json");
        let adjacent_track_sections = match adjacent_track::load_adjacent_track_sections(&adjacent_track_sections) {
            Ok(sections) => sections,
            Err(error) => {
                errors.extend(error.chain().map(|x| x.to_string()));
                Default::default()
            }
        };

        let file_watcher = file_watcher::FileWatcher::try_new(editor_tx, egui_context.clone()).unwrap();

        TrackEditor {
            render_thread: Some(render_thread),
            editor_rx,
            render_tx,
            track_image,
            changes,
            side_panel_tab: None,
            track,
            track_section: &make_track::track_sections::FLAT,
            colour_button_textures: widgets::colour_picker::create_colour_button_textures(egui_context),
            new_track_modal: modals::NewTrackModal::new(),
            viewport: viewport::Viewport::new(egui_context),
            adjacent_track_sections,
            file_watcher,
            model_file_changed_time: None,
        }
    }

    pub fn logic(
        &mut self,
        egui_context: &egui::Context,
        rct2_sprites: Option<&mut sprite_cache::SpriteCache>,
        errors: &mut Vec<String>,
    ) {
        let mut fetch_frame = false;
        for message in self.editor_rx.try_iter() {
            match message {
                TrackEditorMessage::NewFrame => fetch_frame = true,
                TrackEditorMessage::ModelFileChanged => self.model_file_changed_time = Some(std::time::Instant::now()),
                TrackEditorMessage::Error(error) => errors.extend(error),
            }
        }

        if fetch_frame
            && let Ok(mut track_image) = self.track_image.lock()
            && track_image.is_some()
        {
            self.viewport.track_image = track_image.take();
            self.changes.redraw = true;
        }

        if let Some(time) = self.model_file_changed_time {
            if let Some(time_left) = std::time::Duration::from_millis(250).checked_sub(time.elapsed()) {
                egui_context.request_repaint_after(time_left);
            } else {
                self.model_file_changed_time = None;
                self.changes.load_models = true;
            }
        }

        let track = &self.track.desc.tracks[self.track.track_index];
        if self.changes.directory {
            if let Err(error) = self.file_watcher.set_directory(self.track.file_path.directory()) {
                errors.push(error.to_string());
            }
            let _result = self.render_tx.send(RenderMessage::SetDirectory(
                self.track.file_path.directory().to_path_buf(),
            ));
        }
        if self.changes.model_settings {
            let _result = self.render_tx.send(RenderMessage::UpdateModelSettings(track.model_settings));
            self.changes.update_model = true;
        }
        if self.changes.load_models {
            let _result = self.render_tx.send(RenderMessage::LoadModels(Box::new(track.models.clone())));
            self.changes.update_model = true;
        }
        if self.changes.masks {
            let _result = self.render_tx.send(RenderMessage::LoadMasks(track.masks.clone()));
            self.changes.update_model = true;
        }
        if self.changes.offsets {
            let _result = self.render_tx.send(RenderMessage::UpdateOffsets(Box::new(self.track.desc.offsets)));
            self.changes.update_model = true;
        }
        if self.changes.update_model {
            let _result = self.render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
                track_section: self.track_section,
                rotation: self.viewport.rotation,
            }));
            self.changes.render = true;
        }
        if self.changes.render {
            let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
                egui_context: egui_context.clone(),
                rotation: self.viewport.rotation,
                samples: self.track.desc.samples.into(),
                dither: self.track.desc.dither,
                edge_distance: self.track.desc.edge_distance,
                lights: self.track.desc.get_lights(),
            }));
        }

        if self.changes.clear_image {
            self.viewport.clear();
        }
        if self.changes.redraw {
            self.viewport.draw(
                track,
                self.track.desc.metal_supports.as_ref(),
                &self.adjacent_track_sections,
                rct2_sprites,
            );
        }

        self.changes = Changes::default();
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut settings::AppSettings,
        rct2_sprites_loaded: bool,
        errors: &mut Vec<String>,
    ) {
        ui::menu_bars::menu_bar(
            ui,
            &mut self.track,
            &mut self.track_section,
            settings,
            &mut self.changes,
            errors,
        );

        panels::side_panel_tabs(ui, &mut self.side_panel_tab);

        if let Some(tab) = self.side_panel_tab {
            panels::side_panel(
                ui,
                tab,
                &mut self.new_track_modal,
                &mut self.track,
                self.track_section,
                self.viewport.rotation,
                &mut self.changes,
                errors,
            );
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(23, 35, 35));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            self.viewport.show(
                ui,
                &self.track,
                self.track_section,
                rct2_sprites_loaded,
                &self.colour_button_textures,
                &mut self.changes,
            );
        });

        if self.changes.any() {
            ui.ctx().request_repaint();
        }
    }

    pub fn on_exit(&mut self) {
        let _result = self.render_tx.send(RenderMessage::Exit);
        self.render_thread.take().map(std::thread::JoinHandle::join);
    }
}
