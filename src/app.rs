//! Main application

use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui::{self, Color32, ColorImage, Pos2, TextureHandle};

use crate::board::{draw_coordinates, draw_grid, draw_hoshi_points, COORD_MARGIN};
use crate::config::Config;
use crate::pieces::draw_pieces;
use crate::sfen::{detect_board_size, parse_sfen, Piece};

/// Main application state
pub struct SfenApp {
    board: Vec<Vec<Option<Piece>>>,
    board_size: usize,
    textures: HashMap<String, TextureHandle>,
    assets_path: PathBuf,
    config: Config,
    background_color: Color32,
    textures_loaded: bool,
    frame_count: u32,
}

impl SfenApp {
    /// Create a new application with the given SFEN, assets path, and config
    pub fn new(sfen: String, assets_path: PathBuf, config: Config) -> Self {
        let board_size = detect_board_size(&sfen);
        let board = parse_sfen(&sfen, board_size);
        let background_color = config.background_color();
        Self {
            board,
            board_size,
            textures: HashMap::new(),
            assets_path,
            config,
            background_color,
            textures_loaded: false,
            frame_count: 0,
        }
    }

    /// Load textures from configured piece files
    fn load_textures(&mut self, ctx: &egui::Context) {
        if self.textures_loaded {
            return;
        }

        for (sfen_key, filename) in &self.config.pieces {
            let path = self.assets_path.join(filename);
            if let Ok(img) = image::open(&path) {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let pixels = rgba.into_raw();
                let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture = ctx.load_texture(filename, color_image, Default::default());
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
            let coord_space = total_size * COORD_MARGIN;
            let board_pixels = total_size - coord_space * 2.0;
            let cell_size = board_pixels / self.board_size as f32;

            // Center the board, coordinates go in the space around it
            let offset = Pos2::new(
                (available.x - board_pixels) / 2.0,
                (available.y - board_pixels) / 2.0,
            );

            let painter = ui.painter();

            draw_grid(painter, offset, board_pixels, cell_size, self.board_size);
            draw_coordinates(painter, offset, board_pixels, cell_size, self.board_size);
            draw_hoshi_points(painter, offset, cell_size, self.board_size);
            draw_pieces(painter, offset, cell_size, &self.board, &self.textures);
        });
    }
}
