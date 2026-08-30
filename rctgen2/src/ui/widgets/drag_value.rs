use eframe::egui;

fn set_widget_corner_radius(ui: &mut egui::Ui, corner_radius: egui::CornerRadius) {
    let widgets = &mut ui.style_mut().visuals.widgets;
    widgets.inactive.corner_radius = corner_radius;
    widgets.hovered.corner_radius = corner_radius;
    widgets.active.corner_radius = corner_radius;
}

pub struct DragValueSpin<'a, T> {
    value: &'a mut T,
    increment: T,
    range: Option<std::ops::RangeInclusive<T>>,
}

impl<'a, T: eframe::emath::Numeric + std::default::Default> DragValueSpin<'_, T> {
    pub fn new(value: &'a mut T, increment: T) -> DragValueSpin<'a, T> {
        DragValueSpin {
            value,
            increment,
            range: None,
        }
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<T>) -> Self {
        self.range = Some(range);
        self
    }
}

impl<T: eframe::emath::Numeric + std::ops::SubAssign + std::ops::AddAssign> egui::Widget for DragValueSpin<'_, T> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);

            let corner_radius = ui.style().visuals.widgets.active.corner_radius.ne;
            set_widget_corner_radius(
                ui,
                egui::CornerRadius {
                    nw: corner_radius,
                    ne: 0,
                    sw: corner_radius,
                    se: 0,
                },
            );
            let mut response_a = ui.button("-");
            if response_a.clicked() {
                if let Some(range) = &self.range {
                    if *self.value > *range.start() {
                        *self.value -= self.increment;
                    }
                } else {
                    *self.value -= self.increment;
                }
                response_a.mark_changed();
            }

            set_widget_corner_radius(ui, egui::CornerRadius::ZERO);
            let mut drag_value = egui::DragValue::new(self.value).speed(0.01);
            if let Some(range) = &self.range {
                drag_value = drag_value.clamp_existing_to_range(false).range(range.clone());
            }
            let response_b = ui.add_sized(egui::vec2(75.0, 10.0), drag_value);

            set_widget_corner_radius(
                ui,
                egui::CornerRadius {
                    nw: 0,
                    ne: corner_radius,
                    sw: 0,
                    se: corner_radius,
                },
            );
            let mut response_c = ui.button("+");
            if response_c.clicked() {
                if let Some(range) = &self.range {
                    if *self.value < *range.end() {
                        *self.value += self.increment;
                    }
                } else {
                    *self.value += self.increment;
                }
                response_c.mark_changed();
            }

            response_b | response_a | response_c
        })
        .inner
    }
}

pub fn drag_value(
    ui: &mut egui::Ui,
    value: &mut f32,
    label: &str,
    range: Option<core::ops::RangeInclusive<f32>>,
) -> bool {
    let mut changed = false;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        ui.scope(|ui| {
            let corner_radius = ui.style().visuals.widgets.active.corner_radius.ne;
            set_widget_corner_radius(
                ui,
                egui::CornerRadius {
                    nw: 0,
                    ne: corner_radius,
                    sw: 0,
                    se: corner_radius,
                },
            );

            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
            if ui.button("+").clicked() {
                *value += 0.01;
                changed = true;
            }

            set_widget_corner_radius(ui, egui::CornerRadius::ZERO);
            let mut drag_value = egui::DragValue::new(value).speed(0.01);
            if let Some(range) = range {
                drag_value = drag_value.clamp_existing_to_range(false).range(range);
            }
            if ui.add_sized(egui::vec2(75.0, 10.0), drag_value).changed() {
                changed = true;
            }

            set_widget_corner_radius(
                ui,
                egui::CornerRadius {
                    nw: corner_radius,
                    ne: 0,
                    sw: corner_radius,
                    se: 0,
                },
            );
            if ui.button("-").clicked() {
                *value -= 0.01;
                changed = true;
            }
        });

        ui.label(label);
    });
    changed
}
