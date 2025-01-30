use std::hash::Hash;

use egui::{load::TexturePoll, Color32, Context, Id, Pos2, Ui, Vec2};

use crate::app::{
    assets_loader::{
        UnitImageFormat, ORIGINAL_RELIC_SET_SIZE, ORIGINAL_UNIT_SIZE, RELIC_PIECE_SIZE, ROPE_SIZE,
    },
    hsr::{
        relics::{Relic, RelicPart},
        units::Unit,
    },
    ASSETS_LOADER,
};

pub struct RelicAvatar {
    pub relic: Relic,
    clickable: bool,
    size: Vec2,
    vertically_centered: bool,
}

impl Hash for RelicAvatar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.relic.id.hash(state);
    }
}

impl RelicAvatar {
    pub fn new(relic: Relic, clickable: bool, vertically_centered: bool) -> RelicAvatar {
        RelicAvatar {
            relic,
            clickable,
            size: RelicAvatar::get_default_size(relic.part),
            vertically_centered,
        }
    }

    pub fn get_default_size(relic_part: RelicPart) -> Vec2 {
        if relic_part == RelicPart::Rope {
            Vec2::from(ROPE_SIZE) / 4.0
        } else {
            Vec2::from(RELIC_PIECE_SIZE) / 2.0
        }
    }

    // this will always be vertically ju
    pub fn show_ui(&self, ctx: &Context, ui: &mut Ui) {
        let image_texture_load_result =
            ASSETS_LOADER.get_relic_texture(ctx, self.relic.set, self.relic.part);

        if image_texture_load_result.is_err()
            || matches!(
                image_texture_load_result.as_ref().unwrap(),
                TexturePoll::Pending { .. }
            )
        {
            return;
        }

        let image_texture = match image_texture_load_result.unwrap() {
            TexturePoll::Pending { .. } => return,
            TexturePoll::Ready { texture } => texture,
        };

        let rarity_background = ASSETS_LOADER
            .get_5_star_border()
            .maintain_aspect_ratio(false)
            .fit_to_exact_size(self.size);

        let actual_rectangle = if self.vertically_centered {
            // response rect may be larger than the actual area of the image in justified layout
            // because egui is... weird. We need to calc this manually
            let background_response = ui
                .vertical_centered_justified(|ui| ui.add(rarity_background))
                .inner;
            let actual_size = background_response.intrinsic_size.unwrap();
            let x_margin = (background_response.rect.width() - actual_size.x) / 2.0;
            let y_margin = (background_response.rect.height() - actual_size.y) / 2.0;
            let p0 = background_response.rect.min + Vec2::new(x_margin, y_margin);
            let p1 = p0 + actual_size;
            egui::Rect::from_min_max(p0, p1)
        } else {
            ui.add(rarity_background).rect
        };
        // println!("size: {}", actual_rectangle.size());

        if actual_rectangle.is_positive() {
            let target_factor = if ui.rect_contains_pointer(actual_rectangle) {
                1.1
            } else {
                1.0
            };
            let factor = ui
                .ctx()
                .animate_value_with_time(Id::new(self), target_factor, 0.1);
            let x0y0 = (factor - 1.0) / 2.0;
            let x1y1 = 1.0 - (factor - 1.0) / 2.0;
            let uv = egui::Rect::from_min_max(Pos2::new(x0y0, x0y0), Pos2::new(x1y1, x1y1));
            ui.painter().image(
                image_texture.id,
                actual_rectangle,
                uv,
                Color32::WHITE.to_opaque(),
            );
        }
    }
}
