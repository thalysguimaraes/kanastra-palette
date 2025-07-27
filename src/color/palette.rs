use palette::{Srgb, IntoColor, LinSrgb, Oklch, Oklab, Clamp, OklabHue};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct ColorStep {
    pub value: u16,
    pub color: Srgb<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct PaletteOptions {
    pub use_oklab: bool,
}

pub struct PaletteGenerator {
    steps: Vec<u16>,
}

impl PaletteGenerator {
    pub fn new() -> Self {
        Self {
            steps: vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950],
        }
    }
    
    pub fn generate_palette(&self, base_color: &Srgb<f32>, options: &PaletteOptions) -> HashMap<u16, Srgb<f32>> {
        let base_step = self.find_base_step(base_color);
        let mut palette = HashMap::new();
        
        for &step in &self.steps {
            let color = if step == base_step {
                *base_color
            } else {
                self.generate_step_color(base_color, step, base_step, options)
            };
            
            palette.insert(step, color);
        }
        
        palette
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
    
    fn generate_step_color(&self, base_color: &Srgb<f32>, target_step: u16, base_step: u16, _options: &PaletteOptions) -> Srgb<f32> {
        // Always use OKLCH for better perceptual uniformity
        self.generate_with_oklch(base_color, target_step, base_step)
    }
    
    fn generate_with_oklch(&self, base_color: &Srgb<f32>, target_step: u16, base_step: u16) -> Srgb<f32> {
        let oklab: Oklab = (*base_color).into_color();
        let oklch: Oklch = oklab.into_color();
        
        // Get target lightness based on step
        let target_lightness = self.step_to_oklch_lightness(target_step);
        
        // Dynamic chroma adjustment based on both lightness and hue
        let base_chroma = oklch.chroma;
        let hue_degrees = oklch.hue.into_positive_degrees();
        
        // Calculate chroma curve - different hues have different optimal chroma ranges
        let hue_chroma_factor = self.calculate_hue_chroma_factor(hue_degrees);
        
        // Calculate lightness-based chroma adjustment
        let lightness_chroma_curve = self.calculate_lightness_chroma_curve(target_step, target_lightness);
        
        // Combine factors for final chroma
        let adjusted_chroma = base_chroma * lightness_chroma_curve * hue_chroma_factor;
        
        // Apply gamut mapping - cap chroma to ensure colors stay within sRGB
        let max_chroma = self.calculate_max_chroma(target_lightness, hue_degrees);
        let final_chroma = adjusted_chroma.min(max_chroma);
        
        // Apply subtle hue shifting for more natural palette
        let hue_shift = self.calculate_hue_shift(target_step, base_step, hue_degrees);
        let adjusted_hue = OklabHue::from_degrees(hue_degrees + hue_shift);
        
        // Create new color with adjusted values
        let new_oklch = Oklch::new(target_lightness, final_chroma, adjusted_hue);
        
        // Convert back to sRGB and clamp to valid range
        let new_oklab: Oklab = new_oklch.into_color();
        let srgb: Srgb<f32> = new_oklab.into_color();
        
        // Clamp to ensure we're in valid RGB range
        srgb.clamp()
    }
    
    fn step_to_oklch_lightness(&self, step: u16) -> f32 {
        // Non-linear lightness distribution based on modern design systems
        // Values are perceptually balanced for better contrast ratios
        match step {
            50 => 0.978,   // Near white (97.8%)
            100 => 0.936,  // Very light (93.6%)
            200 => 0.881,  // Light (88.1%)
            300 => 0.827,  // Light-medium (82.7%)
            400 => 0.742,  // Medium-light (74.2%)
            500 => 0.648,  // Medium (64.8%)
            600 => 0.573,  // Medium-dark (57.3%)
            700 => 0.469,  // Dark (46.9%)
            800 => 0.394,  // Very dark (39.4%)
            900 => 0.320,  // Near black (32.0%)
            950 => 0.238,  // Almost black (23.8%)
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
            h if h < 60.0 => 0.85 + 0.15 * (h / 60.0),           // Red to yellow
            h if h < 120.0 => 1.0 - 0.2 * ((h - 60.0) / 60.0),  // Yellow to green
            h if h < 180.0 => 0.8 + 0.1 * ((h - 120.0) / 60.0),  // Green to cyan
            h if h < 240.0 => 0.9 + 0.1 * ((h - 180.0) / 60.0),  // Cyan to blue
            h if h < 300.0 => 1.0,                                // Blue to magenta (max chroma)
            _ => 0.9 + 0.1 * ((360.0 - normalized_hue) / 60.0),               // Magenta to red
        }
    }
    
    fn calculate_lightness_chroma_curve(&self, step: u16, lightness: f32) -> f32 {
        // Non-linear chroma adjustment based on lightness
        // Provides natural-looking color progression
        match step {
            50 => 0.15,    // Very light colors need minimal chroma
            100 => 0.25,   
            200 => 0.45,   
            300 => 0.70,   
            400 => 0.90,   
            500 => 1.00,   // Base color maintains full chroma
            600 => 0.95,   
            700 => 0.85,   
            800 => 0.70,   
            900 => 0.50,   
            950 => 0.30,   // Very dark colors need reduced chroma
            _ => {
                // Smooth interpolation for other values
                if lightness > 0.8 {
                    0.15 + (0.85 - lightness) * 2.0
                } else if lightness < 0.3 {
                    0.3 + lightness * 1.5
                } else {
                    0.7 + (lightness - 0.3) * 0.6
                }
            }
        }
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
            h if h < 60.0 || h > 300.0 => shift_factor * max_shift * 0.5,  // Reds/magentas: less shift
            h if h >= 60.0 && h < 150.0 => shift_factor * max_shift * 0.7, // Yellows/greens: moderate shift
            _ => shift_factor * max_shift,                                   // Blues: full shift
        };
        
        // Lighter colors (lower step numbers) get positive (warmer) shift
        // Darker colors (higher step numbers) get negative (cooler) shift
        -hue_specific_shift
    }
    
    fn calculate_max_chroma(&self, lightness: f32, hue: f32) -> f32 {
        // Approximate maximum chroma values for sRGB gamut
        // These values ensure colors stay within displayable range
        
        // Very light or very dark colors have limited chroma range
        if lightness > 0.9 || lightness < 0.1 {
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
            h if h < 30.0 || h > 330.0 => 0.3,   // Reds have lower max chroma
            h if h >= 30.0 && h < 90.0 => 0.28,  // Yellows/oranges
            h if h >= 90.0 && h < 150.0 => 0.32, // Greens
            h if h >= 150.0 && h < 210.0 => 0.35, // Cyans
            h if h >= 210.0 && h < 270.0 => 0.37, // Blues can handle most chroma
            _ => 0.32,                             // Purples
        };
        
        lightness_factor * hue_factor
    }
}

impl Default for PaletteGenerator {
    fn default() -> Self {
        Self::new()
    }
}