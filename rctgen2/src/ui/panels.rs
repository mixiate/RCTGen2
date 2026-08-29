pub mod lights;
pub mod metal_supports;
pub mod offsets;
pub mod render;
pub mod sprites;
pub mod tracks;

use crate::app;
use crate::ui;
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
    sprites_track_selection_modal: &mut ui::modals::TrackSectionSelectionModal,
    track_desc_path: &mut Option<std::path::PathBuf>,
    track_desc: &mut make_track::track_desc::Desc,
    current_track_section: &TrackSection,
    rotation: usize,
    changes: &mut app::Changes,
    errors: &mut Vec<String>,
) {
    match tab {
        SidePanelTab::Tracks => {
            if let Some(path) = &track_desc_path
                && let Some(directory) = path.parent()
            {
                tracks::tracks_panel(
                    &mut track_desc.tracks,
                    directory,
                    errors,
                    current_track_section,
                    changes,
                    ui,
                );
            }
        }
        SidePanelTab::Lights => {
            let changed = lights::lights_panel(&mut track_desc.lights, ui);
            if changed {
                changes.render = true;
            }
        }
        SidePanelTab::Offsets => {
            let changed = offsets::offsets_panel(&mut track_desc.offsets, ui);
            if changed {
                changes.offsets = true;
            }
        }
        SidePanelTab::MetalSupports => {
            let changed = metal_supports::metal_supports_panel(
                &mut track_desc.metal_supports,
                rotation,
                current_track_section,
                ui,
            );
            if changed {
                changes.redraw = true;
            }
        }
        SidePanelTab::Render => {
            let changed = render::render_panel(track_desc, ui);
            if changed {
                changes.render = true;
            }
        }
        SidePanelTab::Sprites => {
            let changed = sprites::sprites_panel(&mut track_desc.original_sprites, sprites_track_selection_modal, ui);
            if changed {
                changes.redraw = true;
            }
        }
    }
}
