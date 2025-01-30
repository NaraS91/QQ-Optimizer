use std::iter::Map;

use egui::{ScrollArea, Vec2};
use enum_map::{Enum, EnumMap};
use strum::IntoEnumIterator;

use super::{
    assets_loader::ELEMENT_SIZE,
    common::{character_avatar::CharacterAvatar, get_section_frame},
    hsr::basics::{Element, Path},
    AppContext, ASSETS_LOADER, COLOR_PALLET,
};

#[derive(Debug, Clone)]
pub struct Units {
    path_buttons: EnumMap<Path, bool>,
    element_buttons: EnumMap<Element, bool>,
}

impl Default for Units {
    fn default() -> Self {
        Self {
            path_buttons: enum_map::enum_map! { _ => false },
            element_buttons: enum_map::enum_map! { _ => false },
        }
    }
}

impl Units {
    pub fn new() -> Units {
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
        let button_width = ELEMENT_SIZE.0 * 0.20;
        let total_buttons_width = (Element::LENGTH + Path::LENGTH) as f32 * button_width;
        let margin = (ui.available_width() - total_buttons_width) / 2.0;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0., 0.);
            ui.add_space(margin);
            for element in Element::iter() {
                if ui
                    .add(
                        egui::ImageButton::new(
                            ASSETS_LOADER
                                .get_element_icon(element)
                                .fit_to_original_size(0.20)
                                .bg_fill(if self.element_buttons[element] {
                                    COLOR_PALLET.button_background()
                                } else {
                                    COLOR_PALLET.unselected_button_background()
                                }),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.element_buttons[element] = !self.element_buttons[element];
                };
            }

            for path in Path::iter() {
                if ui
                    .add(
                        egui::ImageButton::new(
                            ASSETS_LOADER
                                .get_path_icon(path)
                                .fit_to_original_size(0.10)
                                .bg_fill(if self.path_buttons[path] {
                                    COLOR_PALLET.button_background()
                                } else {
                                    COLOR_PALLET.unselected_button_background()
                                }),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.path_buttons[path] = !self.path_buttons[path];
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

                let all_elements = self.element_buttons.iter().all(|(_, b)| !b);
                let all_paths = self.path_buttons.iter().all(|(_, b)| !b);
                let unit_kinds = app_context.units_store.get_all_possessed_unit_kinds(ctx);
                let mut iter = unit_kinds
                    .into_iter()
                    .map(|kind| app_context.units_store.get_unit(ctx, kind))
                    .filter(|op_unit| {
                        if let Some(unit) = op_unit {
                            (self.element_buttons[unit.element] || all_elements)
                                && (self.path_buttons[unit.path] || all_paths)
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter();
                while iter.len() != 0 {
                    ui.horizontal(|ui| {
                        ui.add_space(left_space / 2.0);
                        ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);
                        for _ in 0..cards_num_in_row {
                            if let Some(op_unit) = iter.next() {
                                if let Some(unit) = op_unit {
                                    CharacterAvatar::new(unit, true, false).show_ui(ctx, ui);
                                } else {
                                    ui.label("loading data..."); // TODO: add placeholder
                                }
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
