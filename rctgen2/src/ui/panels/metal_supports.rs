use crate::ui::containers;
use crate::ui::widgets;
use eframe::egui;
use make_track::track_desc::{MetalSupport, TrackSectionMetalSupports};
use make_track::track_sections::TRACK_SECTIONS;
use openrct2::supports::{MetalSupportType, SupportPosition};

fn support_widgets(support: &mut MetalSupport, rotation: usize, ui: &mut egui::Ui) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Height");
        if ui.add(egui::DragValue::new(&mut support.height).speed(0.05)).changed() {
            changed = true;
        }
        ui.label("Rotation");
        if ui.add(egui::DragValue::new(&mut support.rotation).speed(0.01).range(0..=3)).changed() {
            changed = true;
        }
        if ui.checkbox(&mut support.alternates, "Alt").changed() {
            changed = true;
        }
        if ui.checkbox(&mut support.prevent_double_offset, "B").changed() {
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Extra heights");
        for (i, extra_height) in support.extra_heights.iter_mut().enumerate() {
            if ui.add_enabled(i == rotation, egui::DragValue::new(extra_height).speed(0.05)).changed() {
                changed = true;
            }
        }
    });

    changed
}

fn track_section_body(supports: &mut TrackSectionMetalSupports, rotation: usize, ui: &mut egui::Ui) -> bool {
    use strum::IntoEnumIterator as _;

    let mut changed = false;
    let mut added_tile_index = None;
    for (tile_index, support) in supports.iter_mut().enumerate() {
        ui.separator();

        let mut remove_support = false;
        if let Some(support) = support.as_mut() {
            ui.horizontal(|ui| {
                ui.label(format!("{tile_index}:"));

                let selected_text: &'static str = support.position.rotate(rotation).into();
                egui::ComboBox::from_id_salt(tile_index)
                    .selected_text(selected_text)
                    .width(135.0)
                    .height(500.0)
                    .show_ui(ui, |ui| {
                        for support_position in SupportPosition::iter() {
                            let text: &'static str = support_position.rotate(rotation).into();
                            if ui.selectable_value(&mut support.position, support_position, text).changed() {
                                changed = true;
                            }
                        }
                    });
                let actual_position: &'static str = support.position.into();
                ui.label(actual_position);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if widgets::buttons::remove_button(ui) {
                        remove_support = true;
                    }
                });
            });

            if support_widgets(support, rotation, ui) {
                changed = true;
            }
        } else {
            ui.horizontal(|ui| {
                ui.label(format!("{tile_index}:"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if widgets::buttons::add_button(ui) {
                        added_tile_index = Some(tile_index);
                    }
                });
            });
        }

        if remove_support {
            *support = None;
            changed = true;
        }
    }
    ui.separator();

    if let Some(added_tile_index) = added_tile_index
        && let Some(support) = supports.get_mut(added_tile_index)
    {
        *support = Some(Default::default());
        changed = true;
    }
    changed
}

fn main_panel(
    metal_supports: &mut make_track::track_desc::MetalSupports,
    rotation: usize,
    current_track_section: &make_track::track_sections::TrackSection,
    ui: &mut egui::Ui,
) -> bool {
    use strum::IntoEnumIterator as _;

    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label("Type: ");

        let selected_text: &'static str = metal_supports.support_type.into();
        egui::ComboBox::from_id_salt("Metal support type")
            .selected_text(selected_text)
            .width(125.0)
            .height(500.0)
            .show_ui(ui, |ui| {
                for support_type in MetalSupportType::iter() {
                    let text: &'static str = support_type.into();
                    if ui.selectable_value(&mut metal_supports.support_type, support_type, text).changed() {
                        changed = true;
                    }
                }
            });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.allocate_space(egui::Vec2::new(ui.style().spacing.scroll.floating_width, 0.0));
            if ui.button("Add current section").clicked() {
                add_track_section(&mut metal_supports.sections, current_track_section);
            }
        });
    });
    ui.add(egui::Separator::default().spacing(0.0));

    let mut removed_track_section_index = None;
    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
    egui::ScrollArea::vertical()
        .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .auto_shrink(false)
        .show(ui, |ui| {
            for (index, (track_section_name, supports)) in metal_supports.sections.iter_mut().enumerate() {
                let frame = if current_track_section.name == track_section_name {
                    egui::Frame::new().fill(ui.visuals().faint_bg_color)
                } else {
                    egui::Frame::new()
                };
                frame.show(ui, |ui| {
                    let response = containers::collapsible_with_remove(ui, track_section_name, |ui| {
                        track_section_body(supports, rotation, ui)
                    });
                    if response.changed {
                        changed = true;
                    }
                    if response.removed {
                        removed_track_section_index = Some(index);
                    }
                });
            }
        });

    if let Some(index) = removed_track_section_index {
        metal_supports.sections.shift_remove_index(index);
        changed = true;
    }

    changed
}

pub fn metal_supports_panel(
    metal_supports: &mut Option<make_track::track_desc::MetalSupports>,
    rotation: usize,
    current_track_section: &make_track::track_sections::TrackSection,
    ui: &mut egui::Ui,
) -> bool {
    let mut changed = false;
    egui::Panel::right("Metal supports side panel").resizable(false).min_size(340.0).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("Metal Supports");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.allocate_space(egui::Vec2::new(ui.style().spacing.scroll.floating_width, 0.0));
                if metal_supports.is_some() {
                    if widgets::buttons::remove_button(ui) {
                        *metal_supports = None;
                        changed = true;
                    }
                } else if widgets::buttons::add_button(ui) {
                    *metal_supports = Some(Default::default());
                    changed = true;
                }
            });
        });
        ui.add(egui::Separator::default().spacing(0.0));

        if let Some(metal_supports) = metal_supports
            && main_panel(metal_supports, rotation, current_track_section, ui)
        {
            changed = true;
        }
    });
    changed
}

fn add_track_section(
    sections: &mut indexmap::IndexMap<String, TrackSectionMetalSupports>,
    track_section: &make_track::track_sections::TrackSection,
) {
    let name = track_section.name.to_string();
    if let indexmap::map::Entry::Vacant(entry) = sections.entry(name) {
        let mut tiles = heapless::Vec::new();
        let _ignore_result = tiles.resize_default(track_section.tiles.len());

        entry.insert_sorted_by(tiles, |key_a, _, key_b, _| {
            let a_index = TRACK_SECTIONS.iter().position(|x| x.name == key_a);
            let b_index = TRACK_SECTIONS.iter().position(|x| x.name == key_b);
            if let Some(a_index) = a_index
                && let Some(b_index) = b_index
            {
                a_index.cmp(&b_index)
            } else {
                std::cmp::Ordering::Less
            }
        });
    }
}
