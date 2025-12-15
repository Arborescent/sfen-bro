//! SFEN Bro - SFEN clipboard viewer for shogi positions

mod app;
mod board;
mod config;
mod fonts;
mod pieces;
mod sfen;

use std::path::PathBuf;

use arboard::Clipboard;
use eframe::egui;

use app::SfenApp;
use config::{load_config, Config};
use fonts::setup_fonts;
use sfen::{detect_board_size, is_chess, parse_hand, CHESS_SIZE, MINISHOGI_SIZE};

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let (config, assets_path) = if let Some(config_path) = args.get(1).map(PathBuf::from) {
        let config = load_config(&config_path);
        let assets_path = config_path
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf();
        (config, assets_path)
    } else {
        (Config::default(), PathBuf::from("."))
    };

    let sfen = read_sfen_from_clipboard();
    let board_size = detect_board_size(&sfen);
    let hand = parse_hand(&sfen);
    let has_hand = !is_chess(board_size) && (!hand.sente.is_empty() || !hand.gote.is_empty());

    let base_size = match board_size {
        MINISHOGI_SIZE => 500.0,
        CHESS_SIZE => 700.0,
        _ => 900.0, // Standard shogi
    };
    let window_width = base_size * config.scale_factor();

    // Calculate actual height needed based on layout
    let window_height = if has_hand {
        let hand_width_ratio = 0.12;
        let coord_margin_ratio = if board_size == MINISHOGI_SIZE { 0.10 } else { 0.05 };
        let board_area_ratio = 1.0 - hand_width_ratio * 2.0; // 0.76
        let coord_space_ratio = board_area_ratio * coord_margin_ratio;
        let board_ratio = board_area_ratio - coord_space_ratio * 2.0;
        let vertical_margin_ratio = coord_space_ratio * 1.2;
        // Height = top_margin + board + bottom_margin
        window_width * (vertical_margin_ratio * 2.0 + board_ratio)
    } else {
        window_width
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_width, window_height])
            .with_decorations(false)
            .with_title("SFEN Bro"),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "SFEN Bro",
        options,
        Box::new(move |cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(SfenApp::new(sfen, assets_path, config)))
        }),
    )
}

fn read_sfen_from_clipboard() -> String {
    Clipboard::new()
        .and_then(|mut cb| cb.get_text())
        .unwrap_or_else(|_| {
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1".into()
        })
}
