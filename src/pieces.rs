//! Piece rendering

use std::collections::HashMap;

use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Stroke, TextureHandle, Vec2};
use egui::epaint::TextShape;

use crate::sfen::{is_gote, sfen_to_kanji, Piece};

/// Draw all pieces on the board
pub fn draw_pieces(
    painter: &egui::Painter,
    offset: Pos2,
    cell_size: f32,
    board: &[Vec<Option<Piece>>],
    textures: &HashMap<String, TextureHandle>,
    text_color: Color32,
) {
    let piece_size = cell_size;
    for (row, row_pieces) in board.iter().enumerate() {
        for (col, piece_opt) in row_pieces.iter().enumerate() {
            if let Some(piece) = piece_opt {
                let center = Pos2::new(
                    offset.x + (col as f32 + 0.5) * cell_size,
                    offset.y + (row as f32 + 0.5) * cell_size,
                );

                if let Some(texture) = textures.get(&piece.sfen_key) {
                    let rect = Rect::from_center_size(center, Vec2::splat(piece_size));
                    painter.image(
                        texture.id(),
                        rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                } else {
                    draw_piece_as_text(painter, center, cell_size, &piece.sfen_key, text_color);
                }
            }
        }
    }
}

/// Draw a piece using text (kanji) when no texture is available
fn draw_piece_as_text(painter: &egui::Painter, center: Pos2, cell_size: f32, sfen_key: &str, color: Color32) {
    let kanji = sfen_to_kanji(sfen_key);
    let font_size = cell_size * 0.7;
    let font = FontId::new(font_size, FontFamily::Name("YujiMai".into()));

    let is_gote_piece = is_gote(sfen_key);
    let galley = painter.layout_no_wrap(kanji.to_string(), font, color);
    let half_size = galley.size() / 2.0;

    // For gote pieces, rotate 180 degrees around the center
    // TextShape.angle rotates around pos (top-left), so we adjust position accordingly
    let (text_pos, angle) = if is_gote_piece {
        (center + half_size, std::f32::consts::PI)
    } else {
        (center - half_size, 0.0)
    };

    let text_shape = TextShape {
        pos: text_pos,
        galley,
        override_text_color: Some(color),
        underline: Stroke::NONE,
        fallback_color: color,
        opacity_factor: 1.0,
        angle,
    };

    painter.add(text_shape);
}
