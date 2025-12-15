//! Board notation parsing and piece representation

pub const STANDARD_SHOGI_SIZE: usize = 9;
pub const CHESS_SIZE: usize = 8;
pub const MINISHOGI_SIZE: usize = 5;

/// Convert SFEN piece character to kanji representation (for shogi)
pub fn sfen_to_kanji(sfen_key: &str) -> &'static str {
    match sfen_key {
        "K" | "k" => "王",
        "R" => "飛",
        "r" => "飛",
        "+R" | "+r" => "龍",
        "B" => "角",
        "b" => "角",
        "+B" | "+b" => "馬",
        "G" | "g" => "金",
        "S" | "s" => "銀",
        "+S" | "+s" => "全",
        "N" | "n" => "桂",
        "+N" | "+n" => "圭",
        "L" | "l" => "香",
        "+L" | "+l" => "杏",
        "P" | "p" => "歩",
        "+P" | "+p" => "と",
        _ => "?",
    }
}

/// Convert FEN piece character to Unicode chess symbol
pub fn fen_to_unicode(fen_key: &str) -> &'static str {
    match fen_key {
        "K" => "♔",
        "Q" => "♕",
        "R" => "♖",
        "B" => "♗",
        "N" => "♘",
        "P" => "♙",
        "k" => "♚",
        "q" => "♛",
        "r" => "♜",
        "b" => "♝",
        "n" => "♞",
        "p" => "♟",
        _ => "?",
    }
}

/// Check if a piece belongs to gote/black (lowercase)
pub fn is_gote(sfen_key: &str) -> bool {
    let ch = sfen_key.chars().last().unwrap_or(' ');
    ch.is_lowercase()
}

/// Check if board size is chess
pub fn is_chess(board_size: usize) -> bool {
    board_size == CHESS_SIZE
}

/// Detect board size from SFEN/FEN string
pub fn detect_board_size(sfen: &str) -> usize {
    let board_part = sfen.split_whitespace().next().unwrap_or(sfen);
    let row_count = board_part.split('/').count();
    match row_count {
        MINISHOGI_SIZE => MINISHOGI_SIZE,
        CHESS_SIZE => CHESS_SIZE,
        _ => STANDARD_SHOGI_SIZE,
    }
}

/// A piece on the board
#[derive(Clone)]
pub struct Piece {
    pub sfen_key: String,
}

/// Pieces in hand for both players
#[derive(Clone, Default)]
pub struct Hand {
    /// Sente (black) pieces in hand - uppercase SFEN keys with counts
    pub sente: Vec<(String, u32)>,
    /// Gote (white) pieces in hand - lowercase SFEN keys with counts
    pub gote: Vec<(String, u32)>,
}

/// Parse pieces in hand from SFEN string (third field)
pub fn parse_hand(sfen: &str) -> Hand {
    let parts: Vec<&str> = sfen.split_whitespace().collect();
    let hand_str = parts.get(2).unwrap_or(&"-");

    if *hand_str == "-" {
        return Hand::default();
    }

    let mut sente = Vec::new();
    let mut gote = Vec::new();
    let mut count: u32 = 0;

    for ch in hand_str.chars() {
        if ch.is_ascii_digit() {
            count = count * 10 + ch.to_digit(10).unwrap();
        } else if ch.is_alphabetic() {
            let piece_count = if count == 0 { 1 } else { count };
            let key = ch.to_string();
            if ch.is_uppercase() {
                sente.push((key, piece_count));
            } else {
                gote.push((key, piece_count));
            }
            count = 0;
        }
    }

    Hand { sente, gote }
}

/// Parse SFEN board position into a 2D vector
pub fn parse_sfen(sfen: &str, board_size: usize) -> Vec<Vec<Option<Piece>>> {
    let mut board = vec![vec![None; board_size]; board_size];

    let board_part = sfen.split_whitespace().next().unwrap_or(sfen);
    let rows: Vec<&str> = board_part.split('/').collect();

    if rows.len() != board_size {
        return board;
    }

    for (row_idx, row) in rows.iter().enumerate() {
        let mut col = 0;
        let mut promoted = false;

        for ch in row.chars() {
            if col >= board_size {
                break;
            }

            match ch {
                '+' => {
                    promoted = true;
                    continue;
                }
                '1'..='9' => {
                    col += ch.to_digit(10).unwrap() as usize;
                    promoted = false;
                }
                _ if ch.is_alphabetic() => {
                    let sfen_key = if promoted {
                        format!("+{}", ch)
                    } else {
                        ch.to_string()
                    };
                    board[row_idx][col] = Some(Piece { sfen_key });
                    col += 1;
                    promoted = false;
                }
                _ => {
                    promoted = false;
                }
            }
        }
    }

    board
}
