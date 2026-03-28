pub mod accessibility;
pub mod palette;

use ::palette::rgb::Srgb;
use anyhow::{bail, Result};

pub const PALETTE_STEPS: [u16; 11] = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

pub fn is_palette_step(step: u16) -> bool {
    PALETTE_STEPS.contains(&step)
}

pub fn parse_hex_color(hex: &str) -> Result<Srgb<f32>> {
    let hex = hex.trim().trim_start_matches('#');

    match hex.len() {
        3 => {
            let expanded = hex.chars().flat_map(|ch| [ch, ch]).collect::<String>();
            parse_hex_color(&expanded)
        }
        6 => {
            let r = parse_hex_channel(&hex[0..2])?;
            let g = parse_hex_channel(&hex[2..4])?;
            let b = parse_hex_channel(&hex[4..6])?;

            Ok(Srgb::new(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
            ))
        }
        _ => bail!("Invalid hex color format. Use #RGB or #RRGGBB"),
    }
}

pub fn format_hex_color(color: &Srgb<f32>) -> String {
    let (r, g, b) = rgb8(color);
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn rgb8(color: &Srgb<f32>) -> (u8, u8, u8) {
    (
        float_channel_to_u8(color.red),
        float_channel_to_u8(color.green),
        float_channel_to_u8(color.blue),
    )
}

fn parse_hex_channel(channel: &str) -> Result<u8> {
    Ok(u8::from_str_radix(channel, 16)?)
}

fn float_channel_to_u8(channel: f32) -> u8 {
    (channel.clamp(0.0, 1.0) * 255.0).round() as u8
}
