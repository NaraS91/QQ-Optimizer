use crate::app::hsr::units::Unit;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RelicsLevelFilter {
    value: u32
}

impl RelicsLevelFilter {
    pub fn new(value: u32) -> Self {
        RelicsLevelFilter { value: value }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("minimal relic level");
            ui.add(egui::Slider::new(&mut self.value, 0..=15));
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RelicsStatsFilter {
}

impl RelicsStatsFilter {
    pub fn new() -> Self {
        RelicsStatsFilter { }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
        });
    }
}