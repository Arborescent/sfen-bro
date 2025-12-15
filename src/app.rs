//! Main application

use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui::{self, Color32, ColorImage, Pos2, TextureHandle};

use crate::board::{coord_margin, draw_checkerboard, draw_coordinates, draw_grid, draw_hoshi_points};
use crate::config::Config;
use crate::pieces::draw_pieces;
use crate::sfen::{detect_board_size, is_chess, parse_sfen, Piece};

/// Main application state
pub struct SfenApp {
    board: Vec<Vec<Option<Piece>>>,
    board_size: usize,
    textures: HashMap<String, TextureHandle>,
    assets_path: PathBuf,
    config: Config,
    background_color: Color32,
    grid_color: Color32,
    text_color: Color32,
    light_square_color: Color32,
    dark_square_color: Color32,
    is_chess: bool,
    textures_loaded: bool,
    frame_count: u32,
}

impl SfenApp {
    /// Create a new application with the given SFEN, assets path, and config
    pub fn new(sfen: String, assets_path: PathBuf, config: Config) -> Self {
        let board_size = detect_board_size(&sfen);
        let board = parse_sfen(&sfen, board_size);
        let is_chess_board = is_chess(board_size);

        let (background_color, grid_color, text_color) = if is_chess_board {
            (
                config.chess.light_square_color(),
                Color32::TRANSPARENT,
                config.chess.text_color(),
            )
        } else {
            (
                config.shogi.background_color(),
                config.shogi.grid_color(),
                config.shogi.text_color(),
            )
        };

        let light_square_color = config.chess.light_square_color();
        let dark_square_color = config.chess.dark_square_color();

        Self {
            board,
            board_size,
            textures: HashMap::new(),
            assets_path,
            config,
            background_color,
            grid_color,
            text_color,
            light_square_color,
            dark_square_color,
            is_chess: is_chess_board,
            textures_loaded: false,
            frame_count: 0,
        }
    }

    /// Load textures from configured piece files (skipped for chess)
    fn load_textures(&mut self, ctx: &egui::Context) {
        if self.textures_loaded || self.is_chess {
            self.textures_loaded = true;
            return;
        }

        // Determine base path for piece images
        let base_path = if let Some(ref assets) = self.config.shogi.assets_path {
            let assets_path = PathBuf::from(assets);
            if assets_path.is_absolute() {
                assets_path
            } else {
                self.assets_path.join(assets_path)
            }
        } else {
            self.assets_path.clone()
        };

        for (sfen_key, piece_path) in &self.config.shogi.pieces {
            let piece_path = PathBuf::from(piece_path);
            let path = if piece_path.is_absolute() {
                piece_path
            } else {
                base_path.join(piece_path)
            };

            if let Ok(img) = image::open(&path) {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let pixels = rgba.into_raw();
                let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture = ctx.load_texture(sfen_key, color_image, Default::default());
                self.textures.insert(sfen_key.clone(), texture);
            }
        }

        self.textures_loaded = true;
    }
}

impl eframe::App for SfenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count = self.frame_count.saturating_add(1);

        // Close on Escape or when window loses focus
        let lost_focus = self.frame_count > 10 && ctx.input(|i| !i.focused);
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) || lost_focus {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.load_textures(ctx);

        let frame = egui::Frame::NONE.fill(self.background_color);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let available = ui.available_size();
            let total_size = available.x.min(available.y);
            let coord_space = total_size * coord_margin(self.board_size);
            let board_pixels = total_size - coord_space * 2.0;
            let cell_size = board_pixels / self.board_size as f32;

            // Center the board, coordinates go in the space around it
            let offset = Pos2::new(
                (available.x - board_pixels) / 2.0,
                (available.y - board_pixels) / 2.0,
            );

            let painter = ui.painter();

            if self.is_chess {
                draw_checkerboard(
                    painter,
                    offset,
                    cell_size,
                    self.board_size,
                    self.light_square_color,
                    self.dark_square_color,
                );
            } else {
                draw_grid(
                    painter,
                    offset,
                    board_pixels,
                    cell_size,
                    self.board_size,
                    self.grid_color,
                );
                draw_hoshi_points(painter, offset, cell_size, self.board_size, self.grid_color);
            }

            draw_coordinates(
                painter,
                offset,
                board_pixels,
                cell_size,
                self.board_size,
                self.text_color,
            );
            draw_pieces(
                painter,
                offset,
                cell_size,
                &self.board,
                &self.textures,
                self.text_color,
                self.board_size,
            );
        });
    }
}
