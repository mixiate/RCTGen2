pub mod adjacent_track;
mod file_watcher;
pub mod render;
mod track;
mod viewport;

pub use track::Track;

use crate::settings;
use crate::ui;
use crate::ui::modals;
use crate::ui::panels;
use crate::ui::widgets;
use eframe::egui;
use render::{RenderArgs, RenderMessage, SharedTrackImage, TrackImage, UpdateModelArgs};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub enum TrackEditorMessage {
    NewFrame,
    Error(Vec<String>),
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Default)]
    pub struct Changes: u32 {
        const Directory = 1 << 0;
        const ModelSettings = 1 << 1;
        const LoadModels = 1 << 2;
        const Masks = 1 << 3;
        const Offsets = 1 << 4;
        const UpdateModel = 1 << 5;
        const Render = 1 << 6;
        const ClearImage = 1 << 7;
        const Redraw = 1 << 8;

        const LoadTrack = Self::Directory.bits() | Self::ModelSettings.bits() | Self::LoadModels.bits()
            | Self::Masks.bits() | Self::Offsets.bits() | Self::ClearImage.bits();
        const ChangeSubTrack = Self::ModelSettings.bits() | Self::LoadModels.bits() | Self::Masks.bits();
    }
}

pub enum Action {
    Open(std::path::PathBuf),
    Save,
    Export,
    ChangeTrack(usize),
}

enum SaveStatus {
    Changed,
    ConfirmingOpen(std::path::PathBuf),
    SaveAndOpen(std::path::PathBuf),
    Open(std::path::PathBuf),
    ConfirmingClose,
    SaveAndClose,
    Close,
}

#[derive(Default)]
pub struct ExportSettings {
    pub export_enabled: bool,
    pub skip_empty_sprites: bool,
    pub build_enabled: bool,
    pub build: bool,
}

pub struct TrackEditor {
    render_thread: Option<std::thread::JoinHandle<()>>,
    editor_rx: Receiver<TrackEditorMessage>,
    render_tx: Sender<RenderMessage>,
    track_image: SharedTrackImage,
    track: Track,
    changes: Changes,
    action: Option<Action>,
    save_status: Option<SaveStatus>,
    export_settings: ExportSettings,
    side_panel_tab: Option<panels::SidePanelTab>,
    track_section: &'static make_track::track_sections::TrackSection,
    colour_button_textures: Vec<widgets::colour_picker::ButtonTextures>,
    new_track_modal: modals::NewTrackModal,
    viewport: viewport::Viewport,
    adjacent_track_sections: adjacent_track::AdjacentTrackSections,
    file_watcher: file_watcher::FileWatcher,
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

        let file_watcher = file_watcher::FileWatcher::try_new(egui_context.clone()).unwrap();

