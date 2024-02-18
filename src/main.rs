use anyhow::{Context, Result};
use image::{imageops, GenericImage, GenericImageView, ImageBuffer, RgbaImage};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// The path of the sprite sheet to be fixed.
    #[arg(short, long)]
    input: String,
    /// The grid size of the sprite sheet
    #[arg(short, long, default_value_t = 30)]
    grid_size: u32,
    /// The padding of each sprite
    #[arg(short, long, default_value_t = 1)]
    padding: u32,
    /// The output size of the sprite sheet
    #[arg(short, long, default_value_t = 2048)]
    output_size: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let args: Cli = Cli::parse();
    let input = args.input;
    let grid_size = args.grid_size;
    let padding = args.padding;
    let output_size = args.output_size;

    let mut image: RgbaImage = image::open(input.clone())?.to_rgba8();
    let file_path = std::path::Path::new(&input);
    let full_path = file_path.canonicalize().unwrap();
    let parent_path = full_path.parent().unwrap();
    let file_name = full_path.file_stem().unwrap();
    let ext = full_path.extension().unwrap();
    let pot_texture_path = format!(
        "{}/{}_pot.{}",
        parent_path.to_str().unwrap(),
        file_name.to_str().unwrap(),
        ext.to_str().unwrap()
    );

    let src_width = image.width();
    let src_height = image.height();
    let src_padding = 1usize;
    let src_grid_size = 32usize;

    let mut pot_texture = ImageBuffer::new(output_size, output_size);

    println!("Saving pot texture to {}", pot_texture_path);

    let mut row = 0;
    for y in (0..src_height).step_by(src_grid_size + src_padding * 2) {
        let mut col = 0;
        for x in (0..src_width).step_by(src_grid_size + src_padding * 2) {
            let emoji_x = x + src_padding as u32;
            let emoji_y = y + src_padding as u32;
            let mut emoji = image
                .view(emoji_x, emoji_y, src_grid_size as u32, src_grid_size as u32)
                .to_image();
            emoji = imageops::resize(&emoji, grid_size, grid_size, imageops::FilterType::Lanczos3);
            let dst_x = (grid_size + padding * 2) * col + padding;
            let dst_y = (grid_size + padding * 2) * row + padding;
            pot_texture.copy_from(&emoji, dst_x, dst_y)?;
            col += 1;
        }
        row += 1;
    }

    pot_texture
        .save(pot_texture_path)
        .context("Failed to save pot texture")?;

    println!("Save success");
    Ok(())
}
