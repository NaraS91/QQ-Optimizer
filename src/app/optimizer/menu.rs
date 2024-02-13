use egui::{Rounding, RichText, InnerResponse};
use crate::app::COLOR_PALLET;
use super::{Optimizer, Menu};

pub fn new(ui: &mut egui::Ui, optimizer_sate: &Box<Optimizer>) -> InnerResponse<Option<Menu>> {
    let mini_menu_frame = egui::containers::Frame::default()
        .fill(COLOR_PALLET.card())
        .rounding(Rounding::same(5.0));

    egui::TopBottomPanel::top("mini_menu")
        .frame(mini_menu_frame)
        .show_inside(ui, |ui| {
            let team_button = egui::Button::new(RichText::new("Team")
                .color(if matches!(optimizer_sate.menu_state, Menu::Team) {
                            COLOR_PALLET.highlighted_text
                        } else {
                            COLOR_PALLET.text
                        }
                    )
            );

            let optimize_button = egui::Button::new(RichText::new("Optimize")
                    .color(if matches!(optimizer_sate.menu_state, Menu::Optimize) {
                            COLOR_PALLET.highlighted_text
                        } else {
                            COLOR_PALLET.text
                        }
                    )
            );
            ui.horizontal(|ui| {
                let team_button_r = ui.add(team_button);
                let optimize_button_r = ui.add(optimize_button);
                if team_button_r.clicked() {
                    return Some(Menu::Team);
                } else if optimize_button_r.clicked() {
                    return Some(Menu::Optimize);
                }
                None
            }).inner
        })
}