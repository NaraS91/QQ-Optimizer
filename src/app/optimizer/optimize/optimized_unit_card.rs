use crate::app::{hsr::units::UnitKind, light_cones_store::{self, LightConesStore}, relics_store::RelicsStore, units_store::UnitsStore};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OptimizedUnit {
    unit: UnitKind,
}

impl OptimizedUnit {
    pub fn new(unit: UnitKind) -> Self {
        OptimizedUnit { unit: unit }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui, relics_store: &mut RelicsStore, units_store: &mut UnitsStore, light_cones_store: &LightConesStore) {
        let unit_data = units_store.get_unit(self.unit).as_ref().expect("optimized unit should be in the store");
        
        ui.vertical(|ui| {
            let unit_img_path = &format!("file://assets/avatar/{}.webp", self.unit.file_name());
            let lc = light_cones_store.get(unit_data.light_cone.unwrap_or(0)).expect("TODO: placeholder");
            ui.add(egui::Image::new(unit_img_path).fit_to_original_size(1.0));
            ui.horizontal(|ui| {
                ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
                ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
                ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
                ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
                ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
                ui.add(egui::Image::new("file://assets/relics/quantum_head.webp").fit_to_original_size(0.3));
            });
            ui.horizontal(|ui| {
                let width = ui.available_width();
                let unit1_path = &format!("file://assets/avatar/{}.webp", unit_data.team[0].expect("TODO: placeholder img").file_name());
                let unit2_path = &format!("file://assets/avatar/{}.webp", unit_data.team[1].expect("TODO: placeholder img").file_name());
                let unit3_path = &format!("file://assets/avatar/{}.webp", unit_data.team[2].expect("TODO: placeholder img").file_name());
                let lc_path = &format!("file://assets/light_cones/full/{}.webp", lc.kind.file_name());
                ui.add(egui::Image::new(lc_path).fit_to_original_size(0.1));
                ui.add(egui::Image::new(unit1_path).fit_to_original_size(0.6));
                ui.add(egui::Image::new(unit2_path).fit_to_original_size(0.6));
                ui.add(egui::Image::new(unit3_path).fit_to_original_size(0.6));
            });
        });
    }
}
