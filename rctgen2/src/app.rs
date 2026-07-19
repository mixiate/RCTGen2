use crate::render::{LoadTrackArgs, RenderArgs, RenderMessage, SharedTexture, Texture, UpdateModelArgs};
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};

pub enum AppMessage {
    NewFrame,
    Error(Vec<String>),
}

#[derive(Clone, Copy, PartialEq)]
enum SidePanelTab {
    Lights,
    Offsets,
}

pub struct RctGen2App {
    app_rx: Receiver<AppMessage>,
    render_tx: Sender<RenderMessage>,
    render_texture: SharedTexture,
    errors: Vec<String>,
    side_panel_tab: Option<SidePanelTab>,
    track_desc_path: Option<std::path::PathBuf>,
    track_desc: Option<make_track::track_desc::Desc>,
    track_section: &'static make_track::track_sections::TrackSection,
    samples: usize,
    indexed: bool,
    dither: bool,
    rotation: usize,
    texture: Option<Texture>,
}

impl RctGen2App {
    pub fn new(app_rx: Receiver<AppMessage>, render_tx: Sender<RenderMessage>, render_texture: SharedTexture) -> Self {
        Self {
            app_rx,
            render_tx,
            render_texture,
            errors: Vec::new(),
            side_panel_tab: None,
            track_desc_path: None,
            track_desc: None,
            track_section: &make_track::track_sections::FLAT,
            samples: 4,
            indexed: true,
            dither: true,
            rotation: 0,
            texture: None,
        }
    }

    fn load_track(&mut self, egui_context: egui::Context) -> anyhow::Result<()> {
        use anyhow::Context as _;

        let file_result = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file();
        if let Some(file_path) = file_result {
            let directory = file_path
                .parent()
                .with_context(|| format!("Could not get parent directory of {}", file_path.display()))?
                .to_path_buf();
            let track_desc = make_track::track_desc::Desc::load(&file_path)?;

            let _result = self.render_tx.send(RenderMessage::LoadTrack(Box::new(LoadTrackArgs {
                track_desc: track_desc.clone(),
                directory,
            })));
            self.update_model();
            self.dither = track_desc.dither;
            self.track_desc_path = Some(file_path);
            self.track_desc = Some(track_desc);
            self.queue_render(egui_context);
        }
        Ok(())
    }

    fn update_offsets(&self) {
        if let Some(track_desc) = &self.track_desc {
            let _result = self.render_tx.send(RenderMessage::UpdateOffsets(Box::new(track_desc.offsets)));
        }
    }

    fn update_model(&self) {
        let _result = self.render_tx.send(RenderMessage::UpdateModel(UpdateModelArgs {
            track_section: self.track_section,
            rotation: self.rotation,
        }));
    }

    fn queue_render(&self, egui_context: egui::Context) {
        if let Some(track_desc) = &self.track_desc {
            let _result = self.render_tx.send(RenderMessage::Render(RenderArgs {
                egui_context,
                rotation: self.rotation,
                samples: self.samples,
                dither: self.dither,
                indexed: self.indexed,
                lights: track_desc.get_lights(),
            }));
        }
    }

    fn draw_lights_panel(&mut self, ui: &mut egui::Ui) {
        let mut queue_render = false;
        if let Some(track_desc) = self.track_desc.as_mut() {
            egui::Panel::right("Lights").resizable(false).show(ui, |ui| {
                let mut deleted_index = None;
                ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                let visibility = egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible;
                egui::ScrollArea::vertical().scroll_bar_visibility(visibility).show(ui, |ui| {
                    for (i, light) in track_desc.lights.iter_mut().enumerate() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.scope(|ui| {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                                if ui.add(egui::Button::new("✖").fill(egui::Color32::LIGHT_RED)).clicked() {
                                    deleted_index = Some(i);
                                    queue_render = true;
                                }
                            });

                            if inverted_checkbox(ui, &mut light.disabled) {
                                queue_render = true;
                            }
                        });
                        ui.columns_const(|[col_0, col_1]| {
                            col_0.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                if drag_value(ui, &mut light.direction[0], "X", None) {
                                    queue_render = true;
                                }
                            });
                            col_0.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                if drag_value(ui, &mut light.direction[1], "Y", None) {
                                    queue_render = true;
                                }
                            });
                            col_0.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                if drag_value(ui, &mut light.direction[2], "Z", None) {
                                    queue_render = true;
                                }
                            });

