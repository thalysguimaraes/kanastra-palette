use crate::color::format_hex_color;
use palette::{LinSrgb, Srgb};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct ContrastCompliance {
    pub aa_normal: bool,
    pub aaa_normal: bool,
    pub aa_large: bool,
    pub aaa_large: bool,
}

impl ContrastCompliance {
    pub fn rating(&self) -> &'static str {
        if self.aaa_normal {
            "AAA"
        } else if self.aa_normal {
            "AA"
        } else if self.aa_large {
            "AA Large"
        } else {
            "Fail"
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ForegroundSuggestion {
    pub color: Srgb<f32>,
    pub contrast_ratio: f32,
    pub compliance: ContrastCompliance,
}

impl ForegroundSuggestion {
    pub fn hex(&self) -> String {
        format_hex_color(&self.color)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StepAccessibility {
    pub contrast_against_white: f32,
    pub contrast_against_black: f32,
    pub suggested_foreground: ForegroundSuggestion,
}

#[derive(Debug, Clone)]
pub struct PaletteAccessibility {
    pub default_foreground: ForegroundSuggestion,
    pub steps: BTreeMap<u16, StepAccessibility>,
}

impl PaletteAccessibility {
    pub fn new(steps: &BTreeMap<u16, Srgb<f32>>, base_step: u16) -> Self {
        let steps = steps
            .iter()
            .map(|(step, color)| (*step, step_accessibility(color)))
            .collect::<BTreeMap<_, _>>();

        let default_foreground = steps
            .get(&base_step)
            .map(|entry| entry.suggested_foreground)
            .unwrap_or_else(|| suggest_foreground(&Srgb::new(0.0, 0.0, 0.0)));

        Self {
            default_foreground,
            steps,
        }
    }
}

pub fn step_accessibility(background: &Srgb<f32>) -> StepAccessibility {
    let white = Srgb::new(1.0, 1.0, 1.0);
    let black = Srgb::new(0.0, 0.0, 0.0);

    let contrast_against_white = contrast_ratio(background, &white);
    let contrast_against_black = contrast_ratio(background, &black);
    let suggested_foreground = if contrast_against_white >= contrast_against_black {
        build_foreground_suggestion(white, contrast_against_white)
    } else {
        build_foreground_suggestion(black, contrast_against_black)
    };

    StepAccessibility {
        contrast_against_white,
        contrast_against_black,
        suggested_foreground,
    }
}

pub fn suggest_foreground(background: &Srgb<f32>) -> ForegroundSuggestion {
    step_accessibility(background).suggested_foreground
}

pub fn contrast_ratio(foreground: &Srgb<f32>, background: &Srgb<f32>) -> f32 {
    let foreground_luminance = relative_luminance(foreground);
    let background_luminance = relative_luminance(background);
    let (lighter, darker) = if foreground_luminance >= background_luminance {
        (foreground_luminance, background_luminance)
    } else {
        (background_luminance, foreground_luminance)
    };

    (lighter + 0.05) / (darker + 0.05)
}

fn build_foreground_suggestion(color: Srgb<f32>, contrast_ratio: f32) -> ForegroundSuggestion {
    ForegroundSuggestion {
        color,
        contrast_ratio,
        compliance: contrast_compliance(contrast_ratio),
    }
}

fn contrast_compliance(contrast_ratio: f32) -> ContrastCompliance {
    ContrastCompliance {
        aa_normal: contrast_ratio >= 4.5,
        aaa_normal: contrast_ratio >= 7.0,
        aa_large: contrast_ratio >= 3.0,
        aaa_large: contrast_ratio >= 4.5,
    }
}

fn relative_luminance(color: &Srgb<f32>) -> f32 {
    let lin: LinSrgb = color.into_linear();
    0.2126 * lin.red + 0.7152 * lin.green + 0.0722 * lin.blue
}
