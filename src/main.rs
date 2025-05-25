use anyhow::{Context, Result};
use clap::Parser;
use image::{Rgb, RgbImage};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// Generate identicons from hashed seed strings
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Seed text (username, email, etc.) to generate identicon from
    seed: String,

    /// Output file path [default: <seed_hash>.png]
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Size of the identicon in pixels
    #[arg(short('s'), long("size"), default_value_t = 420, value_parser = clap::value_parser!(u32).range(50..=2000))]
    image_size: u32,

    /// Grid size for the pattern
    #[arg(short('g'), long("grid"), default_value_t = 5, value_parser = clap::value_parser!(u32).range(3..=15))]
    grid_size: u32,

    /// Padding as a percentage of size
    #[arg(short, long, default_value_t = 8, value_parser = clap::value_parser!(u32).range(0..=25))]
    padding: u32,
}

fn hash_seed(seed: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.finalize().to_vec()
}

fn generate_color(hash: &[u8]) -> Rgb<u8> {
    // use first 3 bytes for RGB
    let r = hash[0];
    let g = hash[1];
    let b = hash[2];

    // normalize over range to ensure the color isn't too dark or too light
    let min = 50;
    let max = 200;

    Rgb([
        (min + (r as u32 * (max - min) / 255)) as u8,
        (min + (g as u32 * (max - min) / 255)) as u8,
        (min + (b as u32 * (max - min) / 255)) as u8,
    ])
}

fn generate_pattern(hash: &[u8], grid_size: u32) -> Vec<Vec<bool>> {
    let mut pattern = vec![vec![false; grid_size as usize]; grid_size as usize];

    // max cells to generate: 15 x 8 = 120
    // available bits from hash: 29 bytes * 8 = 232 bits
    let mut byte_idx = 3; // skip first 3 bytes used for color
    let mut bit_idx = 0;

    // only need to generate half the width, then mirror horizontally
    let half_width = grid_size.div_ceil(2);

    // iterate through reduced grid
    for y in 0..grid_size {
        for x in 0..half_width {
            if byte_idx < hash.len() {
                // choose corresponding bit in hash for cell
                let byte = hash[byte_idx];
                let bit = (byte >> bit_idx) & 1;

                // fill cell with bit
                pattern[y as usize][x as usize] = bit == 1;

                // mirror horizontally; skip mid on odd
                let mirror_x = grid_size - 1 - x;
                if mirror_x != x {
                    pattern[y as usize][mirror_x as usize] = bit == 1;
                }

                // increment indices
                bit_idx += 1;
                if bit_idx >= 8 {
                    bit_idx = 0;
                    byte_idx += 1;
                }
            }
        }
    }

    pattern
}

fn create_identicon(
    seed: &str,
    image_size: u32,
    grid_size: u32,
    padding_percent: u32,
) -> Result<RgbImage> {
    let hash = hash_seed(seed);
    let color = generate_color(&hash);
    let pattern = generate_pattern(&hash, grid_size);

    // create white background
    let mut img = RgbImage::from_pixel(image_size, image_size, Rgb([255, 255, 255]));

    // calculate padding based on percentage
    let padding = image_size * padding_percent / 100;
    let drawable_size = image_size - (2 * padding);

    // calculate cell size and additional padding to handle non-exact divisions
    let cell_size = drawable_size / grid_size;
    let total_used = cell_size * grid_size;
    let extra_padding = (drawable_size - total_used) / 2;
    let total_padding = padding + extra_padding;

    // draw the pattern; sparse iteration through filled cells
    for (y, row) in pattern.iter().enumerate() {
        for (x, &filled) in row.iter().enumerate() {
            if filled {
                let x_start = total_padding + (x as u32) * cell_size;
                let y_start = total_padding + (y as u32) * cell_size;

                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        let px = x_start + dx;
                        let py = y_start + dy;
                        img.put_pixel(px, py, color);
                    }
                }
            }
        }
    }

    Ok(img)
}

fn main() -> Result<()> {
    let args = Args::parse();

    // generate output path
    let output_path = args.output.unwrap_or_else(|| {
        let name = if args
            .seed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            && args.seed.len() <= 64
        {
            args.seed.clone()
        } else {
            format!("{:x}", Sha256::digest(args.seed.as_bytes()))
        };
        PathBuf::from(format!("{}.png", name))
    });

    // generate and save identicon
    println!("Generating identicon for seed: {}", args.seed);
    let img = create_identicon(&args.seed, args.image_size, args.grid_size, args.padding)
        .context("Failed to generate identicon")?;

    img.save(&output_path).context("Failed to save image")?;

    println!("Identicon saved to: {}", output_path.display());

    Ok(())
}
