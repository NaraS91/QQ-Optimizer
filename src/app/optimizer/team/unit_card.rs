

use crate::app::assets_loader::UnitImageFormat;
use crate::app::ASSETS_LOADER;
use crate::app::{relics_store::RelicsStore, units_store::UnitsStore};
use egui::vec2;
use egui::{CentralPanel, ComboBox, Frame, Label, Margin};

use super::super::super::hsr::units::UnitKind;
use strum::IntoEnumIterator;

pub struct UnitCard {
    id: String,
    selector: UnitSelector,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct UnitSelector {
    selected_id: usize,
    selected_unit: Option<UnitKind>,
}

impl UnitCard {
    pub fn new(id: String, selected_id: usize) -> UnitCard {
        UnitCard {
            selector: UnitSelector::new(selected_id),
            id: id,
        }
    }

    pub fn show_ui(
        &mut self,
        ui: &mut egui::Ui,
        all_units: &Vec<UnitKind>,
        main_unit: UnitKind,
        relics_store: &RelicsStore,
        units_store: &mut UnitsStore,
    ) {
        self.selector.show_ui(
            ui,
            &self.id,
            all_units,
            main_unit,
            relics_store,
            units_store,
        );
    }
}

impl UnitSelector {
    pub fn new(selected_id: usize) -> UnitSelector {
        UnitSelector {
            selected_id,
            selected_unit: None,
        }
    }

    pub fn show_ui(
        &mut self,
        ui: &mut egui::Ui,
        id: &String,
        all_units: &Vec<UnitKind>,
        main_unit: UnitKind,
        _relics_store: &RelicsStore,
        units_store: &mut UnitsStore,
    ) {
        let team = units_store
            .get_unique_data(main_unit)
            .as_ref()
            .expect("main unit should be in the store")
            .team;
        let selected_unit = team[self.selected_id];
        let other_units = team
            .iter()
            .enumerate()
            .filter_map(|(i, op_kind)| {
                if i != self.selected_id {
                    Some(*op_kind)
                } else {
                    None
                }
            })
            .collect();
        ui.vertical(|ui| {
            ComboBox::from_id_source(id)
                .width(ui.available_width())
                .selected_text(
                    selected_unit.map_or_else(|| "None".to_owned(), |unit| unit.to_string()),
                )
                .show_ui(ui, |ui| {
                    self.selectable_values(ui, all_units, main_unit, other_units, units_store)
                });
            if selected_unit.is_none() {
                return;
            }

            let selected_unit = selected_unit.expect("checked above");
            let unit_info = units_store
                .get_unique_data(selected_unit)
                .expect("all selectable units should be in the store");

            let frame = Frame::default().inner_margin(Margin::symmetric(10., 10.));
            CentralPanel::default().frame(frame).show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        ASSETS_LOADER
                            .get_unit_image(selected_unit, UnitImageFormat::Icon)
                            .fit_to_original_size(1.0),
                    );

                    ui.vertical_centered(|ui| {
                        let y = ui.available_height() / 4.;
                        let x = ui.available_width();
                        ui.add_sized(vec2(x, y), Label::new(selected_unit.to_string()));
                        ui.columns(2, |columns| {
                            columns[0].add_sized(
                                vec2(x / 2.0, y),
                                Label::new(format!(
                                    "Lv. {}/{}",
                                    unit_info.level,
                                    unit_info.max_level()
                                )),
                            );
                            columns[1].add_sized(
                                vec2(x / 2.0, y),
                                Label::new(format!("E{}", unit_info.eidolon)),
                            );
                        });
                        ui.columns(3, |columns| {
                            columns[0].add_sized(
                                vec2(x / 3.0, y),
                                Label::new(unit_info.ultimate_level.to_string()),
                            );
                            columns[1].add_sized(
                                vec2(x / 3.0, y),
                                Label::new(unit_info.skill_level.to_string()),
                            );
                            columns[2].add_sized(
                                vec2(x / 3.0, y),
                                Label::new(unit_info.talent_level.to_string()),
                            );
                        });
                        ui.columns(3, |columns| {
                            columns[0].add_sized(
                                vec2(x / 3.0, y),
                                ASSETS_LOADER
                                    .get_unit_image(selected_unit, UnitImageFormat::Icon)
                                    .fit_to_original_size(0.1),
                            );
                            columns[1].add_sized(
                                vec2(x / 3.0, y),
                                ASSETS_LOADER
                                    .get_unit_image(selected_unit, UnitImageFormat::Icon)
                                    .fit_to_original_size(0.1),
                            );
                            columns[2].add_sized(
                                vec2(x / 3.0, y),
                                ASSETS_LOADER
                                    .get_unit_image(selected_unit, UnitImageFormat::Icon)
                                    .fit_to_original_size(0.1),
                            );
                        });
                    })
                })
            });
        });
    }

    fn selectable_values(
        &mut self,
        ui: &mut egui::Ui,
        all_units: &Vec<UnitKind>,
        main_unit: UnitKind,
        other_team_mates: Vec<Option<UnitKind>>,
        units_store: &mut UnitsStore,
    ) {
        for unit in all_units {
            if *unit != main_unit {
                if other_team_mates.contains(&Some(*unit)) {
                    ui.label(unit.to_string());
                } else {
                    if ui.button(unit.to_string()).clicked() {
                        units_store.update_team_mate(main_unit, Some(*unit), self.selected_id);
                    };
                    //ui.selectable_value(&mut self.selected_unit, Some(unit), unit.to_string());
                };
            }
        }
    }
}
