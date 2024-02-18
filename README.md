# Emoji POT
Fix emoji sprite sheet to POT (power of two) size.

# Basic Usage

```sh
emoji_pot -i sheet_google_32.png
```

# Advance Usage

```
Usage: emoji_pot [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>              The path of the sprite sheet to be fixed
  -g, --grid-size <GRID_SIZE>      The grid size of the sprite sheet [default: 30]
  -p, --padding <PADDING>          The padding of each sprite [default: 1]
  -o, --output-size <OUTPUT_SIZE>  The output size of the sprite sheet [default: 2048]
```
