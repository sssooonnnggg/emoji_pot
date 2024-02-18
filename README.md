# Emoji POT
Fix emoji sprite sheet to POT (power of two) size to fix TextMeshPro issues in Unity.
Source sprite sheet can be found at https://github.com/iamcal/emoji-data

# Basic Usage

```sh
emoji_pot -i sheet_google_32.png
```

If successful, the output will be a file named `sheet_google_32_pot.png` in the same directory as the input file.
Input sprite sheet should be any sprite sheet with 32x32 emoji images, the output file will be a 2048x2048 sprite sheet with 30x30 emoji images by default.

# Advance Usage

```
Usage: emoji_pot [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>              The path of the sprite sheet to be fixed
  -g, --grid-size <GRID_SIZE>      The grid size of the sprite sheet [default: 30]
  -p, --padding <PADDING>          The padding of each sprite [default: 1]
  -o, --output-size <OUTPUT_SIZE>  The output size of the sprite sheet [default: 2048]
```
