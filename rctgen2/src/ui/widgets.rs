use eframe::egui;

pub mod buttons;
pub mod colour_picker;
mod drag_value_spin;

pub use drag_value_spin::DragValueSpin;

pub fn tile_index_label(ui: &mut egui::Ui, tile_index: usize) {
    let tile_label = match tile_index {
        0 => "0:",
        1 => "1:",
        2 => "2:",
        3 => "3:",
        4 => "4:",
        5 => "5:",
        6 => "6:",
        7 => "7:",
        8 => "8:",
        9 => "9:",
        _ => "",
    };
    ui.label(tile_label);
}
