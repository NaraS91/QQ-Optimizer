use egui::{Rounding, Ui};

use crate::app::{
    hsr::{
        relics::RelicPart,
        units::{UniqueData, UnitKind},
    },
    light_cones_store::{self, LightConesStore},
    relics_store::RelicsStore,
    units_store::UnitsStore,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OptimizedUnit {
    unit: UnitKind,
}

impl OptimizedUnit {
    pub fn new(unit: UnitKind) -> Self {
        OptimizedUnit { unit: unit }
    }

    pub fn show_ui(
        &mut self,
        ui: &mut egui::Ui,
        relics_store: &mut RelicsStore,
        units_store: &mut UnitsStore,
        light_cones_store: &LightConesStore,
    ) {
        let unit_data = units_store
            .get_unique_data(self.unit)
            .expect("optimized unit should be in the store");

        ui.vertical(|ui| {
            let unit_img_path = &format!(
                "file://assets/characters/icon/{}.webp",
                self.unit.file_name()
            );
            ui.add(egui::Image::new(unit_img_path).fit_to_original_size(1.0));
            ui.horizontal(|ui| {
                for i in 0..unit_data.relics.len() {
                    let part = match i {
                        0 => "head",
                        1 => "hand",
                        2 => "body",
                        3 => "feet",
                        4 => "sphere",
                        5 => "rope",
                        _ => panic!("i cannot be more than 6"),
                    };
                    let mut path = "file://assets/relics/".to_owned();
                    let mut scale = 1.0;
                    if let Some(relic_id) = unit_data.relics[i] {
                        if let Some(relic) = relics_store.get_relic(relic_id) {
                            if relic.part == RelicPart::Rope {
                                scale = 0.125
                            } else {
                                scale = 0.25;
                            }

                            path = path + &relic.set.file_name()[..] + "_";
                        }
                    }

                    ui.add(egui::Image::new(path + part + ".webp").fit_to_original_size(scale));
                }
            });
            ui.horizontal(|ui| {
                let lc = light_cones_store
                    .get(unit_data.light_cone.unwrap_or(0))
                    .expect("TODO: placeholder");

                let mut lc_path: String;
                if let Some(lc_id) = unit_data.light_cone {
                    light_cones_store
                        .get(lc_id)
                        .expect("equiped lc should be in the store");

                    lc_path = format!(
                        "file://assets/light_cones/resized/{}.webp",
                        lc.kind.file_name()
                    )
                } else {
                    lc_path = "file://assets/light_cones/resized/placeholder.webp".to_string();
                }

                let response = ui.add(
                    egui::Image::new(lc_path)
                        .fit_to_original_size(1.)
                        .rounding(Rounding::same(13.)),
                );

                ui.put(
                    response.rect,
                    egui::Image::new("file://assets/light_cones/resized/frame.webp")
                        .fit_to_original_size(1.),
                );

                Self::unit_image(ui, unit_data.team[0]);
                Self::unit_image(ui, unit_data.team[1]);
                Self::unit_image(ui, unit_data.team[2]);
            });
        });
    }

    fn unit_image(ui: &mut Ui, kind_op: Option<UnitKind>) {
        let path = if let Some(kind) = kind_op {
            format!(
                "file://assets/characters/icon_scaled/{}.webp",
                kind.file_name()
            )
        } else {
            "file://assets/characters/placeholder.webp".to_string()
        };

        let response = ui.add(
            egui::Image::new(path)
                .fit_to_original_size(1.)
                .rounding(Rounding::same(13.)),
        );

        ui.put(
            response.rect,
            egui::Image::new("file://assets/characters/frame.webp").fit_to_original_size(1.),
        );
    }
}
