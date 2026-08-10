pub mod lights;
pub mod metal_supports;
pub mod offsets;
pub mod render;
pub mod sprites;

use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
pub enum SidePanelTab {
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
                side_panel_tab(ui, "💡", selected_tab, SidePanelTab::Lights);
                side_panel_tab(ui, "↔", selected_tab, SidePanelTab::Offsets);
                side_panel_tab(ui, "|", selected_tab, SidePanelTab::MetalSupports);
                side_panel_tab(ui, "📷", selected_tab, SidePanelTab::Render);
                side_panel_tab(ui, "🖼️", selected_tab, SidePanelTab::Sprites);
            });
        });
    });
}
