//! Help › About Tibetan Design Layout: tabs About · Contributors · Models. About is the splash with the
//! ArtCraft mark, version, and community links (Discord first and largest), plus the other ArtCraft
//! apps; Contributors and Models are the compiled-in credits (`crate::credits`).

use designcraft_engine::links;
use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::DesignApp;
use crate::theme::{Tokens, semibold};

/// ArtCraft brand blue (from `docs/brand/artcraft-mark.svg`).
pub const BRAND: Color32 = Color32::from_rgb(0x4e, 0x7b, 0xfb);
/// Discord's brand colour, used for the "More Tibetan apps" buttons (colour only; no Discord artwork).
pub const DISCORD: Color32 = Color32::from_rgb(0x58, 0x65, 0xf2);

/// Tibetan resource links (URL, label, description).
pub const SIBLINGS: &[(&str, &str, &str)] = &[
    ("https://nangchen.com", "Nangchen", "Tibetan apps"),
    ("https://studytibetan.com", "StudyTibetan", "Tibetan learning"),
    ("https://tibetanebook.com", "TibetanEbook", "Tibetan books"),
];

/// The About window's tabs, in `UiState::about_tab` order (`help.about {tab}` names them in lowercase).
pub const ABOUT_TABS: [&str; 3] = ["About", "Contributors", "Models"];

