use crate::app::COLOR_PALLET;
use egui::Margin;

pub mod character_avatar;
pub mod light_cone_avatar;
pub mod relic_avatar;

pub fn get_section_frame() -> egui::containers::Frame {
    egui::containers::Frame::default()
        .fill(COLOR_PALLET.section())
        .rounding(egui::Rounding::same(8.0))
        .inner_margin(Margin::same(20.0))
}
