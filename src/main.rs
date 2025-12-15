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
use sfen::{detect_board_size, MINISHOGI_SIZE};

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
    let base_size = if board_size == MINISHOGI_SIZE {
        500.0
    } else {
        900.0
    };
    let window_size = base_size * config.scale_factor();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_size, window_size])
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