/// Paint the ArtCraft mark into `rect` (square), rasterised once per size and cached.
pub fn paint_mark(ui: &egui::Ui, rect: Rect, _color: Color32) {
    let key = egui::Id::new("tibetan-layout-logo");
    let cached: Option<egui::TextureHandle> = ui.ctx().data(|d| d.get_temp(key));
    let texture = cached.or_else(|| {
        let decoded = image::load_from_memory(include_bytes!("../../../assets/app-icon/designcraft-1024.png")).ok()?.to_rgba8();
        let size = [decoded.width() as usize, decoded.height() as usize];
        let pixels = egui::ColorImage::from_rgba_unmultiplied(size, decoded.as_raw());
        let texture = crate::widgets::load_texture(ui.ctx(), "tibetan-layout-logo", pixels, egui::TextureOptions::LINEAR);
        ui.ctx().data_mut(|d| d.insert_temp(key, texture.clone()));
        Some(texture)
    });
    if let Some(texture) = texture {
        ui.painter().image(texture.id(), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
    }
}

/// A speech-bubble glyph (drawn in code) for Discord buttons.
pub fn paint_chat_icon(p: &egui::Painter, c: egui::Pos2, s: f32, color: Color32) {
    let body = Rect::from_center_size(c - vec2(0.0, s * 0.06), vec2(s, s * 0.72));
    p.rect_filled(body, s * 0.24, color);
    let tail = vec![
        pos2(body.left() + s * 0.22, body.bottom() - 1.0),
        pos2(body.left() + s * 0.12, body.bottom() + s * 0.24),
        pos2(body.left() + s * 0.44, body.bottom() - 1.0),
    ];
    p.add(egui::Shape::convex_polygon(tail, color, Stroke::NONE));
    let eye = s * 0.09;
    let bg = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 380 { DISCORD } else { Color32::WHITE };
    p.circle_filled(body.center() - vec2(s * 0.18, 0.0), eye, bg);
    p.circle_filled(body.center() + vec2(s * 0.18, 0.0), eye, bg);
}

/// The big "More Tibetan apps" button. Returns true when clicked.
pub fn discord_button(ui: &mut egui::Ui, label: &str, size: egui::Vec2) -> bool {
    let (r, resp) = ui.allocate_exact_size(size, Sense::click());
    let resp = resp.on_hover_text(links::DISCORD).on_hover_cursor(egui::CursorIcon::PointingHand);
    let fill = if resp.hovered() { Color32::from_rgb(0x47, 0x52, 0xc4) } else { DISCORD };
    ui.painter().rect_filled(r, size.y / 2.0, fill);
    let icon = size.y * 0.5;
    let font = semibold((size.y * 0.42).max(11.0));
    let galley = crate::rtl::plain(ui.ctx(), label, font, Color32::WHITE);
    let total = icon + 8.0 + galley.size().x;
    let x0 = r.center().x - total / 2.0;
    paint_chat_icon(ui.painter(), pos2(x0 + icon / 2.0, r.center().y), icon, Color32::WHITE);
    ui.painter().galley(pos2(x0 + icon + 8.0, r.center().y - galley.size().y / 2.0), galley, Color32::WHITE);
    resp.clicked()
}

/// A text link that opens `url`. Returns true when clicked.
fn link(ui: &mut egui::Ui, text: &str, url: &str, color: Color32) -> bool {
    let resp = ui.add(egui::Label::new(egui::RichText::new(text).size(13.0).color(color).underline()).sense(Sense::click()));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    resp.clicked()
}

pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open: Option<&'static str> = None;
    let area = egui::Id::new("about");
    let instance = area.with("placement");
    let resp = crate::modal_drag::modal(ctx, area, instance).show(ctx, |ui| {
        ui.set_width(if app.ui.about_tab == 0 { 460.0 } else { 640.0 });
        // The tab row is the title row. The tabs register after the drag area, so they stay clickable.
        let margin = egui::Frame::popup(ui.style()).inner_margin;
        let top = ui.cursor().top();
        let title_row = Rect::from_min_max(
            pos2(ui.max_rect().left() - margin.leftf(), top - margin.topf()),
            pos2(ui.max_rect().right() + margin.rightf(), top + ui.spacing().interact_size.y),
        );
        crate::modal_drag::title_drag(ui, instance, title_row);
        ui.horizontal(|ui| {
            for (i, l) in ABOUT_TABS.iter().enumerate() {
                let i = i as u8;
                if ui.selectable_label(app.ui.about_tab == i, crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, l))).clicked() {
                    app.ui.about_tab = i;
                }
            }
        });
        ui.separator();
        if app.ui.about_tab != 0 {
            let h = 400.0_f32.min(ctx.content_rect().height() * 0.7).max(160.0);
            ui.allocate_ui_with_layout(vec2(ui.available_width(), h), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_min_height(h);
                if app.ui.about_tab == 1 {
                    crate::credits::contributors_ui(ui, &app.ui.language);
                } else {
                    crate::credits::models_ui(ui, &app.ui.language);
                }
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                    app.ui.about = false;
                }
            });
            return;
        }
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            let (r, _) = ui.allocate_exact_size(vec2(72.0, 72.0), Sense::hover());
            paint_mark(ui, r, BRAND);
            ui.add_space(6.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Tibetan Design Layout")).font(semibold(26.0)).color(t.text_strong),
            ));
            crate::rtl::label(
                ui,
                egui::RichText::new(format!("{} {}", crate::i18n::tr(&app.ui.language, "Version"), env!("CARGO_PKG_VERSION")))
                    .size(12.0)
                    .color(t.text_dim),
            );
            ui.add_space(4.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Page layout for print and screen — fast, open and scriptable."))
                    .size(13.0)
                    .color(t.text),
            ));
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(
                    &app.ui.language,
                    "Tibetan and English page layout. Based on DesignCraft; original credits retained.",
                ))
                .size(12.0)
                .color(t.text_dim),
            ));
            ui.label(crate::rtl::widget(ui, "བོད་ཀྱི་ཤོག་ངོས་བཟོ་བཀོད།"));
            ui.label(crate::rtl::widget(ui, "Created by ནང་ཆེན་བཀྲ་ཤིས།"));
            let update_label = if app.ui.language == "bo" {
                "གསར་སྒྱུར་རང་འགུལ་གྱིས་ཞིབ་བཤེར།"
            } else {
                "Automatically check for updates"
            };
            ui.checkbox(&mut app.ui.automatic_updates, update_label);

            ui.add_space(16.0);
            if discord_button(ui, "More Tibetan apps", vec2(300.0, 40.0)) {
                open = Some("discord");
            }
            ui.add_space(4.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "studytibetan.com — help, feedback and show-and-tell"))
                    .size(11.0)
                    .color(t.text_dim),
            ));
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                // Centre the row of links.
                let w = 330.0;
                ui.add_space(((ui.available_width() - w) / 2.0).max(0.0));
                for (text, key) in
                    [("Tibetan Design Layout page", "appPage"), ("GitHub", "github"), ("nangchen.com", "website"), ("Report an issue", "issues")]
                {
                    if link(ui, crate::i18n::tr(&app.ui.language, text), links::get(key).unwrap_or_default(), t.accent) {
                        open = Some(key);
                    }
                    ui.add_space(6.0);
                }
            });
            ui.add_space(16.0);
            ui.separator();
            ui.add_space(6.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "More Tibetan apps")).font(semibold(12.0)).color(t.text_strong),
            ));
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let pad = ui.spacing().button_padding.x * 2.0 + ui.spacing().item_spacing.x;
                let w: f32 = SIBLINGS
                    .iter()
                    .map(|(_, n, _)| ui.painter().layout_no_wrap(n.to_string(), egui::FontId::proportional(12.0), Color32::WHITE).size().x + pad)
                    .sum::<f32>()
                    - ui.spacing().item_spacing.x;
                ui.add_space(((ui.available_width() - w) / 2.0).max(0.0));
                for (slug, name, what) in SIBLINGS {
                    let url = (*slug).to_string();
                    let b =
                        ui.add(egui::Button::new(egui::RichText::new(*name).size(12.0)).corner_radius(10.0)).on_hover_text(format!("{what} — {url}"));
                    if b.clicked() {
                        app.ui.pending_urls.push(url);
                    }
                }
            });
            ui.add_space(12.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Open source. No telemetry.")).size(11.0).color(t.text_dim),
            ));
            ui.add_space(8.0);
            if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                app.ui.about = false;
            }
        });
    });
    if resp.should_close() {
        app.ui.about = false;
    }
    if let Some(k) = open {
        let _ = app.run(&format!("help.{k}"), json!({}));
    }
}
