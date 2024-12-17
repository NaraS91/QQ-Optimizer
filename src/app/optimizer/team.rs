use std::{borrow::Borrow, io::Empty};

use egui::{Context, Margin};

use crate::app::{
    hsr::units::{Modifier, Unit, UnitKind},
    relics_store::RelicsStore,
    units_store::UnitsStore,
    COLOR_PALLET,
};
mod unit_card;
use unit_card::UnitCard;

pub struct Team {
    unit_cards: [Box<UnitCard>; 3],
    main_unit: UnitKind,
    supporting_units: [Box<Option<Unit>>; 3],
}

impl Default for Team {
    fn default() -> Self {
        Team::new(UnitKind::Qingque)
    }
}

impl Team {
    pub fn new(main_unit: UnitKind) -> Team {
        Team {
            unit_cards: [
                Box::new(UnitCard::new("unit_1_card".to_owned(), 0)),
                Box::new(UnitCard::new("unit_2_card".to_owned(), 1)),
                Box::new(UnitCard::new("unit_3_card".to_owned(), 2)),
            ],
            main_unit: main_unit,
            supporting_units: [Box::new(None), Box::new(None), Box::new(None)],
        }
    }

    pub fn change_main_unit(&mut self, main_unit: UnitKind) {
        self.main_unit = main_unit;
    }

    pub fn show_ui(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        relics_store: &RelicsStore,
        units_store: &mut UnitsStore,
    ) {
        // let _main_unit = units_store
        //     .get_unit(ctx, self.main_unit)
        //     .as_ref();
        let all_kinds = units_store.get_all_kinds();

        let width = ui.available_size().x;
        let card_width = width * 0.25;
        let card_spacing = width * 0.1;

        let left_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .outer_margin(Margin {
                left: 0.,
                right: 0.,
                top: 10.,
                bottom: 10.,
            });

        egui::containers::SidePanel::left("unit_1")
            .frame(left_card)
            .resizable(false)
            .exact_width(card_width)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                self.unit_cards[0].show_ui(
                    ctx,
                    ui,
                    &all_kinds,
                    self.main_unit,
                    relics_store,
                    units_store,
                )
            });

        let right_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .outer_margin(Margin {
                left: 0.,
                right: 0.,
                top: 10.,
                bottom: 10.,
            });

        egui::containers::SidePanel::right("unit_3")
            .frame(right_card)
            .resizable(false)
            .exact_width(card_width)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                self.unit_cards[2].show_ui(
                    ctx,
                    ui,
                    &all_kinds,
                    self.main_unit,
                    relics_store,
                    units_store,
                )
            });

        let center_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .outer_margin(Margin {
                left: card_spacing,
                right: card_spacing,
                top: 10.,
                bottom: 10.,
            });

        egui::containers::CentralPanel::default()
            .frame(center_card)
            .show_inside(ui, |ui| {
                self.unit_cards[1].show_ui(
                    ctx,
                    ui,
                    &all_kinds,
                    self.main_unit,
                    relics_store,
                    units_store,
                )
            });

        self.update_supporting_units(ctx, units_store);
    }

    pub fn get_active_modifiers(&self) -> Vec<(Unit, Modifier)> {
        let mut modifiers: Vec<(Unit, Modifier)> = Vec::new();
        for i in 0..3 {
            let unit_modifiers = self.unit_cards[i].get_active_modifiers();
            if unit_modifiers.len() > 0 && self.supporting_units[i].is_some() {
                modifiers.extend(
                    unit_modifiers
                        .iter()
                        .map(|m| (self.supporting_units[i].clone().unwrap(), m.clone())),
                );
            }
        }
        modifiers
    }

    fn update_supporting_units(&mut self, ctx: &Context, units_store: &mut UnitsStore) {
        let mut update_buffs = false;
        let team = units_store
            .get_unique_data(self.main_unit)
            .expect("main unit should be here zzzzz")
            .team;
        for i in 0..3 {
            if team[i] != Option::as_ref(&*self.supporting_units[i]).map(|u| u.kind) {
                update_buffs = true;
                if let Some(unit_kind) = team[i] {
                    self.supporting_units[i] = Box::new(units_store.get_unit(ctx, unit_kind));
                } else {
                    self.supporting_units[i] = Box::new(None);
                }
            }
        }

        if update_buffs {
            self.update_buffs();
        }
    }

    fn update_buffs(&self) {
        //TODO: update buffs as units have changes, maybe move buffs to not async place... idk think about it hf
    }
}
