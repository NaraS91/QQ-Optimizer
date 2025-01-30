use egui::{self};

use self::{optimize::Optimize, team::Team};

use super::{common::get_section_frame, AppContext};

mod menu;
mod optimize;
mod team;

pub struct Optimizer {
    menu_state: Menu,
    team: Box<Team>,
    optimize: Optimize,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
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

impl Optimizer {
    pub fn show_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, app_ctx: &mut AppContext) {
        self.optimize
            .set_main_unit_buffs(self.team.get_active_modifiers());

        let menu_state = egui::CentralPanel::default()
            .frame(get_section_frame())
            .show_inside(ui, |ui| {
                let menu_response = menu::new(ui, self.menu_state.clone());

                match &mut self.menu_state {
                    Menu::Team => self.team.show_ui(ctx, ui, app_ctx),
                    Menu::Optimize => self.optimize.show_ui(ctx, ui, app_ctx),
                }

                menu_response.inner
            });

        match menu_state.inner {
            Some(state) => self.menu_state = state,
            _ => {}
        };
    }
}
