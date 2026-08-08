use crate::widgets::colour_picker;
use eframe::egui;
use openrct2::colour::Colour;

static COLOUR_SELECTION_GRID_COLOURS: [Colour; 54] = [
    Colour::Black,
    Colour::SaturatedRed,
    Colour::DarkOrange,
    Colour::DarkYellow,
    Colour::GrassGreenDark,
    Colour::SaturatedGreen,
    Colour::AquaDark,
    Colour::DarkBlue,
    Colour::SaturatedPurpleDark,
    Colour::Grey,
    Colour::BrightRed,
    Colour::LightOrange,
    Colour::Yellow,
    Colour::MossGreen,
    Colour::BrightGreen,
    Colour::Teal,
    Colour::LightBlue,
    Colour::BrightPurple,
    Colour::White,
    Colour::LightPink,
    Colour::OrangeLight,
    Colour::BrightYellow,
    Colour::GrassGreenLight,
    Colour::SaturatedGreenLight,
    Colour::Aquamarine,
    Colour::IcyBlue,
    Colour::SaturatedPurpleLight,
    Colour::DullBrownDark,
    Colour::BordeauxRedDark,
    Colour::TanDark,
    Colour::SaturatedBrown,
    Colour::DarkOliveDark,
    Colour::OliveDark,
    Colour::DullGreenDark,
    Colour::DarkPurple,
    Colour::DarkPink,
    Colour::DarkBrown,
    Colour::BordeauxRed,
    Colour::SalmonPink,
    Colour::LightBrown,
    Colour::DarkOliveGreen,
    Colour::OliveGreen,
    Colour::DarkGreen,
    Colour::LightPurple,
    Colour::BrightPink,
    Colour::DullBrownLight,
    Colour::BordeauxRedLight,
    Colour::TanLight,
    Colour::SaturatedBrownLight,
    Colour::DarkOliveLight,
    Colour::OliveLight,
    Colour::DullGreenLight,
    Colour::DullPurpleLight,
    Colour::MagentaLight,
];

pub struct ColourSelectionModal {
    pub open: bool,
    id: egui::Id,
    hovered: Option<Colour>,
}

impl ColourSelectionModal {
    pub fn new() -> Self {
        ColourSelectionModal {
            open: false,
            id: egui::Id::new("colour select modal"),
            hovered: None,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        textures: &[colour_picker::ButtonTextures],
        current_colour: Colour,
    ) -> Option<Colour> {
        if !self.open {
            return None;
        }

        let mut selected_colour = None;

        let modal = egui::containers::modal::Modal::new(self.id).backdrop_color(egui::Color32::TRANSPARENT).show(
            ui.ctx(),
            |ui| {
                let mut hovered = false;
                egui::Grid::new("Colour selection modal grid")
                    .spacing([0.0, 0.0])
                    .min_col_width(0.0)
                    .show(ui, |ui| {
                        for (i, colour) in COLOUR_SELECTION_GRID_COLOURS.iter().enumerate() {
                            let pressed =
                                self.hovered.map(|x| x == *colour).unwrap_or_default() || current_colour == *colour;
                            let response = colour_picker::colour_button(ui, &textures[*colour as usize], pressed);
                            if response.hovered() {
                                self.hovered = Some(*colour);
                                hovered = true;
                            }
                            if response.clicked() {
                                self.open = false;
                                selected_colour = Some(*colour);
                            }

                            if i != 0 && (i + 1) % 9 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                if !hovered {
                    self.hovered = None;
                }
            },
        );

        if modal.should_close() {
            self.open = false;
        }

        selected_colour
    }
}
