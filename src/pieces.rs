//! Piece rendering

use std::collections::HashMap;

use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Stroke, TextureHandle, Vec2};
use egui::epaint::TextShape;

use crate::sfen::{fen_to_unicode, is_chess, is_gote, sfen_to_kanji, Piece};

/// Draw all pieces on the board
pub fn draw_pieces(
    painter: &egui::Painter,
    offset: Pos2,
    cell_size: f32,
    board: &[Vec<Option<Piece>>],
    textures: &HashMap<String, TextureHandle>,
    text_color: Color32,
    board_size: usize,
) {
    let piece_size = cell_size;
    let use_textures = !is_chess(board_size);

    for (row, row_pieces) in board.iter().enumerate() {
        for (col, piece_opt) in row_pieces.iter().enumerate() {
            if let Some(piece) = piece_opt {
                let center = Pos2::new(
                    offset.x + (col as f32 + 0.5) * cell_size,
                    offset.y + (row as f32 + 0.5) * cell_size,
                );

                if use_textures {
                    if let Some(texture) = textures.get(&piece.sfen_key) {
                        let rect = Rect::from_center_size(center, Vec2::splat(piece_size));
                        painter.image(
                            texture.id(),
                            rect,
                            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );
                        continue;
                    }
                }
                draw_piece_as_text(painter, center, cell_size, &piece.sfen_key, text_color, board_size);
            }
        }
    }
}

/// Draw a piece using text when no texture is available
fn draw_piece_as_text(painter: &egui::Painter, center: Pos2, cell_size: f32, piece_key: &str, color: Color32, board_size: usize) {
    let font_size = cell_size * 0.7;

    if is_chess(board_size) {
        // Chess: Unicode symbols, no rotation
        let symbol = fen_to_unicode(piece_key);
        let font = FontId::proportional(font_size);
        painter.text(center, egui::Align2::CENTER_CENTER, symbol, font, color);
    } else {
        // Shogi: Kanji with YujiMai font, gote pieces rotated
        let kanji = sfen_to_kanji(piece_key);
        let font = FontId::new(font_size, FontFamily::Name("YujiMai".into()));

        let is_gote_piece = is_gote(piece_key);
        let galley = painter.layout_no_wrap(kanji.to_string(), font, color);
        let half_size = galley.size() / 2.0;

        // For gote pieces, rotate 180 degrees around the center
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
}

/// Draw pieces in hand for a single player
/// `is_gote_hand` determines orientation (gote pieces are rotated)
pub fn draw_hand(
    painter: &egui::Painter,
    top_left: Pos2,
    cell_size: f32,
    hand: &[(String, u32)],
    textures: &HashMap<String, TextureHandle>,
    text_color: Color32,
    border_color: Color32,
    is_gote_hand: bool,
) {
    let piece_size = cell_size * 0.9;
    let count_font_size = cell_size * 0.35;
    let count_font = FontId::proportional(count_font_size);
    let padding = cell_size * 0.1;

    // Draw border rectangle
    let rect_height = hand.len() as f32 * cell_size + padding * 2.0;
    let rect_width = cell_size + padding * 2.0;
    let rect = Rect::from_min_size(
        Pos2::new(top_left.x - padding, top_left.y - padding),
        Vec2::new(rect_width, rect_height),
    );
    painter.rect_stroke(rect, 2.0, Stroke::new(1.0, border_color), egui::StrokeKind::Outside);

    for (idx, (sfen_key, count)) in hand.iter().enumerate() {
        let center = Pos2::new(
            top_left.x + cell_size * 0.5,
            top_left.y + (idx as f32 + 0.5) * cell_size,
        );

        // Try texture first
        let display_key = if is_gote_hand {
            sfen_key.to_lowercase()
        } else {
            sfen_key.to_uppercase()
        };

        if let Some(texture) = textures.get(&display_key) {
            let rect = Rect::from_center_size(center, Vec2::splat(piece_size));
            painter.image(
                texture.id(),
                rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        } else {
            // Fall back to kanji
            draw_hand_piece_kanji(painter, center, cell_size, &display_key, text_color, is_gote_hand);
        }

        // Draw count if more than 1
        if *count > 1 {
            let count_pos = Pos2::new(
                center.x + cell_size * 0.3,
                center.y + cell_size * 0.3,
            );
            painter.text(count_pos, egui::Align2::CENTER_CENTER, count.to_string(), count_font.clone(), text_color);
        }
    }
}

/// Draw a hand piece using kanji
fn draw_hand_piece_kanji(
    painter: &egui::Painter,
    center: Pos2,
    cell_size: f32,
    piece_key: &str,
    color: Color32,
    is_gote_hand: bool,
) {
    let font_size = cell_size * 0.65;
    let kanji = sfen_to_kanji(piece_key);
    let font = FontId::new(font_size, FontFamily::Name("YujiMai".into()));

    let galley = painter.layout_no_wrap(kanji.to_string(), font, color);
    let half_size = galley.size() / 2.0;

    let (text_pos, angle) = if is_gote_hand {
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
