use crate::ui::widgets::colour_picker;
use eframe::egui;
use openrct2::colour::Colour;

static COLOUR_SELECTION_GRID_COLOURS: [Colour; 54] = [
    Colour::Black,
    Colour::SaturatedRed,
    Colour::DarkOrange,
    Colour::DarkYellow,
    Colour::ForestGreen,
    Colour::SaturatedGreen,
    Colour::DeepWater,
    Colour::DarkBlue,
    Colour::Violet,
    Colour::Grey,
    Colour::BrightRed,
    Colour::LightOrange,
    Colour::Yellow,
    Colour::MossGreen,
    Colour::BrightGreen,
    Colour::DarkWater,
    Colour::LightBlue,
    Colour::BrightPurple,
    Colour::White,
    Colour::LightPink,
    Colour::PastelOrange,
    Colour::BrightYellow,
    Colour::Chartreuse,
    Colour::LimeGreen,
    Colour::LightWater,
    Colour::IcyBlue,
    Colour::Lavender,
    Colour::Umber,
    Colour::Maroon,
    Colour::Sepia,
    Colour::SaturatedBrown,
    Colour::ArmyGreen,
    Colour::HunterGreen,
    Colour::Viridian,
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
    Colour::Beige,
    Colour::CoralPink,
    Colour::Peach,
    Colour::Tan,
    Colour::HoneyDew,
    Colour::Celadon,
    Colour::SeafoamGreen,
    Colour::Periwinkle,
    Colour::PastelPink,
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
        mut position: egui::Pos2,
    ) -> Option<Colour> {
        if !self.open {
            return None;
        }

        let mut selected_colour = None;

        position.x -= f32::from(ui.style().spacing.window_margin.left) + ui.style().visuals.window_stroke.width;
        let modal = egui::containers::modal::Modal::new(self.id)
            .area(egui::containers::Area::new(self.id).movable(false).current_pos(position))
            .backdrop_color(egui::Color32::TRANSPARENT)
            .show(ui.ctx(), |ui| {
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
            });

        if modal.should_close() {
            self.open = false;
        }

        selected_colour
    }
}