                            col_1.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                if drag_value(ui, &mut light.diffuse_strength, "Diffuse", Some(0.0..=2.0)) {
                                    queue_render = true;
                                }
                            });
                            col_1.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                if drag_value(ui, &mut light.specular_strength, "Specular", Some(0.0..=2.0)) {
                                    queue_render = true;
                                }
                            });
                            col_1.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                if ui.checkbox(&mut light.shadow, "Shadow").clicked() {
                                    queue_render = true;
                                }
                            });
                        });

                        ui.separator();
                    }
                    ui.vertical_centered(|ui| {
                        ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                        if ui.add(egui::Button::new("Add light").fill(egui::Color32::LIGHT_GREEN)).clicked() {
                            track_desc.lights.push(make_track::track_desc::Light {
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
                    track_desc.lights.remove(i);
                    queue_render = true;
                }
            });
        }
        if queue_render {
            self.queue_render(ui.ctx().clone());
        }
    }

    fn draw_offsets_panel(&mut self, ui: &mut egui::Ui) {
        let mut removed_offsets = false;
        let mut update_offsets = false;

        if let Some(track_desc) = self.track_desc.as_mut() {
            egui::Panel::right("Offsets").resizable(false).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    if track_desc.offsets.is_some() {
                        if ui.button("Remove offsets").clicked() {
                            track_desc.offsets = None;
                            removed_offsets = true;
                            update_offsets = true;
                        }
                        ui.separator();
                    }
                });
                if let Some(offsets) = track_desc.offsets.as_mut() {
                    let mut remove_gentle_banked_right = false;

                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                    let visibility = egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible;
                    egui::ScrollArea::vertical().scroll_bar_visibility(visibility).show(ui, |ui| {
                        if offsets_widget(ui, "Flat", &mut offsets.flat) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Gentle", &mut offsets.gentle) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Steep", &mut offsets.steep) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Flat Banked", &mut offsets.flat_banked) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if let Some(gentle_banked_right) = offsets.gentle_banked_right.as_mut() {
                            if offsets_widget(ui, "Gentle Banked Left", &mut offsets.gentle_banked) {
                                update_offsets = true;
                            }
                            ui.separator();
                            if offsets_widget(ui, "Gentle Banked Right", gentle_banked_right) {
                                update_offsets = true;
                            }
                            ui.vertical_centered(|ui| {
                                if ui.button("Remove Gentle Banked Right").clicked() {
                                    remove_gentle_banked_right = true;
                                }
                            });
                        } else {
                            if offsets_widget(ui, "Gentle Banked", &mut offsets.gentle_banked) {
                                update_offsets = true;
                            }
                            ui.separator();
                            ui.vertical_centered(|ui| {
                                if ui.button("Add Gentle Banked Right").clicked() {
                                    offsets.gentle_banked_right = Some(Default::default());
                                }
                            });
                        }
                        ui.separator();
                        if offsets_widget(ui, "Inverted", &mut offsets.inverted) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Diagonal", &mut offsets.diagonal) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Diagonal Gentle", &mut offsets.diagonal_gentle) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Diagonal Steep", &mut offsets.diagonal_steep) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Diagonal Banked", &mut offsets.diagonal_banked) {
                            update_offsets = true;
                        }
                        ui.separator();
                        if offsets_widget(ui, "Vertical", &mut offsets.vertical) {
                            update_offsets = true;
                        }
                    });
                    if remove_gentle_banked_right {
                        offsets.gentle_banked_right = None;
                    }
                } else if !removed_offsets {
                    ui.vertical_centered(|ui| {
                        if ui.button("Add offsets").clicked() {
                            track_desc.offsets = Some(make_track::track_desc::Offsets::default());
                        }
                    });
                }
            });
        }

        if update_offsets {
            self.update_offsets();
            self.update_model();
            self.queue_render(ui.ctx().clone());
        }
    }

    fn draw_side_panel(&mut self, ui: &mut egui::Ui) {
        match self.side_panel_tab {
            Some(SidePanelTab::Lights) => self.draw_lights_panel(ui),
            Some(SidePanelTab::Offsets) => self.draw_offsets_panel(ui),
            None => {}
        }
    }
}

