use egui::{self, NumExt, TextStyle, Style, style::WidgetVisuals};
use epaint::{self, RectShape, FontId};

use super::COLOR_PALLET;

pub struct ComboBoxImage{
    id_source: egui::Id,
    selected_text: String,
    image_path: String,
    width: Option<f32>,
}

impl ComboBoxImage {
    pub fn from_id_source(id_source: impl std::hash::Hash) -> Self {
        Self {
            id_source: egui::Id::new(id_source),
            selected_text: Default::default(),
            image_path: Default::default(),
            width: None,
        }
    }

    pub fn show_ui(self, ui: &mut egui::Ui, menu_contents: impl FnOnce(&mut egui::Ui)) -> egui::Response {
        let Self {
            id_source,
            selected_text,
            image_path,
            width,
        } = self;

        let button_id = ui.make_persistent_id(id_source);

        ui.horizontal(|ui| {
            if let Some(width) = width {
                ui.spacing_mut().slider_width = width;
            }
            let mut response = combo_box_image(ui, button_id, selected_text, menu_contents);
            response
        })
        .inner
    }
}

#[allow(clippy::needless_pass_by_value)]
fn combo_box_image(
    ui: &mut egui::Ui,
    button_id: egui::Id,
    selected: impl ToString,
    menu_contents: impl FnOnce(&mut egui::Ui),
) -> egui::Response {
    let popup_id = button_id.with("popup");

    let is_popup_open = ui.memory(|m| {m.is_popup_open(popup_id)});
    let button_response = button_frame(ui, button_id, is_popup_open, egui::Sense::click(), |ui| {
        // We don't want to change width when user selects something new
        let full_minimum_width = ui.spacing().slider_width;
        let icon_size = egui::Vec2::splat(ui.spacing().icon_width);

        let galley = ui
            .fonts(|f| {f.layout_no_wrap(selected.to_string(),  TextStyle::Button.resolve(ui.style()), COLOR_PALLET.text)});

        let width = galley.size().x + ui.spacing().item_spacing.x + icon_size.x;
        let width = width.at_least(full_minimum_width);
        let height = galley.size().y.max(icon_size.y);

        let (_, rect) = ui.allocate_space(egui::Vec2::new(width, height));
        let button_rect = ui.min_rect().expand2(ui.spacing().button_padding);
        let response = ui.interact(button_rect, button_id, egui::Sense::click());
        // response.active |= is_popup_open;

        let icon_rect = egui::Align2::RIGHT_CENTER.align_size_within_rect(icon_size, rect);
        let visuals = if is_popup_open {
            &ui.visuals().widgets.open
        } else {
            ui.style().interact(&response)
        };
        paint_icon(ui.painter(), icon_rect.expand(visuals.expansion), visuals);

        let text_rect = egui::Align2::LEFT_CENTER.align_size_within_rect(galley.size(), rect);
        ui.painter()
            .galley(text_rect.min, galley, visuals.text_color());
    });

    if button_response.clicked() {
        ui.memory_mut(|m| {m.toggle_popup(popup_id)});
    }
    egui::popup::popup_below_widget(ui, popup_id, &button_response, |ui| {
        egui::ScrollArea::horizontal().max_height(ui.spacing().combo_height).show(ui, menu_contents)
    });

    button_response
}

fn button_frame(
    ui: &mut egui::Ui,
    id: egui::Id,
    is_popup_open: bool,
    sense: egui::Sense,
    add_contents: impl FnOnce(&mut egui::Ui),
) -> egui::Response {
    let where_to_put_background = ui.painter().add(egui::Shape::Noop);

    let margin = ui.spacing().button_padding;
    let interact_size = ui.spacing().interact_size;

    let mut outer_rect = ui.available_rect_before_wrap();
    outer_rect.set_height(outer_rect.height().at_least(interact_size.y));

    let inner_rect = outer_rect.shrink2(margin);
    let mut content_ui = ui.child_ui(inner_rect, egui::Layout::left_to_right(egui::Align::Center));
    add_contents(&mut content_ui);

    let mut outer_rect = content_ui.min_rect().expand2(margin);
    outer_rect.set_height(outer_rect.height().at_least(interact_size.y));

    let response = ui.interact(outer_rect, id, sense);
    let visuals = if is_popup_open {
        &ui.visuals().widgets.open
    } else {
        ui.style().interact(&response)
    };

    ui.painter().set(
        where_to_put_background,
        egui::Shape::Rect(RectShape{
            rect: outer_rect.expand(visuals.expansion),
            rounding: visuals.rounding,
            fill: visuals.bg_fill,
            stroke: visuals.bg_stroke,
            fill_texture_id: Default::default(),
            uv: egui::Rect::ZERO
        }),
    );

    ui.advance_cursor_after_rect(outer_rect);

    response
}

fn paint_icon(painter: &egui::Painter, rect: egui::Rect, visuals: &WidgetVisuals) {
    let rect = egui::Rect::from_center_size(
        rect.center(),
        egui::vec2(rect.width() * 0.7, rect.height() * 0.45),
    );
    painter.add(egui::Shape::closed_line(
        vec![rect.left_top(), rect.right_top(), rect.center_bottom()],
        visuals.fg_stroke,
    ));
}