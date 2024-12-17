use crate::app::COLOR_PALLET;
use egui::{self, Margin, Rounding};

use self::{optimize::Optimize, team::Team};

use super::{
    light_cones_store::LightConesStore, relics_store::RelicsStore, units_store::UnitsStore,
};

mod menu;
mod optimize;
mod team;

pub struct Optimizer {
    menu_state: Menu,
    team: Box<Team>,
    optimize: Optimize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Menu {
    Team,
    Optimize,
}

impl Default for Optimizer {
    fn default() -> Self {
        Optimizer {
            menu_state: Menu::Team,
            team: Default::default(),
            optimize: Optimize::default(),
        }
    }
}

pub fn new_page(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    optimizer: &mut Box<Optimizer>,
    relics_store: &mut RelicsStore,
    units_store: &mut UnitsStore,
    light_cones_store: &mut LightConesStore,
) {
    let section_frame = egui::containers::Frame::default()
        .fill(COLOR_PALLET.section())
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::same(20.0));

    optimizer
        .optimize
        .set_main_unit_buffs(optimizer.team.get_active_modifiers());

    let menu_state = egui::CentralPanel::default()
        .frame(section_frame)
        .show_inside(ui, |ui| {
            let menu_response = menu::new(ui, optimizer);

            match &mut optimizer.menu_state {
                Menu::Team => optimizer.team.show_ui(ctx, ui, relics_store, units_store),
                Menu::Optimize => optimizer.optimize.show_ui(
                    ctx,
                    ui,
                    relics_store,
                    units_store,
                    light_cones_store,
                ),
            }

            menu_response.inner
        });

    match menu_state.inner {
        Some(state) => optimizer.menu_state = state,
        _ => {}
    };
}
