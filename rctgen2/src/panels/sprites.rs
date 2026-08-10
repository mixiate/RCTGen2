use crate::containers;
use crate::modals;
use crate::widgets;
use eframe::egui;
use egui::containers::scroll_area::ScrollBarVisibility;
use make_track::track_desc::TrackSectionSprites;
use make_track::track_sections::TRACK_SECTIONS;

fn sprite_widgets(sprite: &mut make_track::track_desc::Sprite, ui: &mut egui::Ui) -> bool {
    let mut changed = false;
    if ui
        .add_sized(
            [75.0, ui.style().spacing.interact_size.y],
            egui::DragValue::new(&mut sprite.index).speed(0.0).update_while_editing(false),
        )
        .changed()
    {
        changed = true;
    }
    if ui.add(egui::DragValue::new(&mut sprite.offset[0]).speed(0.05)).changed() {
        changed = true;
    }
    if ui.add(egui::DragValue::new(&mut sprite.offset[1]).speed(0.05)).changed() {
        changed = true;
    }
    if ui.add(egui::DragValue::new(&mut sprite.offset[2]).speed(0.05)).changed() {
        changed = true;
    }
    changed
}

fn sprites_grid(sprites: &mut heapless::Vec<make_track::track_desc::Sprite, 2>, ui: &mut egui::Ui) -> bool {
    let mut changed = false;
    ui.vertical(|ui| {
        let sprites_is_full = sprites.is_full();
        let sprites_len = sprites.len();

        let mut add_sprite = false;
        let mut removed_index = None;

        for (sprite_index, sprite) in sprites.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if sprite_widgets(sprite, ui) {
                    changed = true;
                }
                if widgets::buttons::remove_button(ui) {
                    removed_index = Some(sprite_index);
                }
                if !sprites_is_full && sprites_len - 1 == sprite_index && widgets::buttons::add_button(ui) {
                    add_sprite = true;
                }
            });
            ui.end_row();
        }

        if sprites.is_empty() && widgets::buttons::add_button(ui) {
            add_sprite = true;
        }

        if add_sprite {
            let _ignore_full = sprites.push(make_track::track_desc::Sprite::default());
            changed = true;
        }
        if let Some(removed_index) = removed_index {
            sprites.remove(removed_index);
            changed = true;
        }
    });
    changed
}

fn track_section_body(sprites: &mut TrackSectionSprites, ui: &mut egui::Ui) -> bool {
    let mut changed = false;
    for rotation in 0..4 {
        ui.separator();

        for (tile_index, tiles) in sprites.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                widgets::tile_index_label(ui, tile_index);

                if sprites_grid(&mut tiles[rotation], ui) {
                    changed = true;
                }
            });
        }
    }
    ui.separator();
    changed
}

pub fn sprites_panel(
    sprites: &mut indexmap::IndexMap<String, TrackSectionSprites>,
    track_section_selection_modal: &mut modals::TrackSectionSelectionModal,
    ui: &mut egui::Ui,
) -> bool {
    let mut changed = false;
    egui::Panel::right("Sprites side panel").resizable(false).min_size(340.0).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            if ui.button("Add piece").clicked() {
                track_section_selection_modal.open();
            }
        });
        ui.add(egui::Separator::default().spacing(0.0));

        let mut removed_track_section_index = None;

        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();

        egui::ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                for (index, (track_section_name, sprites)) in sprites.iter_mut().enumerate() {
                    let response = containers::collapsible_with_remove(ui, track_section_name, |ui| {
                        track_section_body(sprites, ui)
                    });
                    if response.changed {
                        changed = true;
                    }
                    if response.removed {
                        removed_track_section_index = Some(index);
                    }
                }
            });

        if let Some(index) = removed_track_section_index {
            sprites.shift_remove_index(index);
            changed = true;
        }
    });

    if let Some(track_section) = track_section_selection_modal.draw(ui) {
        add_track_section(sprites, track_section);
        changed = true;
    }

    changed
}

fn add_track_section(
    sprites: &mut indexmap::IndexMap<String, TrackSectionSprites>,
    track_section: &make_track::track_sections::TrackSection,
) {
    let name = track_section.name.to_string();
    if let indexmap::map::Entry::Vacant(entry) = sprites.entry(name) {
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
