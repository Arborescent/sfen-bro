//! Font setup

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

const FONT_BYTES: &[u8] = include_bytes!("../resources/YujiMai-Regular.ttf");

/// Setup custom fonts for the application
pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "YujiMai".to_owned(),
        FontData::from_static(FONT_BYTES).into(),
    );

    fonts.families
        .entry(FontFamily::Name("YujiMai".into()))
        .or_default()
        .push("YujiMai".to_owned());

    // Also add to proportional as fallback for kanji
    fonts.families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("YujiMai".to_owned());

    ctx.set_fonts(fonts);
}
