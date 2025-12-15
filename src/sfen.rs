//! SFEN parsing and piece representation

pub const STANDARD_SHOGI_SIZE: usize = 9;
pub const MINISHOGI_SIZE: usize = 5;

/// Convert SFEN piece character to kanji representation
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

/// Check if a piece belongs to gote (lowercase = gote)
pub fn is_gote(sfen_key: &str) -> bool {
    let ch = sfen_key.chars().last().unwrap_or(' ');
    ch.is_lowercase()
}

/// Detect board size from SFEN string
pub fn detect_board_size(sfen: &str) -> usize {
    let board_part = sfen.split_whitespace().next().unwrap_or(sfen);
    let row_count = board_part.split('/').count();
    if row_count == MINISHOGI_SIZE {
        MINISHOGI_SIZE
    } else {
        STANDARD_SHOGI_SIZE
    }
}

/// A piece on the board
#[derive(Clone)]
pub struct Piece {
    pub sfen_key: String,
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
