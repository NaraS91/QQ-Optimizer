use std::iter::Map;

use egui::{ScrollArea, Vec2};
use enum_map::{Enum, EnumMap};
use strum::IntoEnumIterator;

use super::{
    assets_loader::RELIC_PIECE_SIZE,
    common::{character_avatar::CharacterAvatar, get_section_frame, relic_avatar::RelicAvatar},
    hsr::relics::{CavernSet, PlanarSet, RelicPart, RelicSet},
    AppContext, ASSETS_LOADER, COLOR_PALLET,
};

#[derive(Debug, Clone)]
pub struct RelicsGallery {
    relic_parts: EnumMap<RelicPart, bool>,
    planar_sets: EnumMap<PlanarSet, bool>,
    cavern_sets: EnumMap<CavernSet, bool>,
}

impl Default for RelicsGallery {
    fn default() -> Self {
        Self {
            relic_parts: enum_map::enum_map! { _ => false },
            planar_sets: enum_map::enum_map! { _ => false },
            cavern_sets: enum_map::enum_map! { _ => false },
        }
    }
}

impl RelicsGallery {
    pub fn new() -> RelicsGallery {
        Default::default()
    }
    pub fn show_ui(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        app_context: &mut AppContext,
    ) {
        egui::CentralPanel::default()
            .frame(get_section_frame())
            .show_inside(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0., 10.);
                self.show_filter(ctx, ui, app_context);
                self.show_gallery(ctx, ui, app_context);
            });
    }

    fn show_filter(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        app_context: &mut AppContext,
    ) {
        let button_width = RELIC_PIECE_SIZE.0 * 0.50;
        let total_buttons_width =
            (CavernSet::LENGTH + PlanarSet::LENGTH + RelicPart::LENGTH) as f32 * button_width;
        // let margin = (ui.available_width() - total_buttons_width) / 2.0;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0., 0.);
            // ui.add_space(margin);
            for relic_part in RelicPart::iter() {
                if ui
                    .add(
                        egui::ImageButton::new(
                            ASSETS_LOADER
                                .get_relic_part_placeholder(relic_part)
                                .fit_to_original_size(0.50)
                                .bg_fill(if self.relic_parts[relic_part] {
                                    COLOR_PALLET.button_background()
                                } else {
                                    COLOR_PALLET.unselected_button_background()
                                }),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.relic_parts[relic_part] = !self.relic_parts[relic_part];
                };
            }

            for cavern_set in CavernSet::iter() {
                if ui
                    .add(
                        egui::ImageButton::new(
                            ASSETS_LOADER
                                .get_relic_set_placeholder(RelicSet::Cavern(cavern_set))
                                .fit_to_original_size(0.50)
                                .bg_fill(if self.cavern_sets[cavern_set] {
                                    COLOR_PALLET.button_background()
                                } else {
                                    COLOR_PALLET.unselected_button_background()
                                }),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.cavern_sets[cavern_set] = !self.cavern_sets[cavern_set];
                };
            }

            for planar_set in PlanarSet::iter() {
                if ui
                    .add(
                        egui::ImageButton::new(
                            ASSETS_LOADER
                                .get_relic_set_placeholder(RelicSet::Planar(planar_set))
                                .fit_to_original_size(0.50)
                                .bg_fill(if self.planar_sets[planar_set] {
                                    COLOR_PALLET.button_background()
                                } else {
                                    COLOR_PALLET.unselected_button_background()
                                }),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.planar_sets[planar_set] = !self.planar_sets[planar_set];
                };
            }
        });
    }

    fn show_gallery(&self, ctx: &egui::Context, ui: &mut egui::Ui, app_context: &mut AppContext) {
        let available_width = ui.available_width();
        ui.vertical_centered(|ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let spacing = 20.;
                let card_width = CharacterAvatar::get_default_size().x;
                let cards_num_in_row = (available_width / (card_width + spacing)) as usize;
                let left_space =
                    available_width - cards_num_in_row as f32 * (card_width + spacing) + spacing;

                let relics = app_context.relics_store.get_all_as_ref();
                let all_parts = self.relic_parts.iter().all(|(_, b)| !b);
                let all_planar_sets = self.planar_sets.iter().all(|(_, b)| !b);
                let all_cavern_sets = self.cavern_sets.iter().all(|(_, b)| !b);

                let mut iter = relics
                    .into_iter()
                    .filter(|relic| {
                        (self.relic_parts[relic.part] || all_parts)
                            && ((all_cavern_sets && all_planar_sets)
                                || match relic.set {
                                    RelicSet::Cavern(set) => self.cavern_sets[set],
                                    RelicSet::Planar(set) => self.planar_sets[set],
                                })
                    })
                    .collect::<Vec<_>>()
                    .into_iter();
                while iter.len() != 0 {
                    ui.horizontal(|ui| {
                        ui.add_space(left_space / 2.0);
                        ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);
                        for _ in 0..cards_num_in_row {
                            if let Some(relic) = iter.next() {
                                RelicAvatar::new(*relic, true, false).show_ui(ctx, ui);
                            } else {
                                break;
                            }
                        }
                        ui.add_space(left_space / 2.0);
                    });
                }
            });
        });
    }
}
