//! Board rendering

use eframe::egui::{self, Align2, Color32, FontId, Pos2, Stroke};

use crate::sfen::STANDARD_SHOGI_SIZE;

pub const COORD_MARGIN: f32 = 0.05;

const KANJI_NUMERALS: [&str; 9] = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// Draw the board grid lines
pub fn draw_grid(
    painter: &egui::Painter,
    offset: Pos2,
    board_pixels: f32,
    cell_size: f32,
    board_size: usize,
    color: Color32,
) {
    for i in 0..=board_size {
        let x = offset.x + i as f32 * cell_size;
        let y = offset.y + i as f32 * cell_size;
        painter.line_segment(
            [Pos2::new(x, offset.y), Pos2::new(x, offset.y + board_pixels)],
            Stroke::new(1.0, color),
        );
        painter.line_segment(
            [Pos2::new(offset.x, y), Pos2::new(offset.x + board_pixels, y)],
            Stroke::new(1.0, color),
        );
    }
}

/// Draw board coordinates (files as numbers, ranks as kanji)
pub fn draw_coordinates(
    painter: &egui::Painter,
    offset: Pos2,
    board_pixels: f32,
    cell_size: f32,
    board_size: usize,
    color: Color32,
) {
    let font_size = cell_size * 0.35;
    let font = FontId::proportional(font_size);
    let margin = cell_size * COORD_MARGIN;

    // Files (columns) - numbered right to left
    for col in 0..board_size {
        let file_num = board_size - col;
        let x = offset.x + (col as f32 + 0.5) * cell_size;
        let y = offset.y - margin;
        painter.text(
            Pos2::new(x, y),
            Align2::CENTER_BOTTOM,
            file_num.to_string(),
            font.clone(),
            color,
        );
    }

    // Ranks (rows) - kanji numerals
    for (row, kanji) in KANJI_NUMERALS.iter().enumerate().take(board_size) {
        let x = offset.x + board_pixels + margin;
        let y = offset.y + (row as f32 + 0.5) * cell_size;
        painter.text(
            Pos2::new(x, y),
            Align2::LEFT_CENTER,
            *kanji,
            font.clone(),
            color,
        );
    }
}

/// Draw hoshi (star) points on standard shogi board
pub fn draw_hoshi_points(painter: &egui::Painter, offset: Pos2, cell_size: f32, board_size: usize, color: Color32) {
    if board_size != STANDARD_SHOGI_SIZE {
        return;
    }

    let hoshi_radius = cell_size * 0.06;
    for (row, col) in [(3, 3), (3, 6), (6, 3), (6, 6)] {
        let center = Pos2::new(
            offset.x + col as f32 * cell_size,
            offset.y + row as f32 * cell_size,
        );
        painter.circle_filled(center, hoshi_radius, color);
    }
}
