//! The look of the program: warm paper colours, deep green for actions, comfortable text.

use eframe::egui::{self, Color32, CornerRadius, FontFamily, FontId, Stroke, TextStyle};

pub const PAPER: Color32 = Color32::from_rgb(0xF5, 0xF0, 0xE6);
pub const CARD: Color32 = Color32::from_rgb(0xFF, 0xFD, 0xF8);
pub const BORDER: Color32 = Color32::from_rgb(0xD9, 0xCF, 0xBC);
pub const GREEN: Color32 = Color32::from_rgb(0x2F, 0x6B, 0x57);
pub const GREEN_SOFT: Color32 = Color32::from_rgb(0xDD, 0xEB, 0xE3);
pub const GOLD: Color32 = Color32::from_rgb(0x8F, 0x6C, 0x1F);
pub const GOLD_SOFT: Color32 = Color32::from_rgb(0xF6, 0xEC, 0xD2);
pub const INK: Color32 = Color32::from_rgb(0x2A, 0x26, 0x20);
pub const MUTED: Color32 = Color32::from_rgb(0x66, 0x5E, 0x52);

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::ThemePreference::Light);
    let mut v = egui::Visuals::light();
    v.panel_fill = PAPER;
    v.window_fill = CARD;
    v.extreme_bg_color = Color32::WHITE;
    v.faint_bg_color = Color32::from_rgb(0xEF, 0xE9, 0xDC);
    v.hyperlink_color = GREEN;
    v.selection.bg_fill = GREEN_SOFT;
    v.selection.stroke = Stroke::new(1.0, GREEN);
    v.window_stroke = Stroke::new(1.0, BORDER);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, INK);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    let radius = CornerRadius::same(6);
    let button = |w: &mut egui::style::WidgetVisuals, fill: Color32, edge: Color32| {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.bg_stroke = Stroke::new(1.0, edge);
        w.fg_stroke = Stroke::new(1.0, INK);
        w.corner_radius = radius;
    };
    button(&mut v.widgets.inactive, Color32::from_rgb(0xEC, 0xE5, 0xD5), BORDER);
    button(&mut v.widgets.hovered, Color32::from_rgb(0xE3, 0xDB, 0xC8), GREEN);
    button(&mut v.widgets.active, GREEN_SOFT, GREEN);
    button(&mut v.widgets.open, GREEN_SOFT, GREEN);
    ctx.set_visuals_of(egui::Theme::Light, v);

    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(10.0, 4.0);
        style.spacing.interact_size.y = 26.0;
        for (kind, size, family) in [
            (TextStyle::Body, 15.0, FontFamily::Proportional),
            (TextStyle::Button, 15.0, FontFamily::Proportional),
            (TextStyle::Small, 12.0, FontFamily::Proportional),
            (TextStyle::Heading, 21.0, FontFamily::Proportional),
            (TextStyle::Monospace, 14.0, FontFamily::Monospace),
        ] {
            style.text_styles.insert(kind, FontId::new(size, family));
        }
    });
}
