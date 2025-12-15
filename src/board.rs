//! Board rendering

use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

use crate::sfen::{CHESS_SIZE, STANDARD_SHOGI_SIZE};

const COORD_MARGIN_STANDARD: f32 = 0.05;
const COORD_MARGIN_OTHER: f32 = 0.10;
const TEXT_OFFSET: f32 = 0.08;

const KANJI_NUMERALS: [&str; 9] = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
const CHESS_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

/// Get coordinate margin based on board size
pub fn coord_margin(board_size: usize) -> f32 {
    if board_size == STANDARD_SHOGI_SIZE {
        COORD_MARGIN_STANDARD
    } else {
        COORD_MARGIN_OTHER
    }
}

/// Draw the board grid lines (for shogi)
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

/// Draw a checkerboard pattern (for chess)
pub fn draw_checkerboard(
    painter: &egui::Painter,
    offset: Pos2,
    cell_size: f32,
    board_size: usize,
    light_color: Color32,
    dark_color: Color32,
) {
    for row in 0..board_size {
        for col in 0..board_size {
            let is_light = (row + col) % 2 == 0;
            let color = if is_light { light_color } else { dark_color };
            let rect = Rect::from_min_size(
                Pos2::new(offset.x + col as f32 * cell_size, offset.y + row as f32 * cell_size),
                Vec2::splat(cell_size),
            );
            painter.rect_filled(rect, 0.0, color);
        }
    }
}

/// Draw board coordinates
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
    let margin = cell_size * TEXT_OFFSET;

    if board_size == CHESS_SIZE {
        // Chess: files a-h (left to right), ranks 8-1 (top to bottom)
        for (col, file) in CHESS_FILES.iter().enumerate() {
            let x = offset.x + (col as f32 + 0.5) * cell_size;
            let y = offset.y + board_pixels + margin;
            painter.text(
                Pos2::new(x, y),
                Align2::CENTER_TOP,
                *file,
                font.clone(),
                color,
            );
        }
        for row in 0..board_size {
            let rank = board_size - row;
            let x = offset.x - margin;
            let y = offset.y + (row as f32 + 0.5) * cell_size;
            painter.text(
                Pos2::new(x, y),
                Align2::RIGHT_CENTER,
                rank.to_string(),
                font.clone(),
                color,
            );
        }
    } else {
        // Shogi: files numbered right to left, ranks as kanji
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
