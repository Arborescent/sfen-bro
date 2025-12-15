# sfen-bro

SFEN clipboard viewer for shogi positions. Reads SFEN from clipboard and displays the board with piece textures.

Supports standard shogi (9x9) and minishogi (5x5).

## Usage

```bash
sfen-bro [path-to-config.json]
```

Copy an SFEN string to your clipboard, then run the command. A borderless window will open displaying the board. Dismiss it by pressing Escape or clicking outside the window (losing focus).

Without a config file, pieces are rendered using kanji characters. Gote pieces are displayed upside down.

## Configuration (optional)

Create a JSON config file to customize the display. Place the config file in the same directory as your piece images.

```json
{
  "background": "#F0D9B5",
  "scale": 1.0,
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
| `scale` | Window size multiplier | `1.0` |
| `pieces` | Map of SFEN characters to texture filenames | (none, uses kanji) |

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
