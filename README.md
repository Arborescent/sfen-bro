# sfen-bro

Quick preview tool for SFEN shogi positions. Supports standard shogi (9x9) and minishogi (5x5).

## Usage

1. Copy an SFEN string to your clipboard
2. Run `sfen-bro [path-to-config.json]`
3. A borderless window opens displaying the board
4. Dismiss with Escape or by clicking outside the window (losing focus)

Without a config file, pieces are rendered using kanji characters. Gote pieces are displayed upside down.

## Configuration (optional)

Create a JSON config file to customize the display. Place the config file in the same directory as your piece images.

```json
{
  "background": "#F0D9B5",
  "scale": 1.0,
  "assets_path": "images/pieces",
  "pieces": {
    "K": "sente_king.png",
    "k": "gote_king.png",
    "G": "sente_gold.png",
    "g": "gote_gold.png",
    "S": "sente_silver.png",
    "+S": "sente_promoted_silver.png",
    "s": "gote_silver.png",
    "+s": "gote_promoted_silver.png",
    "N": "sente_knight.png",
    "+N": "sente_promoted_knight.png",
    "n": "gote_knight.png",
    "+n": "gote_promoted_knight.png",
    "L": "sente_lance.png",
    "+L": "sente_promoted_lance.png",
    "l": "gote_lance.png",
    "+l": "gote_promoted_lance.png",
    "B": "sente_bishop.png",
    "+B": "sente_horse.png",
    "b": "gote_bishop.png",
    "+b": "gote_horse.png",
    "R": "sente_rook.png",
    "+R": "sente_dragon.png",
    "r": "gote_rook.png",
    "+r": "gote_dragon.png",
    "P": "sente_pawn.png",
    "+P": "sente_tokin.png",
    "p": "gote_pawn.png",
    "+p": "gote_tokin.png"
  }
}
```

### Configuration options

| Option | Description | Default |
|--------|-------------|---------|
| `background` | Board background color in HTML notation (#RGB, #RRGGBB, or #RRGGBBAA) | `#F0D9B5` |
| `grid_color` | Grid line color in HTML notation | `#000000` |
| `text_color` | Text color for coordinates and kanji pieces | `#000000` |
| `scale` | Window size multiplier | `1.0` |
| `assets_path` | Base path for piece images (relative to config file or absolute) | config file directory |
| `pieces` | Map of SFEN characters to image paths (relative to assets_path or absolute) | (none, uses kanji) |

### SFEN piece characters

| Character | Piece |
|-----------|-------|
| K/k | King |
| R/r | Rook |
| B/b | Bishop |
| G/g | Gold |
| S/s | Silver |
| N/n | Knight |
| L/l | Lance |
| P/p | Pawn |

- Uppercase = Sente (black, moves first)
- Lowercase = Gote (white)
- `+` prefix = Promoted piece

## Building

```bash
cargo build --release
```

## License

MIT
