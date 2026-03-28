use kanastra_palette_rs::{
    format_hex_color, parse_hex_color, suggest_foreground, ColorPalette, PaletteAlgorithm,
    PaletteOptions, PALETTE_STEPS,
};
use palette::{IntoColor, Oklch};

#[test]
fn test_palette_generation_preserves_input_at_default_step() {
    let palette = ColorPalette::new("#0066CC").unwrap();

    assert_eq!(palette.steps.len(), PALETTE_STEPS.len());
    assert_eq!(palette.base_step, 500);
    assert_eq!(format_hex_color(&palette.base_color), "#0066cc");
    assert_eq!(
        format_hex_color(palette.steps.get(&500).unwrap()),
        "#0066cc"
    );
}

#[test]
fn test_short_hex_color_is_supported_by_core_parser() {
    let palette = ColorPalette::new("#abc").unwrap();

    assert_eq!(format_hex_color(&palette.base_color), "#aabbcc");
    assert_eq!(
        format_hex_color(palette.steps.get(&500).unwrap()),
        "#aabbcc"
    );
}

#[test]
fn test_custom_anchor_step_preserves_input_color() {
    let palette = ColorPalette::new_with_options(
        "#FF0000",
        &PaletteOptions {
            anchor_step: 600,
            algorithm: PaletteAlgorithm::Oklch,
        },
    )
    .unwrap();

    assert_eq!(palette.base_step, 600);
    assert_eq!(format_hex_color(&palette.base_color), "#ff0000");
    assert_eq!(
        format_hex_color(palette.steps.get(&600).unwrap()),
        "#ff0000"
    );
}

#[test]
fn test_invalid_hex_color() {
    assert!(ColorPalette::new("invalid").is_err());
    assert!(ColorPalette::new("#GG0000").is_err());
    assert!(ColorPalette::new("#00").is_err());
}

#[test]
fn test_lightness_is_monotonic_across_steps() {
    let palette = ColorPalette::new("#0066CC").unwrap();
    let mut previous_lightness = f32::MAX;

    for step in PALETTE_STEPS {
        let color = palette.steps.get(&step).unwrap();
        let oklch: Oklch = (*color).into_color();
        assert!(
            oklch.l <= previous_lightness + 0.001,
            "step {step} should not be lighter than the previous step"
        );
        previous_lightness = oklch.l;
    }
}

#[test]
fn test_accessibility_prefers_white_on_dark_backgrounds() {
    let background = parse_hex_color("#334155").unwrap();
    let suggestion = suggest_foreground(&background);

    assert_eq!(suggestion.hex(), "#ffffff");
    assert!(suggestion.compliance.aa_normal);
}

#[test]
fn test_accessibility_prefers_black_on_light_backgrounds() {
    let background = parse_hex_color("#f8fafc").unwrap();
    let suggestion = suggest_foreground(&background);

    assert_eq!(suggestion.hex(), "#000000");
    assert!(suggestion.compliance.aaa_normal);
}
