pub mod color;
pub mod export;
pub mod ui;

pub use color::palette::{ColorStep, PaletteAlgorithm, PaletteGenerator, PaletteOptions};
pub use color::{format_hex_color, is_palette_step, parse_hex_color, rgb8, PALETTE_STEPS};

use palette::Srgb;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub base_color: Srgb<f32>,
    pub base_step: u16,
    pub algorithm: PaletteAlgorithm,
    pub steps: BTreeMap<u16, Srgb<f32>>,
}

impl ColorPalette {
    pub fn new(hex: &str) -> anyhow::Result<Self> {
        Self::new_with_options(hex, &PaletteOptions::default())
    }

    pub fn new_with_options(hex: &str, options: &PaletteOptions) -> anyhow::Result<Self> {
        let base_color = parse_hex_color(hex)?;
        let generator = PaletteGenerator::new();
        let steps = generator.generate_palette(&base_color, options)?;

        Ok(Self {
            base_color,
            base_step: options.anchor_step,
            algorithm: options.algorithm,
            steps,
        })
    }
}
