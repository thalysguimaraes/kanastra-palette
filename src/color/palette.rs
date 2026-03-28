use crate::color::{is_palette_step, PALETTE_STEPS};
use anyhow::{bail, Result};
use palette::{Clamp, IntoColor, LinSrgb, Oklab, OklabHue, Oklch, Srgb};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct ColorStep {
    pub value: u16,
    pub color: Srgb<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaletteAlgorithm {
    #[default]
    Oklch,
}

impl PaletteAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Oklch => "oklch",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PaletteOptions {
    pub anchor_step: u16,
    pub algorithm: PaletteAlgorithm,
}

impl Default for PaletteOptions {
    fn default() -> Self {
        Self {
            anchor_step: 500,
            algorithm: PaletteAlgorithm::Oklch,
        }
    }
}

pub struct PaletteGenerator;

impl PaletteGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_palette(
        &self,
        base_color: &Srgb<f32>,
        options: &PaletteOptions,
    ) -> Result<BTreeMap<u16, Srgb<f32>>> {
        if !is_palette_step(options.anchor_step) {
            bail!(
                "Unsupported base step {}. Supported steps: {:?}",
                options.anchor_step,
                PALETTE_STEPS
            );
        }

        let mut palette = BTreeMap::new();

        for step in PALETTE_STEPS {
            let color = if step == options.anchor_step {
                *base_color
            } else {
                self.generate_step_color(base_color, step, options)
            };

            palette.insert(step, color);
        }

        Ok(palette)
    }

    pub fn find_base_step(&self, color: &Srgb<f32>) -> u16 {
        let luminance = self.calculate_relative_luminance(color);

        let step_luminances = [
            (50, 0.95),
            (100, 0.90),
            (200, 0.82),
            (300, 0.68),
            (400, 0.50),
            (500, 0.36),
            (600, 0.23),
            (700, 0.13),
            (800, 0.07),
            (900, 0.03),
            (950, 0.01),
        ];

        let mut closest_step = 500;
        let mut min_diff = f32::MAX;

        for (step, target_lum) in step_luminances {
            let diff = (luminance - target_lum).abs();
            if diff < min_diff {
                min_diff = diff;
                closest_step = step;
            }
        }

        closest_step
    }

    fn calculate_relative_luminance(&self, color: &Srgb<f32>) -> f32 {
        let lin: LinSrgb = color.into_linear();

        0.2126 * lin.red + 0.7152 * lin.green + 0.0722 * lin.blue
    }

    fn generate_step_color(
        &self,
        base_color: &Srgb<f32>,
        target_step: u16,
        options: &PaletteOptions,
    ) -> Srgb<f32> {
        match options.algorithm {
            PaletteAlgorithm::Oklch => {
                self.generate_with_oklch(base_color, target_step, options.anchor_step)
            }
        }
    }

    fn generate_with_oklch(
        &self,
        base_color: &Srgb<f32>,
        target_step: u16,
        anchor_step: u16,
    ) -> Srgb<f32> {
        let oklab: Oklab = (*base_color).into_color();
        let oklch: Oklch = oklab.into_color();

        let target_lightness = self.calculate_target_lightness(oklch.l, target_step, anchor_step);

        // Dynamic chroma adjustment based on both lightness and hue
        let base_chroma = oklch.chroma;
        let hue_degrees = oklch.hue.into_positive_degrees();

        // Calculate chroma curve - different hues have different optimal chroma ranges
        let hue_chroma_factor = self.calculate_hue_chroma_factor(hue_degrees);

        // Reduce chroma as we move away from the anchor lightness to keep steps stable.
        let lightness_chroma_curve =
            self.calculate_lightness_chroma_curve(target_lightness, oklch.l);

        // Combine factors for final chroma
        let adjusted_chroma = base_chroma * lightness_chroma_curve * hue_chroma_factor;

        // Apply gamut mapping - cap chroma to ensure colors stay within sRGB
        let max_chroma = self.calculate_max_chroma(target_lightness, hue_degrees);
        let final_chroma = adjusted_chroma.min(max_chroma);

        // Apply subtle hue shifting for more natural palette
        let hue_shift = self.calculate_hue_shift(target_step, anchor_step, hue_degrees);
        let adjusted_hue = OklabHue::from_degrees(hue_degrees + hue_shift);

        // Create new color with adjusted values
        let new_oklch = Oklch::new(target_lightness, final_chroma, adjusted_hue);

        // Convert back to sRGB and clamp to valid range
        let new_oklab: Oklab = new_oklch.into_color();
        let srgb: Srgb<f32> = new_oklab.into_color();

        // Clamp to ensure we're in valid RGB range
        srgb.clamp()
    }

    fn calculate_target_lightness(
        &self,
        base_lightness: f32,
        target_step: u16,
        anchor_step: u16,
    ) -> f32 {
        let relative_offset =
            self.step_to_oklch_lightness(target_step) - self.step_to_oklch_lightness(anchor_step);

        (base_lightness + relative_offset).clamp(0.02, 0.98)
    }

    fn step_to_oklch_lightness(&self, step: u16) -> f32 {
        // Non-linear lightness distribution based on modern design systems
        // Values are perceptually balanced for better contrast ratios
        match step {
            50 => 0.978,  // Near white (97.8%)
            100 => 0.936, // Very light (93.6%)
            200 => 0.881, // Light (88.1%)
            300 => 0.827, // Light-medium (82.7%)
            400 => 0.742, // Medium-light (74.2%)
            500 => 0.648, // Medium (64.8%)
            600 => 0.573, // Medium-dark (57.3%)
            700 => 0.469, // Dark (46.9%)
            800 => 0.394, // Very dark (39.4%)
            900 => 0.320, // Near black (32.0%)
            950 => 0.238, // Almost black (23.8%)
            _ => 0.648,
        }
    }

    fn calculate_hue_chroma_factor(&self, hue: f32) -> f32 {
        // Different hues have different maximum chroma in sRGB
        // This function provides a multiplier based on the hue
        let normalized_hue = hue % 360.0;

        // Use a smooth curve to adjust chroma based on hue
        // Blues and purples can handle more chroma, yellows need less
        match normalized_hue {
            h if h < 60.0 => 0.85 + 0.15 * (h / 60.0), // Red to yellow
            h if h < 120.0 => 1.0 - 0.2 * ((h - 60.0) / 60.0), // Yellow to green
            h if h < 180.0 => 0.8 + 0.1 * ((h - 120.0) / 60.0), // Green to cyan
            h if h < 240.0 => 0.9 + 0.1 * ((h - 180.0) / 60.0), // Cyan to blue
            h if h < 300.0 => 1.0,                     // Blue to magenta (max chroma)
            _ => 0.9 + 0.1 * ((360.0 - normalized_hue) / 60.0), // Magenta to red
        }
    }

    fn calculate_lightness_chroma_curve(&self, target_lightness: f32, base_lightness: f32) -> f32 {
        let distance_factor =
            (1.0 - ((target_lightness - base_lightness).abs() / 0.75)).clamp(0.35, 1.0);
        let edge_factor = (1.0 - (((target_lightness - 0.5).abs() * 2.0) * 0.55)).clamp(0.30, 1.0);

        (distance_factor * edge_factor).clamp(0.18, 1.0)
    }

    fn calculate_hue_shift(&self, target_step: u16, base_step: u16, base_hue: f32) -> f32 {
        // Subtle hue shifting for more natural palettes
        // Lighter shades shift warmer, darker shades shift cooler
        let step_difference = target_step as i16 - base_step as i16;
        let max_shift = 8.0; // Maximum hue shift in degrees

        // Calculate shift based on step difference
        let shift_factor = (step_difference as f32) / 450.0; // Normalize to -1 to 1 range

        // Apply different shift patterns based on base hue
        let hue_specific_shift = match base_hue {
            h if !(60.0..=300.0).contains(&h) => shift_factor * max_shift * 0.5, // Reds/magentas: less shift
            h if (60.0..150.0).contains(&h) => shift_factor * max_shift * 0.7, // Yellows/greens: moderate shift
            _ => shift_factor * max_shift,                                     // Blues: full shift
        };

        // Lighter colors (lower step numbers) get positive (warmer) shift
        // Darker colors (higher step numbers) get negative (cooler) shift
        -hue_specific_shift
    }

    fn calculate_max_chroma(&self, lightness: f32, hue: f32) -> f32 {
        // Approximate maximum chroma values for sRGB gamut
        // These values ensure colors stay within displayable range

        // Very light or very dark colors have limited chroma range
        if !(0.1..=0.9).contains(&lightness) {
            return 0.05;
        }

        // Calculate base max chroma based on lightness
        let lightness_factor = if lightness < 0.5 {
            lightness * 2.0
        } else {
            2.0 - (lightness * 2.0)
        };

        // Adjust based on hue - some hues can handle more chroma
        let hue_factor = match hue {
            h if !(30.0..=330.0).contains(&h) => 0.3, // Reds have lower max chroma
            h if (30.0..90.0).contains(&h) => 0.28,   // Yellows/oranges
            h if (90.0..150.0).contains(&h) => 0.32,  // Greens
            h if (150.0..210.0).contains(&h) => 0.35, // Cyans
            h if (210.0..270.0).contains(&h) => 0.37, // Blues can handle most chroma
            _ => 0.32,                                // Purples
        };

        lightness_factor * hue_factor
    }
}

impl Default for PaletteGenerator {
    fn default() -> Self {
        Self::new()
    }
}
