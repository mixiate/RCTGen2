use crate::adjacent_track;
use crate::drawing;
use crate::sprite_cache;
use crate::track_editor;
use crate::ui::widgets::colour_picker;
use crate::ui::widgets::colour_picker::ColourPicker;
use eframe::egui;
use make_track::track_desc;

pub struct Viewport {
    drawing_options: crate::drawing::Options,
    pub rotation: usize,
    pub track_image: Option<track_editor::TrackImage>,
    back_buffer: egui::TextureHandle,
    back_buffer_image: renderer::image::Image,
    colour_buttons: [ColourPicker; 3],
}

impl Viewport {
    pub fn new(egui_context: &egui::Context) -> Self {
        let back_buffer_size = 512;
        let back_buffer = egui::ColorImage::filled([back_buffer_size, back_buffer_size], egui::Color32::TRANSPARENT);
        let back_buffer = egui_context.load_texture("back buffer", back_buffer, egui::TextureOptions::default());
        let mut back_buffer_image = renderer::image::Image::new(back_buffer_size, back_buffer_size);
        back_buffer_image.offset = glam::IVec2::new(back_buffer_size as i32 / 2, back_buffer_size as i32 / 2);

        Viewport {
            drawing_options: Default::default(),
            rotation: 0,
            track_image: None,
            back_buffer,
            back_buffer_image,
            colour_buttons: [ColourPicker::new(), ColourPicker::new(), ColourPicker::new()],
        }
    }

    pub fn clear(&mut self) {
        self.track_image = None;
        self.back_buffer_image.pixels_mut().fill(0);
        let image = egui::ColorImage::from_rgba_unmultiplied(self.back_buffer.size(), self.back_buffer_image.pixels());
        self.back_buffer.set(image, egui::TextureOptions::default());
    }

    pub fn draw(
        &mut self,
        track: &track_desc::Track,
        metal_supports: Option<&track_desc::MetalSupports>,
        adjacent_track_sections: &adjacent_track::AdjacentTrackSections,
        rct2_sprites: Option<&mut sprite_cache::SpriteCache>,
    ) {
        if let Some(track_image) = &self.track_image {
            let max_tile_height = track_image
                .track_section
                .tiles
                .iter()
                .max_by(|a, b| a[2].cmp(&b[2]))
                .map(|x| i32::from(x[2]))
                .unwrap_or(0);
            self.back_buffer_image.offset.y = (self.back_buffer_image.height() as i32 / 2) + (max_tile_height / 2);

            self.back_buffer_image.pixels_mut().fill(0);
            drawing::draw(
                track,
                metal_supports,
                track_image,
                &self.drawing_options,
                adjacent_track_sections,
                rct2_sprites,
                &mut self.back_buffer_image,
            );
            let image =
                egui::ColorImage::from_rgba_unmultiplied(self.back_buffer.size(), self.back_buffer_image.pixels());
            self.back_buffer.set(image, egui::TextureOptions::default());
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        track: &track_editor::Track,
        current_track_section: &make_track::track_sections::TrackSection,
        rct2_sprites_loaded: bool,
        colour_button_textures: &[colour_picker::ButtonTextures],
        changes: &mut track_editor::Changes,
    ) {
        {
            let texture_size = self.back_buffer.size_vec2();
            let image = egui::Image::from_texture((self.back_buffer.id(), texture_size));
            let image_pos = ui.max_rect().center() - (texture_size / egui::Vec2::new(2.0, 2.0));
            let image_rect = egui::Rect::from_min_size(image_pos, texture_size);
            ui.place(image_rect, image);
        }

        let margin = egui::Margin {
            left: 10,
            right: 10,
            top: 10,
            bottom: 5,
        };
        let frame = egui::Frame::popup(ui.style()).outer_margin(margin).shadow(egui::Shadow::NONE);
        frame.show(ui, |ui| {
            if ui.checkbox(&mut self.drawing_options.indexed, "Indexed").changed() {
                changes.redraw = true;
            }
        });
        let frame = frame.outer_margin(egui::Margin::symmetric(10, 5));
        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                for (index, button) in self.colour_buttons.iter_mut().enumerate() {
                    if button.button(ui, colour_button_textures, &mut self.drawing_options.colours[index]) {
                        changes.redraw = true;
                    }
                }
            });
        });
        frame.show(ui, |ui| {
            let supports_checkbox_enabled = if let Some(metal_supports) = &track.desc.metal_supports {
                metal_supports.sections.contains_key(current_track_section.name)
            } else {
                false
            };
            if ui
                .add_enabled(
                    supports_checkbox_enabled && rct2_sprites_loaded,
                    egui::Checkbox::new(&mut self.drawing_options.supports, "Supports"),
                )
                .changed()
            {
                changes.redraw = true;
            }
        });
        frame.show(ui, |ui| {
            let track = &track.desc.tracks[track.track_index];
            let original_track_checkbox_enabled = track.original_sprites.contains_key(current_track_section.name);
            if ui
                .add_enabled(
                    original_track_checkbox_enabled && rct2_sprites_loaded,
                    egui::Checkbox::new(&mut self.drawing_options.original_track, "Original"),
                )
                .changed()
            {
                changes.redraw = true;
            }
            let adjacent_track_checkbox_enabled = !track.original_sprites.is_empty();
            if ui
                .add_enabled(
                    adjacent_track_checkbox_enabled && rct2_sprites_loaded,
                    egui::Checkbox::new(&mut self.drawing_options.adjacent_track, "Adjacent"),
                )
                .changed()
            {
                changes.redraw = true;
            }
        });
        frame.show(ui, |ui| {
            if ui
                .add_sized(
                    egui::Vec2::new(35.0, 35.0),
                    egui::Button::new(egui::RichText::new("↻").size(25.0)),
                )
                .clicked()
            {
                self.rotation = (self.rotation + 1) & 3;
                changes.update_model = true;
            }
        });
    }
}
