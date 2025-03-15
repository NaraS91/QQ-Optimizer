use std::iter::Map;

use egui::{ScrollArea, Vec2};
use enum_map::{Enum, EnumMap};
use strum::IntoEnumIterator;

use super::{
    assets_loader::PATH_SIZE,
    common::{get_section_frame, light_cone_avatar::LightConeAvatar},
    hsr::basics::Path,
    AppContext, ASSETS_LOADER, COLOR_PALLET,
};

#[derive(Debug, Clone)]
pub struct LightConesGallery {
    path_buttons: EnumMap<Path, bool>,
}

impl Default for LightConesGallery {
    fn default() -> Self {
        Self {
            path_buttons: enum_map::enum_map! { _ => false },
        }
    }
}

impl LightConesGallery {
    pub fn new() -> LightConesGallery {
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
        let button_width = PATH_SIZE.0 * 0.10;
        let total_buttons_width = Path::LENGTH as f32 * button_width;
        let margin = (ui.available_width() - total_buttons_width) / 2.0;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0., 0.);
            ui.add_space(margin);
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
                let card_width = LightConeAvatar::get_default_size().x;
                let cards_num_in_row = (available_width / (card_width + spacing)) as usize;
                let left_space =
                    available_width - cards_num_in_row as f32 * (card_width + spacing) + spacing;

                let light_cones = app_context.light_cones_store.get_all_as_ref();
                let all_paths = self.path_buttons.iter().all(|(_, b)| !b);

                let mut iter = light_cones
                    .into_iter()
                    .filter(|lc| all_paths || self.path_buttons[lc.path])
                    .collect::<Vec<_>>()
                    .into_iter();
                while iter.len() != 0 {
                    ui.horizontal(|ui| {
                        ui.add_space(left_space / 2.0);
                        ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);
                        for _ in 0..cards_num_in_row {
                            if let Some(light_cone) = iter.next() {
                                LightConeAvatar::new(*light_cone, true, false).show_ui(ctx, ui);
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
