pub mod lights;
pub mod metal_supports;
pub mod offsets;
pub mod render;
pub mod sprites;
pub mod tracks;

use crate::track_editor;
use crate::ui::modals;
use eframe::egui;
use make_track::track_sections::TrackSection;

#[derive(Clone, Copy, PartialEq)]
pub enum SidePanelTab {
    Tracks,
    Lights,
    Offsets,
    MetalSupports,
    Render,
    Sprites,
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

pub fn side_panel_tabs(ui: &mut egui::Ui, selected_tab: &mut Option<SidePanelTab>) {
    let mut frame = egui::Frame::side_top_panel(ui.style());
    frame.inner_margin = egui::Margin::ZERO;

    egui::Panel::right("SidePanelTabs").resizable(false).exact_size(40.0).frame(frame).show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                side_panel_tab(ui, "🎢", selected_tab, SidePanelTab::Tracks);
                side_panel_tab(ui, "💡", selected_tab, SidePanelTab::Lights);
                side_panel_tab(ui, "📷", selected_tab, SidePanelTab::Render);
                side_panel_tab(ui, "↔", selected_tab, SidePanelTab::Offsets);
                side_panel_tab(ui, "|", selected_tab, SidePanelTab::MetalSupports);
                side_panel_tab(ui, "🖼️", selected_tab, SidePanelTab::Sprites);
            });
        });
    });
}

#[expect(clippy::too_many_arguments)]
pub fn side_panel(
    ui: &mut egui::Ui,
    tab: SidePanelTab,
    new_track_modal: &mut modals::NewTrackModal,
    track: &mut track_editor::Track,
    current_track_section: &TrackSection,
    rotation: usize,
    changes: &mut track_editor::Changes,
    errors: &mut Vec<String>,
) {
    match tab {
        SidePanelTab::Tracks => {
            tracks::tracks_panel(track, errors, current_track_section, new_track_modal, changes, ui);
        }
        SidePanelTab::Lights => {
            let changed = lights::lights_panel(&mut track.desc.lights, ui);
            if changed {
                changes.render = true;
            }
        }
        SidePanelTab::Offsets => {
            let changed = offsets::offsets_panel(&mut track.desc.offsets, ui);
            if changed {
                changes.offsets = true;
            }
        }
        SidePanelTab::MetalSupports => {
            let changed = metal_supports::metal_supports_panel(
                &mut track.desc.metal_supports,
                rotation,
                current_track_section,
                ui,
            );
            if changed {
                changes.redraw = true;
            }
        }
        SidePanelTab::Render => {
            let changed = render::render_panel(&mut track.desc, ui);
            if changed {
                changes.render = true;
            }
        }
        SidePanelTab::Sprites => {
            if let Some(track) = track.desc.tracks.get_mut(track.track_index) {
                let changed = sprites::sprites_panel(&mut track.original_sprites, current_track_section, ui);
                if changed {
                    changes.redraw = true;
                }
            }
        }
    }
}
