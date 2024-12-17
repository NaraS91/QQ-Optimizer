use crate::app::assets_loader::UnitImageFormat;
use crate::app::hsr::units::{
    BuffScaling, Modifier, ModifierData, ModifierOrDOT, SkillData, SkillKind, Source, UniqueData,
    Unit,
};
use crate::app::ASSETS_LOADER;
use crate::app::{relics_store::RelicsStore, units_store::UnitsStore};
use egui::{vec2, ScrollArea, Ui};
use egui::{CentralPanel, ComboBox, Frame, Label, Margin};
use itertools::Itertools;

use super::super::super::hsr::units::UnitKind;
use strum::IntoEnumIterator;

pub struct UnitCard {
    id: String,
    selector: UnitSelector,
    unit_buffs: Option<SupportingUnitBuffs>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct UnitSelector {
    id_in_team: usize,
}

struct SupportingUnitBuffs {
    unit: UnitKind,
    skill_modifiers: Vec<(SkillData, Modifier)>,
    eidolon_modifier: Vec<(u8, Modifier)>,
}

impl UnitCard {
    pub fn new(id: String, id_in_team: usize) -> UnitCard {
        UnitCard {
            selector: UnitSelector::new(id_in_team),
            id: id,
            unit_buffs: None,
        }
    }

    pub fn show_ui(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        all_units: &Vec<UnitKind>,
        main_unit: UnitKind,
        relics_store: &RelicsStore,
        units_store: &mut UnitsStore,
    ) {
        ui.vertical(|ui| {
            self.selector.show_ui(
                ui,
                &self.id,
                all_units,
                main_unit,
                relics_store,
                units_store,
            );

            if let Some(selected_unit_kind) = units_store
                .get_unique_data(main_unit)
                .as_ref()
                .expect("main unit should be in the store")
                .team[self.selector.id_in_team]
            {
                if self.unit_buffs.is_none()
                    || self.unit_buffs.as_ref().unwrap().unit != selected_unit_kind
                {
                    if let Some(selected_unit) = units_store.get_unit(ctx, selected_unit_kind) {
                        self.unit_buffs = Some(SupportingUnitBuffs::new(&selected_unit))
                    }
                }
                self.show_unit_details(ui, selected_unit_kind, units_store)
            }
        });
    }

    pub fn get_skill_modifiers(&self) -> Vec<&(SkillData, Modifier)> {
        self.unit_buffs
            .as_ref()
            .map_or(Vec::new(), |vs| vs.skill_modifiers.iter().collect())
    }

    pub fn get_eidolon_modifiers(&self) -> Vec<&(u8, Modifier)> {
        self.unit_buffs
            .as_ref()
            .map_or(Vec::new(), |vs| vs.eidolon_modifier.iter().collect())
    }

    pub fn get_active_modifiers(&self) -> Vec<Modifier> {
        self.unit_buffs
            .as_ref()
            .map_or(Vec::new(), |buffs| buffs.get_active_modifiers())
    }

    fn show_unit_details(
        &self,
        ui: &mut Ui,
        selected_unit: UnitKind,
        units_store: &mut UnitsStore,
    ) {
        let unit_info = units_store
            .get_unique_data(selected_unit)
            .expect("all selectable units should be in the store");

        let frame = Frame::default().inner_margin(Margin::symmetric(10., 10.));
        CentralPanel::default().frame(frame).show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
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
                });
                if let Some(unit_buffs) = self.unit_buffs.as_ref() {
                    unit_buffs.show_ui(ui);
                };
            })
        });
    }
}

impl UnitSelector {
    pub fn new(id_in_team: usize) -> UnitSelector {
        UnitSelector {
            id_in_team: id_in_team,
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
        let selected_unit = team[self.id_in_team];
        let other_units = team
            .iter()
            .enumerate()
            .filter_map(|(i, op_kind)| {
                if i != self.id_in_team {
                    Some(*op_kind)
                } else {
                    None
                }
            })
            .collect();

        ComboBox::from_id_source(id)
            .width(ui.available_width())
            .selected_text(selected_unit.map_or_else(|| "None".to_owned(), |unit| unit.to_string()))
            .show_ui(ui, |ui| {
                self.selectable_values(ui, all_units, main_unit, other_units, units_store)
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
                        units_store.update_team_mate(main_unit, Some(*unit), self.id_in_team);
                    };
                };
            }
        }
    }
}

impl SupportingUnitBuffs {
    pub fn new(unit: &Unit) -> SupportingUnitBuffs {
        let modifiers_and_dots: Vec<ModifierOrDOT> = unit.kind.get_modifiers()(unit);
        SupportingUnitBuffs {
            unit: unit.kind,
            skill_modifiers: modifiers_and_dots
                .iter()
                .filter_map(|m_or_dot| match m_or_dot {
                    ModifierOrDOT::DOT(_) => None,
                    ModifierOrDOT::Modifier(modifier) => {
                        let skill_kind_op = SkillKind::try_from_source(&modifier.source.1);
                        if skill_kind_op.is_none() {
                            return None;
                        }

                        let skill_kind = skill_kind_op.unwrap();
                        let skill_data = unit.skills.iter().find(|skill_data| {
                            skill_data.kind.is_some_and(|kind| kind == skill_kind)
                        });
                        Some((
                            skill_data
                                .to_owned()
                                .expect(&format!("expected: {:?} to exist", skill_kind))
                                .clone(),
                            modifier.to_owned(),
                        ))
                    }
                })
                .collect_vec(),
            eidolon_modifier: modifiers_and_dots
                .into_iter()
                .filter_map(|m_or_dot| match m_or_dot {
                    ModifierOrDOT::DOT(_) => None,
                    ModifierOrDOT::Modifier(modifier) => match modifier.source.1 {
                        Source::Eidolon(i) => Some((i, modifier)),
                        _ => None,
                    },
                })
                .collect_vec(),
        }
    }

    pub fn show_ui(&self, ui: &mut egui::Ui) {
        self.skill_modifiers.iter().for_each(|modifier| {
            self.show_header(ui, modifier);
            modifier
                .1
                .data
                .iter()
                .for_each(|modifier_data| self.show_modifier_data(ui, modifier_data))
        });
    }

    pub fn get_active_modifiers(&self) -> Vec<Modifier> {
        self.skill_modifiers
            .iter()
            .map(|p| p.1.clone())
            .chain(self.eidolon_modifier.iter().map(|t| t.1.clone()))
            .collect()
    }

    fn show_header(&self, ui: &mut egui::Ui, skill_modifier: &(SkillData, Modifier)) {
        let skill = &skill_modifier.0;
        ui.horizontal(|ui| {
            ui.add(
                ASSETS_LOADER
                    .get_unit_ability(self.unit, skill)
                    .fit_to_original_size(1.0),
            );
            ui.label(skill.name.to_owned());
        });
    }

    fn show_modifier_data(&self, ui: &mut egui::Ui, modifier_data: &ModifierData) {
        ui.vertical(|ui| {
            ui.label(format!(
                "+X{} {}",
                if modifier_data.scaling == BuffScaling::Multiplicative {
                    "%"
                } else {
                    ""
                },
                modifier_data.stat.to_string()
            ));
        });
    }
}
