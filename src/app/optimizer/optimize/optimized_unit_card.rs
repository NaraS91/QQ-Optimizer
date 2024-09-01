use egui::{Rounding, Ui};

use crate::app::{
    assets_loader::{LightConeImageFormat, UnitImageFormat},
    hsr::{relics::RelicPart, units::UnitKind},
    light_cones_store::LightConesStore,
    relics_store::RelicsStore,
    units_store::UnitsStore,
    ASSETS_LOADER,
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
            ui.add(
                ASSETS_LOADER
                    .get_unit_image(self.unit, UnitImageFormat::Icon)
                    //.fit_to_original_size(1.0)
                    .max_width(f32::min(ui.available_width(), 160.)),
            );
            ui.horizontal_wrapped(|ui| {
                for i in 0..unit_data.relics.len() {
                    let part = match i {
                        0 => RelicPart::Head,
                        1 => RelicPart::Hands,
                        2 => RelicPart::Body,
                        3 => RelicPart::Feet,
                        4 => RelicPart::Sphere,
                        5 => RelicPart::Rope,
                        _ => panic!("i cannot be more than 6"),
                    };
                    let mut found = false;
                    let mut scale = 1.0;
                    if let Some(relic_id) = unit_data.relics[i] {
                        if let Some(relic) = relics_store.get_relic(relic_id) {
                            if relic.part == RelicPart::Rope {
                                scale = 0.125
                            } else {
                                scale = 0.25;
                            }

                            ui.add(
                                ASSETS_LOADER
                                    .get_relic_image(relic.set, relic.part)
                                    .fit_to_original_size(scale),
                            );
                            found = true;
                        }
                    }

                    if !found {
                        ui.add(
                            ASSETS_LOADER
                                .get_relic_part_placeholder(part)
                                .fit_to_original_size(scale),
                        );
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                let lc = light_cones_store
                    .get(unit_data.light_cone.unwrap_or(0))
                    .expect("TODO: placeholder");

                let image = if let Some(lc_id) = unit_data.light_cone {
                    light_cones_store
                        .get(lc_id)
                        .expect("equiped lc should be in the store");

                    ASSETS_LOADER.get_light_cone_image(lc.kind, LightConeImageFormat::Resized)
                } else {
                    ASSETS_LOADER.get_light_cone_placeholder(LightConeImageFormat::Resized)
                };

                let response = ui.add(image.fit_to_original_size(1.).rounding(Rounding::same(13.)));

                ui.put(
                    response.rect,
                    ASSETS_LOADER
                        .get_light_cone_frame(LightConeImageFormat::Resized)
                        .fit_to_original_size(1.),
                );

                Self::unit_image(ui, unit_data.team[0]);
                Self::unit_image(ui, unit_data.team[1]);
                Self::unit_image(ui, unit_data.team[2]);
            });
        });
    }

    fn unit_image(ui: &mut Ui, kind_op: Option<UnitKind>) {
        let image = if let Some(kind) = kind_op {
            ASSETS_LOADER.get_unit_image(kind, UnitImageFormat::IconScaled)
        } else {
            ASSETS_LOADER.get_unit_placeholder()
        };

        let response = ui.add(image.fit_to_original_size(1.).rounding(Rounding::same(13.)));

        ui.put(
            response.rect,
            ASSETS_LOADER.get_unit_frame().fit_to_original_size(1.),
        );
    }
}
