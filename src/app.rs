use egui::{Color32, Margin, RichText};
use egui_extras::StripBuilder;
use serde::{Deserialize, Serialize};

use self::{
    light_cones_store::LightConesStore, relics_store::RelicsStore, units_store::UnitsStore,
};
mod comboBoxImage;
mod hsr;
mod light_cones_store;
mod optimizer;
mod relics_store;
mod units_store;

pub static COLOR_PALLET: ColorPallet = ColorPallet {
    top_menu: Color32::from_rgb(57, 62, 70),
    background: Color32::from_rgb(34, 40, 49),
    card: Color32::from_rgb(52, 61, 75),
    section: Color32::from_rgb(42, 49, 60),
    text: Color32::from_rgb(238, 238, 238),
    highlighted_text: Color32::from_rgb(0, 173, 181),
};

const UNITS_STORE_KEY: &str = "unit_store";
const RELICS_STORE_KEY: &str = "relics_store";
const LIGHT_CONES_STORE_KEY: &str = "relics_store";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// if we add new fields, give them default values when deserializing old state
pub struct QQOptimizer {
    label: String,
    menu_state: MenuState,
    sub_page: PageState,
    relics_store: RelicsStore,
    units_store: UnitsStore,
    light_cones_store: LightConesStore,
}

enum PageState {
    OptimizerTab(Box<optimizer::Optimizer>),
}

#[derive(Deserialize, Serialize)]
pub struct ColorPallet {
    top_menu: Color32,
    background: Color32,
    card: Color32,
    section: Color32,
    text: Color32,
    highlighted_text: Color32,
}

impl ColorPallet {
    pub fn top_menu(&self) -> Color32 {
        self.top_menu
    }
    pub fn background(&self) -> Color32 {
        self.background
    }
    pub fn card(&self) -> Color32 {
        self.card
    }
    pub fn section(&self) -> Color32 {
        self.section
    }
    pub fn text(&self) -> Color32 {
        self.text
    }
    pub fn highlighted_text(&self) -> Color32 {
        self.highlighted_text
    }
}

enum MenuState {
    Units,
    Optimizer,
}

impl Default for QQOptimizer {
    fn default() -> Self {
        Self {
            label: "QQ optimizer".to_owned(),
            menu_state: MenuState::Units,
            sub_page: PageState::OptimizerTab(Box::new(Default::default())),
            relics_store: Default::default(),
            units_store: Default::default(),
            light_cones_store: Default::default(),
        }
    }
}

impl QQOptimizer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app = Self::default();
            app.relics_store = eframe::get_value(storage, RELICS_STORE_KEY).unwrap_or_default();
            app.units_store = eframe::get_value(storage, UNITS_STORE_KEY).unwrap_or_default();
            app.light_cones_store =
                eframe::get_value(storage, LIGHT_CONES_STORE_KEY).unwrap_or_default();
            return app;
        }

        Default::default()
    }
}

impl eframe::App for QQOptimizer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, RELICS_STORE_KEY, &self.relics_store);
        eframe::set_value(storage, UNITS_STORE_KEY, &self.units_store);
        eframe::set_value(storage, LIGHT_CONES_STORE_KEY, &self.light_cones_store);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let top_panel = egui::TopBottomPanel::top("top_panel").exact_height(20.0);
        let top_panel_frame = egui::containers::Frame::default().fill(COLOR_PALLET.top_menu);
        top_panel.frame(top_panel_frame).show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                let optimizer_button = egui::Button::new(RichText::new("Optimizer").color(
                    if matches!(self.menu_state, MenuState::Optimizer) {
                        COLOR_PALLET.highlighted_text
                    } else {
                        COLOR_PALLET.text
                    },
                ));

                let units_button = egui::Button::new(RichText::new("Units").color(
                    if matches!(self.menu_state, MenuState::Units) {
                        COLOR_PALLET.highlighted_text
                    } else {
                        COLOR_PALLET.text
                    },
                ));

                let optimizer_button_r = ui.add(optimizer_button);
                let units_button_r = ui.add(units_button);
                if optimizer_button_r.clicked() {
                    self.menu_state = MenuState::Optimizer;
                } else if units_button_r.clicked() {
                    self.menu_state = MenuState::Units;
                }
            });
        });

        let background_frame = egui::containers::Frame::default()
            .fill(COLOR_PALLET.background)
            .inner_margin(Margin::same(20.0));

        egui::CentralPanel::default()
            .frame(background_frame)
            .show(ctx, |ui| {
                match &mut self.sub_page {
                    PageState::OptimizerTab(optimizer) => optimizer::new_page(
                        ui,
                        optimizer,
                        &mut self.relics_store,
                        &mut self.units_store,
                        &mut self.light_cones_store,
                    ),
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
