use egui::Margin;

use crate::app::{light_cones_store::{self, LightConesStore}, relics_store::RelicsStore, units_store::UnitsStore, COLOR_PALLET};

use self::{optimized_unit_card::OptimizedUnit, relics_filter_card::{RelicsLevelFilter, RelicsStatsFilter}, requirements_card::Requirements};

mod optimized_unit_card;
mod relics_filter_card;
mod requirements_card;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Optimize {
    unit_card: OptimizedUnit,
    relics_level_filter: RelicsLevelFilter,
    relics_stat_filter: RelicsStatsFilter,
    requirements: Requirements,
}

impl Default for Optimize {
    fn default() -> Self {
        Optimize {
            unit_card: OptimizedUnit::new(
                crate::app::hsr::units::UnitKind::Qingque
            ),
            relics_level_filter: RelicsLevelFilter::new(0),
            relics_stat_filter: RelicsStatsFilter::new(),
            requirements: Requirements::new()
        }
    }
}

impl Optimize {
    pub fn new() -> Optimize {
        Optimize::default()
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui, relics_store: &mut RelicsStore, units_store: &mut UnitsStore, light_cones_store: &LightConesStore){
        let width = ui.available_size().x;
        let card_width = width * 0.32;
        let card_spacing = width * 0.02;

        let top_card = egui::containers::Frame::default()
            .outer_margin(Margin{left: 0., right: 0., top: 10., bottom: 0.});

        egui::containers::TopBottomPanel::top("top_cards")
            .frame(top_card)
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| {

                let unit_card = egui::containers::Frame::default()
                    .fill(COLOR_PALLET.card())
                    .rounding(egui::Rounding::same(5.0))
                    .inner_margin(Margin::symmetric(10., 10.))
                    .outer_margin(Margin{left: 0., right: 0., top: 0., bottom: 10.});
                
                egui::containers::SidePanel::left("optimized_unit")
                    .frame(unit_card)
                    .resizable(false)
                    .exact_width(card_width)
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        self.unit_card.show_ui(ui, relics_store, units_store, light_cones_store)
                    });

                let relics_reqs = egui::containers::Frame::default()
                    .fill(COLOR_PALLET.card())
                    .rounding(egui::Rounding::same(5.0))
                    .inner_margin(Margin::symmetric(10., 10.))
                    .outer_margin(Margin{left: 0., right: 0., top: 0., bottom: 10.});
                
                egui::containers::SidePanel::right("relics_requiremetns")
                    .frame(relics_reqs)
                    .resizable(false)
                    .exact_width(card_width)
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        self.requirements.show_ui(ui)
                    });

                let center_panel = egui::containers::Frame::default()
                    .outer_margin(Margin{left: card_spacing, right: card_spacing, top: 0., bottom: 10.});
        
                egui::containers::CentralPanel::default()
                    .frame(center_panel)
                    .show_inside(ui, |ui| {
                        let level_filter_card = egui::containers::Frame::default()
                            .fill(COLOR_PALLET.card())
                            .rounding(egui::Rounding::same(5.0))
                            .inner_margin(Margin::symmetric(10., 10.))
                            .outer_margin(Margin{left: 0., right: 0., top: 0., bottom: 10.});

                        egui::containers::TopBottomPanel::top("relics_levels_panel")
                            .frame(level_filter_card)
                            .show_inside(ui, |ui| {
                                self.relics_level_filter.show_ui(ui)
                            });

                        let stats_filter_card = egui::containers::Frame::default()
                            .fill(COLOR_PALLET.card())
                            .inner_margin(Margin::symmetric(10., 10.))
                            .rounding(egui::Rounding::same(5.0));

                        egui::containers::CentralPanel::default()
                            .frame(stats_filter_card)
                            .show_inside(ui, |ui| {
                                self.relics_stat_filter.show_ui(ui)
                            });
                    });
            });
    }
}