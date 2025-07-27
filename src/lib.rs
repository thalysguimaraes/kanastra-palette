pub mod color;
pub mod export;
pub mod ui;

pub use color::palette::{PaletteGenerator, PaletteOptions, ColorStep};

use palette::Srgb;
use std::collections::HashMap;

pub struct ColorPalette {
    pub base_color: Srgb<f32>,
    pub steps: HashMap<u16, Srgb<f32>>,
}

impl ColorPalette {
    pub fn new(hex: &str) -> anyhow::Result<Self> {
        let base_color = Self::hex_to_rgb(hex)?;
        let generator = PaletteGenerator::new();
        let steps = generator.generate_palette(&base_color, &PaletteOptions::default());
        
        Ok(Self {
            base_color,
            steps,
        })
    }
    
    fn hex_to_rgb(hex: &str) -> anyhow::Result<Srgb<f32>> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color format");
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        Ok(Srgb::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        ))
    }
}