impl eframe::App for RctGen2App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut fetch_frame = false;
        for message in self.app_rx.try_iter() {
            match message {
                AppMessage::NewFrame => fetch_frame = true,
                AppMessage::Error(errors) => self.errors.extend(errors),
            }
        }

        let mut update_model = false;
        let mut queue_render = false;
        let previous_track_section = self.track_section;

        egui::Panel::top("Top Menu").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.add(egui::Button::new("Open...").min_size(egui::Vec2::new(200.0, 0.0))).clicked()
                        && let Err(error) = self.load_track(ui.ctx().clone())
                    {
                        self.errors.extend(error.chain().map(|x| x.to_string()));
                    }

                    if let Some(path) = &self.track_desc_path
                        && let Some(track_desc) = &self.track_desc
                    {
                        if ui.button("Save").clicked()
                            && let Err(error) = track_desc.save(path)
                        {
                            self.errors.extend(error.chain().map(|x| x.to_string()));
                        }
                    } else {
                        ui.add_enabled(false, egui::Button::new("Save"));
                    }

                    ui.separator();

                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                if ui.add(egui::DragValue::new(&mut self.samples).prefix("Samples: ").range(1..=4)).changed() {
                    queue_render = true;
                }
                if ui.checkbox(&mut self.indexed, "Indexed").changed() {
                    queue_render = true;
                }
                if ui.checkbox(&mut self.dither, "Dithered").changed() {
                    queue_render = true;
                }

                egui::ComboBox::from_id_salt("Track section")
                    .selected_text(self.track_section.name)
                    .width(300.0)
                    .height(500.0)
                    .show_ui(ui, |ui| {
                        for track_section in make_track::track_sections::TRACK_SECTIONS {
                            ui.selectable_value(&mut self.track_section, track_section, track_section.name);
                        }
                    });
                if self.track_section != previous_track_section {
                    update_model = true;
                    queue_render = true;
                }

                if ui.add(egui::Button::new("↻")).clicked() {
                    self.rotation += 1;
                    if self.rotation == 4 {
                        self.rotation = 0;
                    }
                    update_model = true;
                    queue_render = true;
                }
            });
        });

        side_panel_tabs(ui, &mut self.side_panel_tab);

        self.draw_side_panel(ui);

        if update_model {
            self.update_model();
        }
        if queue_render {
            self.queue_render(ui.ctx().clone());
        }

        if fetch_frame
            && let Ok(mut render_texture) = self.render_texture.lock()
            && render_texture.is_some()
        {
            self.texture = render_texture.take();
        }

        let frame = egui::Frame::default().fill(egui::Color32::from_rgb(34, 33, 39));
        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            if let Some(texture) = &self.texture {
                let texture_size = texture.handle.size_vec2();
                let image = egui::Image::from_texture((texture.handle.id(), texture_size));

                let image_pos = ui.max_rect().center();
                let image_pos = image_pos + egui::Vec2::new(texture.offset.x as f32, texture.offset.y as f32);
                let image_rect = egui::Rect::from_min_size(image_pos, texture_size);

                ui.place(image_rect, image);
            }
        });

        if !self.errors.is_empty() {
            let modal = egui::containers::modal::Modal::new(egui::Id::new("Error")).show(ui.ctx(), |ui| {
                ui.set_width(600.0);

                ui.vertical_centered(|ui| {
                    ui.heading("Error");
                    ui.separator();

                    for error in &self.errors {
                        ui.add(egui::Label::new(error));
                    }
                });
            });

            if modal.should_close() {
                self.errors.clear();
            }
        }
    }

    fn on_exit(&mut self) {
        let _result = self.render_tx.send(RenderMessage::Exit);
    }
}

fn drag_value(ui: &mut egui::Ui, value: &mut f32, label: &str, range: Option<core::ops::RangeInclusive<f32>>) -> bool {
    let mut changed = false;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        ui.scope(|ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
            if ui.button("+").clicked() {
                *value += 0.01;
                changed = true;
            }
            let mut drag_value = egui::DragValue::new(value).speed(0.01);
            if let Some(range) = range {
                drag_value = drag_value.clamp_existing_to_range(false).range(range);
            }
            if ui.add_sized(egui::vec2(75.0, 10.0), drag_value).changed() {
                changed = true;
            }
            if ui.button("-").clicked() {
                *value -= 0.01;
                changed = true;
            }
        });

        ui.label(label);
    });
    changed
}

fn inverted_checkbox(ui: &mut egui::Ui, value: &mut bool) -> bool {
    let mut inverse_value = !(*value);
    if ui.checkbox(&mut inverse_value, "Enabled").clicked() {
        *value = !inverse_value;
        true
    } else {
        false
    }
}

fn offsets_widget(ui: &mut egui::Ui, name: &str, offsets: &mut [[f32; 2]]) -> bool {
    let mut changed = false;
    ui.label(name);
    ui.columns_const(|[col_0, col_1]| {
        for offset in offsets.iter_mut() {
            if drag_value(col_0, &mut offset[0], "X", None) {
                changed = true;
            }
            if drag_value(col_1, &mut offset[1], "Y", None) {
                changed = true;
            }
        }
    });
    changed
}

fn side_panel_tab(ui: &mut egui::Ui, text: &str, selected_tab: &mut Option<SidePanelTab>, tab: SidePanelTab) {
    let selected = *selected_tab == Some(tab);
    if ui
        .add_sized(
            [40.0, 40.0],
            egui::Button::new(egui::RichText::new(text).size(25.0)).selected(selected).corner_radius(0.0),
        )
        .clicked()
    {
        if selected {
            *selected_tab = None;
        } else {
            *selected_tab = Some(tab);
        }
    }
}

fn side_panel_tabs(ui: &mut egui::Ui, selected_tab: &mut Option<SidePanelTab>) {
    let mut frame = egui::Frame::side_top_panel(ui.style());
    frame.inner_margin = egui::Margin::ZERO;

    egui::Panel::right("SidePanelTabs").resizable(false).exact_size(40.0).frame(frame).show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                side_panel_tab(ui, "💡", selected_tab, SidePanelTab::Lights);
                side_panel_tab(ui, "↔", selected_tab, SidePanelTab::Offsets);
            });
        });
    });
}
