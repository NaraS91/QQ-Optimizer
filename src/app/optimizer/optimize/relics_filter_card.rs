use egui::Ui;

use crate::app::{
    hsr::{
        basics::Element,
        relics::{RelicPart, RelicStat},
        units::Unit,
    },
    ColorPallet,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RelicsLevelFilter {
    value: u32,
}

impl RelicsLevelFilter {
    pub fn new(value: u32) -> Self {
        RelicsLevelFilter { value: value }
    }

    pub fn get_min_level(&self) -> u32 {
        self.value
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
    pub head: u32,
    pub hands: u32,
    pub body: u32,
    pub feet: u32,
    pub sphere: u32,
    pub rope: u32,
}

const DEFAULT_VALUES: [[Option<RelicStat>; 7]; 4] = [
    [
        Some(RelicStat::HpP),
        Some(RelicStat::AtkP),
        Some(RelicStat::DefP),
        Some(RelicStat::Cr),
        Some(RelicStat::Cd),
        Some(RelicStat::OH),
        Some(RelicStat::EHR),
    ],
    [
        Some(RelicStat::HpP),
        Some(RelicStat::AtkP),
        Some(RelicStat::DefP),
        Some(RelicStat::Spd),
        None,
        None,
        None,
    ],
    [
        Some(RelicStat::HpP),
        Some(RelicStat::AtkP),
        Some(RelicStat::DefP),
        Some(RelicStat::ElementalDmg(Element::Quantum)),
        None,
        None,
        None,
    ],
    [
        Some(RelicStat::HpP),
        Some(RelicStat::AtkP),
        Some(RelicStat::DefP),
        Some(RelicStat::BE),
        Some(RelicStat::Err),
        None,
        None,
    ],
];

impl RelicsStatsFilter {
    pub fn new() -> Self {
        RelicsStatsFilter {
            head: 1 << RelicStat::Hp.unique_id(),
            hands: 1 << RelicStat::Atk.unique_id(),
            body: Self::get_mask(0),
            feet: Self::get_mask(1),
            sphere: Self::get_mask(2),
            rope: Self::get_mask(3),
        }
    }

    fn get_mask(i: usize) -> u32 {
        let mut result = 0;
        for relic_stat_op in &DEFAULT_VALUES[i] {
            if let Some(relic_stat) = relic_stat_op {
                result = result | 1 << relic_stat.unique_id();
            }
        }
        result
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Image::new("file://assets/relics/head.webp"));
                ui.label("Hp");
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Image::new("file://assets/relics/hand.webp"));
                ui.label("Atk");
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Image::new("file://assets/relics/body.webp"));
                Self::add_labels(ui, 0, &mut self.body);
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Image::new("file://assets/relics/feet.webp"));
                Self::add_labels(ui, 1, &mut self.feet);
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Image::new("file://assets/relics/sphere.webp"));
                Self::add_labels(ui, 2, &mut self.sphere);
            });

            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Image::new("file://assets/relics/rope.webp"));
                Self::add_labels(ui, 3, &mut self.rope);
            });
        });
    }

    fn add_labels(ui: &mut Ui, id: usize, value: &mut u32) {
        for stat_op in DEFAULT_VALUES[id] {
            if let Some(stat) = stat_op {
                if ui
                    .selectable_label(*value & 1 << stat.unique_id() != 0, stat.to_str())
                    .clicked()
                {
                    *value = *value ^ 1 << stat.unique_id();
                }
            }
        }
    }
}