        TrackEditor {
            render_thread: Some(render_thread),
            editor_rx,
            render_tx,
            track_image,
            changes: Changes::LoadTrack,
            action: None,
            save_status: None,
            export_settings: ExportSettings::default(),
            side_panel_tab: None,
            track,
            track_section: &make_track::track_sections::FLAT,
            colour_button_textures: widgets::colour_picker::create_colour_button_textures(egui_context),
            new_track_modal: modals::NewTrackModal::new(),
            viewport: viewport::Viewport::new(egui_context),
            adjacent_track_sections,
            file_watcher,
        }
    }

    pub fn logic(
        &mut self,
        egui_context: &egui::Context,
        data_directory: &std::path::Path,
        settings: &mut settings::Settings,
        rct2_sprites: Option<&rct::csg::Archive>,
        errors: &mut Vec<String>,
    ) {
        use bitflags::Flags as _;

        let mut fetch_frame = false;
        for message in self.editor_rx.try_iter() {
            match message {
                TrackEditorMessage::NewFrame => fetch_frame = true,
                TrackEditorMessage::Error(error) => errors.extend(error),
            }
        }

        if fetch_frame
            && let Ok(mut track_image) = self.track_image.lock()
            && track_image.is_some()
        {
            self.viewport.track_image = track_image.take();
            self.changes |= Changes::Redraw;
        }

        match self.file_watcher.check() {
            Some(file_watcher::Status::Delay(time_left)) => egui_context.request_repaint_after(time_left),
            Some(file_watcher::Status::ReloadModels) => self.changes |= Changes::LoadModels,
            None => {}
        }

        let mut save = false;
        let mut path_to_open = None;
        match self.action.take() {
            Some(Action::Open(file_path)) => {
                if let Some(SaveStatus::Changed) = self.save_status {
                    self.save_status = Some(SaveStatus::ConfirmingOpen(file_path));
                } else {
                    path_to_open = Some(file_path);
                }
            }
            Some(Action::Save) => save = true,
            Some(Action::Export) => {
                if let Some(export_directory) = &settings.track_export_directory {
                    match make_track::make_track(
                        data_directory,
                        &self.track.desc,
                        self.track.file_path.directory(),
                        export_directory,
                        self.export_settings.skip_empty_sprites,
                    ) {
                        Ok(_) => {
                            if self.export_settings.build
                                && let Some(input_path) = &settings.track_build_input_path
                                && let Some(output_path) = &settings.track_build_output_path
                                && let Err(error) = sprite_build::build(input_path, output_path)
                            {
                                errors.extend(error.chain().map(|x| x.to_string()));
                            }
                        }
                        Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
                    }
                }
            }
            Some(Action::ChangeTrack(index)) => {
                self.track.track_index = index;
                self.changes |= Changes::ChangeSubTrack;
            }
            None => {}
        }

        if !self.changes.is_empty() && self.save_status.is_none() {
            self.save_status = Some(SaveStatus::Changed);
        }

        match self.save_status.take() {
            Some(SaveStatus::SaveAndOpen(file_path)) => {
                save = true;
                path_to_open = Some(file_path);
            }
            Some(SaveStatus::Open(file_path)) => {
                path_to_open = Some(file_path);
            }
            Some(SaveStatus::SaveAndClose) => {
                egui_context.send_viewport_cmd(egui::ViewportCommand::Close);
                save = true;
            }
            Some(SaveStatus::Close) => {
                egui_context.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            status => self.save_status = status,
        }

        if save {
            match self.track.desc.save(&self.track.file_path) {
                Ok(_) => self.save_status = None,
                Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
            }
        }
        if let Some(file_path) = path_to_open {
            match Track::load(file_path) {
                Ok(new_track) => {
                    self.track = new_track;
                    self.changes |= Changes::LoadTrack;
                    settings.recent_track_files.add(&self.track.file_path);
                }
                Err(error) => errors.extend(error.chain().map(|x| x.to_string())),
            }
        }

        let track = &self.track.desc.tracks[self.track.track_index];
        if self.changes.contains(Changes::Directory) {
            if let Err(error) = self.file_watcher.set_directory(self.track.file_path.directory()) {
                errors.push(error.to_string());
            }
            let _result = self.render_tx.send(RenderMessage::SetDirectory(
                self.track.file_path.directory().to_path_buf(),
            ));
        }
        if self.changes.contains(Changes::ModelSettings) {
            let _result = self.render_tx.send(RenderMessage::UpdateModelSettings(track.model_settings));
            self.changes |= Changes::UpdateModel;
        }
        if self.changes.contains(Changes::LoadModels) {
            let _result = self.render_tx.send(RenderMessage::LoadModels(Box::new(track.models.clone())));
            self.changes |= Changes::UpdateModel;
        }
        if self.changes.contains(Changes::Masks) {
            let _result = self.render_tx.send(RenderMessage::LoadMasks(track.masks.clone()));
            self.changes |= Changes::UpdateModel;
        }
        if self.changes.contains(Changes::Offsets) {
            let _result = self.render_tx.send(RenderMessage::UpdateOffsets(Box::new(self.track.desc.offsets)));
            self.changes |= Changes::UpdateModel;
        }
        if self.changes.contains(Changes::UpdateModel) {
            let _result = self.render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
                track_section: self.track_section,
                rotation: self.viewport.rotation,
            }));
            self.changes |= Changes::Render;
        }
        if self.changes.contains(Changes::Render) {
            let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
                egui_context: egui_context.clone(),
                rotation: self.viewport.rotation,
                samples: self.track.desc.samples.into(),
                dither: self.track.desc.dither,
                edge_distance: self.track.desc.edge_distance,
                lights: self.track.desc.get_lights(),
            }));
        }

        if self.changes.contains(Changes::ClearImage) {
            self.viewport.clear();
        }
        if self.changes.contains(Changes::Redraw) {
            self.viewport.draw(
                track,
                self.track.desc.metal_supports.as_ref(),
                &self.adjacent_track_sections,
                rct2_sprites,
                egui_context.zoom_factor(),
            );
        }

        self.changes.clear();
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut settings::AppSettings,
        rct2_sprites_loaded: bool,
        errors: &mut Vec<String>,
    ) {
        self.export_settings.export_enabled = settings.settings.track_export_directory.is_some();
        self.export_settings.build_enabled =
            settings.settings.track_build_input_path.is_some() && settings.settings.track_build_output_path.is_some();
        self.action = ui::menu_bars::menu_bar(
            ui,
            &mut self.track,
            &mut self.track_section,
            &mut self.export_settings,
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

        if ui.input(|i| i.viewport().close_requested()) && self.save_status.is_some() {
            self.save_status = Some(SaveStatus::ConfirmingClose);
            ui.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }

        match self.save_status.take() {
            Some(SaveStatus::ConfirmingOpen(file_path)) => match modals::save_confirm::save_confirm_modal(ui) {
                Some(modals::save_confirm::Choice::NoSave) => self.save_status = Some(SaveStatus::Open(file_path)),
                Some(modals::save_confirm::Choice::Save) => self.save_status = Some(SaveStatus::SaveAndOpen(file_path)),
                Some(modals::save_confirm::Choice::Cancel) => self.save_status = Some(SaveStatus::Changed),
                None => self.save_status = Some(SaveStatus::ConfirmingOpen(file_path)),
            },
            Some(SaveStatus::ConfirmingClose) => match modals::save_confirm::save_confirm_modal(ui) {
                Some(modals::save_confirm::Choice::NoSave) => self.save_status = Some(SaveStatus::Close),
                Some(modals::save_confirm::Choice::Save) => self.save_status = Some(SaveStatus::SaveAndClose),
                Some(modals::save_confirm::Choice::Cancel) => self.save_status = Some(SaveStatus::Changed),
                None => self.save_status = Some(SaveStatus::ConfirmingClose),
            },
            status => self.save_status = status,
        }

        if !self.changes.is_empty() || self.action.is_some() {
            ui.ctx().request_repaint();
        }
    }

    pub fn on_exit(&mut self) {
        let _result = self.render_tx.send(RenderMessage::Exit);
        self.render_thread.take().map(std::thread::JoinHandle::join);
    }
}
