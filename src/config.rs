//! Configuration loading

use eframe::egui::Color32;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Shogi-specific configuration
#[derive(Deserialize, Default)]
pub struct ShogiConfig {
    /// Background color in HTML notation (e.g., "#F0D9B5")
    #[serde(default)]
    pub background: Option<String>,

    /// Grid line color in HTML notation
    #[serde(default)]
    pub grid_color: Option<String>,

    /// Text color for coordinates and kanji pieces in HTML notation
    #[serde(default)]
    pub text_color: Option<String>,

    /// Base path for piece images (relative to config file or absolute)
    #[serde(default)]
    pub assets_path: Option<String>,

    /// Piece texture mappings (SFEN key -> path relative to assets_path, or absolute)
    #[serde(default)]
    pub pieces: HashMap<String, String>,
}

/// Chess-specific configuration
#[derive(Deserialize, Default)]
pub struct ChessConfig {
    /// Light square color in HTML notation (default: white)
    #[serde(default)]
    pub light_squares: Option<String>,

    /// Dark square color in HTML notation (default: green)
    #[serde(default)]
    pub dark_squares: Option<String>,

    /// Text color for coordinates and pieces in HTML notation
    #[serde(default)]
    pub text_color: Option<String>,
}

/// Application configuration
#[derive(Deserialize, Default)]
pub struct Config {
    /// Window scale multiplier (default: 1.0)
    #[serde(default)]
    pub scale: Option<f32>,

    /// Shogi-specific settings
    #[serde(default)]
    pub shogi: ShogiConfig,

    /// Chess-specific settings
    #[serde(default)]
    pub chess: ChessConfig,
}

impl ShogiConfig {
    /// Parse background color from HTML notation to Color32
    pub fn background_color(&self) -> Color32 {
        self.background
            .as_ref()
            .and_then(|s| parse_html_color(s))
            .unwrap_or(Color32::from_rgb(240, 217, 181))
    }

    /// Parse grid color from HTML notation to Color32
    pub fn grid_color(&self) -> Color32 {
        self.grid_color
            .as_ref()
            .and_then(|s| parse_html_color(s))
            .unwrap_or(Color32::BLACK)
    }

    /// Parse text color from HTML notation to Color32
    pub fn text_color(&self) -> Color32 {
        self.text_color
            .as_ref()
            .and_then(|s| parse_html_color(s))
            .unwrap_or(Color32::BLACK)
    }
}

impl ChessConfig {
    /// Parse light square color from HTML notation to Color32
    pub fn light_square_color(&self) -> Color32 {
        self.light_squares
            .as_ref()
            .and_then(|s| parse_html_color(s))
            .unwrap_or(Color32::WHITE)
    }

    /// Parse dark square color from HTML notation to Color32
    pub fn dark_square_color(&self) -> Color32 {
        self.dark_squares
            .as_ref()
            .and_then(|s| parse_html_color(s))
            .unwrap_or(Color32::from_rgb(92, 122, 153))
    }

    /// Parse text color from HTML notation to Color32
    pub fn text_color(&self) -> Color32 {
        self.text_color
            .as_ref()
            .and_then(|s| parse_html_color(s))
            .unwrap_or(Color32::BLACK)
    }
}

impl Config {
    /// Get scale factor (default 1.0)
    pub fn scale_factor(&self) -> f32 {
        self.scale.unwrap_or(1.0).max(0.1)
    }
}

/// Parse HTML color notation (#RGB, #RRGGBB, or #RRGGBBAA)
fn parse_html_color(s: &str) -> Option<Color32> {
    let s = s.strip_prefix('#')?;
    match s.len() {
        3 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
            Some(Color32::from_rgb(r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some(Color32::from_rgb(r, g, b))
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some(Color32::from_rgba_unmultiplied(r, g, b, a))
        }
        _ => None,
    }
}

/// Load configuration from JSON file
pub fn load_config(path: &PathBuf) -> Config {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read config file: {}", e);
        std::process::exit(1);
    });
    serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Failed to parse config file: {}", e);
        std::process::exit(1);
    })
}
