use egui::Margin;

use crate::app::{hsr::units::UnitKind, relics_store::RelicsStore, units_store::UnitsStore, COLOR_PALLET};
mod unit_card;
use unit_card::UnitCard;

pub struct Team {
    unit_cards: [Box<UnitCard>; 3],
    main_unit: UnitKind
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
                Box::new(UnitCard::new("unit_3_card".to_owned(), 2))
            ],
            main_unit: main_unit
        }
    }

    pub fn change_main_unit(&mut self, main_unit: UnitKind) {
        self.main_unit = main_unit;
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui, relics_store: &RelicsStore, units_store: &mut UnitsStore){
        let main_unit = units_store.get_unit(self.main_unit).as_ref().expect("store should always have the main unit");
        let all_kinds = units_store.get_all_kinds();
        
        let width = ui.available_size().x;
        let card_width = width * 0.25;
        let card_spacing = width * 0.1;

        let left_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .outer_margin(Margin{left: 0., right: 0., top: 10., bottom: 10.});

        egui::containers::SidePanel::left("unit_1")
            .frame(left_card)
            .resizable(false)
            .exact_width(card_width)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                self.unit_cards[0].show_ui(ui, &all_kinds, self.main_unit, relics_store, units_store)
            });
        
        let right_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .outer_margin(Margin{left: 0., right: 0., top: 10., bottom: 10.});

        egui::containers::SidePanel::right("unit_3")
            .frame(right_card)
            .resizable(false)
            .exact_width(card_width)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                self.unit_cards[2].show_ui(ui, &all_kinds, self.main_unit, relics_store, units_store)
            });

        let center_card = egui::containers::Frame::default()
            .fill(COLOR_PALLET.card())
            .rounding(egui::Rounding::same(5.0))
            .outer_margin(Margin{left: card_spacing, right: card_spacing, top: 10., bottom: 10.});

        egui::containers::CentralPanel::default()
            .frame(center_card)
            .show_inside(ui, |ui| {
                self.unit_cards[1].show_ui(ui, &all_kinds, self.main_unit, relics_store, units_store)
            });
    }
}