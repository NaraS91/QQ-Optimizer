#[derive(serde::Deserialize, serde::Serialize)]
pub struct Requirements {}

impl Requirements {
    pub fn new() -> Self {
        Requirements {}
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("some text");
        ui.label("lorem ipsum");
    }
}
