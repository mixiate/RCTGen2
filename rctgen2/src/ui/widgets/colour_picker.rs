use crate::ui;
use eframe::egui;
use openrct2::colour::Colour;

pub struct ButtonTextures {
    unpressed: egui::TextureHandle,
    pressed: egui::TextureHandle,
}

fn create_texture(
    egui_context: &egui::Context,
    image: &renderer::image::IndexedImage,
    colour: Colour,
) -> egui::TextureHandle {
    let pixels: Vec<u8> = image
        .as_raw()
        .iter()
        .flat_map(|pixel| {
            let pixel = if (243..=254).contains(pixel) {
                openrct2::colour::COLOUR_RAMPS[colour as usize][usize::from(*pixel) - 243]
            } else {
                *pixel
            };
            renderer::palette::PALETTE[usize::from(pixel)]
        })
        .collect();

    let texture_size = [usize::from(image.width()), usize::from(image.height())];
    let egui_image = egui::ColorImage::from_rgb(texture_size, &pixels);
    egui_context.load_texture(String::new(), egui_image, egui::TextureOptions::default())
}

pub fn create_colour_button_textures(egui_context: &egui::Context) -> Vec<ButtonTextures> {
    use strum::IntoEnumIterator as _;

    let unpressed = include_bytes!("../../../resources/colour_button.png");
    let pressed = include_bytes!("../../../resources/colour_button_pressed.png");

    let unpressed =
        renderer::image::IndexedImage::read(std::io::Cursor::new(unpressed), &renderer::palette::PALETTE_FLAT).unwrap();
    let pressed =
        renderer::image::IndexedImage::read(std::io::Cursor::new(pressed), &renderer::palette::PALETTE_FLAT).unwrap();

    Colour::iter()
        .map(|colour| {
            let unpressed = create_texture(egui_context, &unpressed, colour);
            let pressed = create_texture(egui_context, &pressed, colour);
            ButtonTextures { unpressed, pressed }
        })
        .collect()
}

pub fn colour_button(ui: &mut egui::Ui, textures: &ButtonTextures, pressed: bool) -> egui::Response {
    let texture_handle = if pressed {
        &textures.pressed
    } else {
        &textures.unpressed
    };
    ui.add(egui::Button::image(texture_handle).frame(false))
}

pub struct ColourPicker {
    hovered: bool,
    modal: ui::modals::ColourSelectionModal,
}

impl ColourPicker {
    pub fn new() -> Self {
        ColourPicker {
            hovered: false,
            modal: ui::modals::ColourSelectionModal::new(),
        }
    }

    pub fn button(&mut self, ui: &mut egui::Ui, textures: &[ButtonTextures], selected_colour: &mut Colour) -> bool {
        let response = colour_button(
            ui,
            &textures[*selected_colour as usize],
            self.modal.open || self.hovered,
        );

        self.hovered = response.hovered();
        if response.clicked() {
            self.modal.open = true;
        }

        if let Some(colour) = self.modal.show(ui, textures, *selected_colour) {
            *selected_colour = colour;
            true
        } else {
            false
        }
    }
}
