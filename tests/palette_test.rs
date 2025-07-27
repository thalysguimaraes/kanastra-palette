use kanastra_palette_rs::{ColorPalette, PaletteGenerator, PaletteOptions};
use palette::Srgb;

#[test]
fn test_palette_generation() {
    let palette = ColorPalette::new("#0066CC").unwrap();
    assert_eq!(palette.steps.len(), 11);
    
    // Verify all expected steps are present
    let expected_steps = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
    for step in expected_steps {
        assert!(palette.steps.contains_key(&step));
    }
}

#[test]
fn test_hex_to_rgb_conversion() {
    let palette = ColorPalette::new("#FF0000").unwrap();
    let base = palette.base_color;
    
    // Red color should have max red, no green or blue
    assert!((base.red - 1.0).abs() < 0.01);
    assert!(base.green < 0.01);
    assert!(base.blue < 0.01);
}

#[test]
fn test_invalid_hex_color() {
    assert!(ColorPalette::new("invalid").is_err());
    assert!(ColorPalette::new("#GG0000").is_err());
    assert!(ColorPalette::new("#00").is_err());
}

#[test]
fn test_luminance_calculation() {
    let generator = PaletteGenerator::new();
    
    // Test with known color
    let blue = Srgb::new(0.0, 0.4, 0.8);
    let _palette = generator.generate_palette(&blue, &PaletteOptions::default());
    
    // Blue with these values should map to around step 500-600
    let base_step = generator.find_base_step(&blue);
    // Let's see what the actual value is
    println!("Base step for blue(0.0, 0.4, 0.8): {}", base_step);
    assert!(base_step >= 300 && base_step <= 700);
}

#[test] 
fn test_oklab_generation() {
    let generator = PaletteGenerator::new();
    let blue = Srgb::new(0.0, 0.4, 0.8);
    
    let options = PaletteOptions { use_oklab: true };
    let palette = generator.generate_palette(&blue, &options);
    
    assert_eq!(palette.len(), 11);
    
    // Verify colors get progressively lighter/darker
    let light = palette.get(&100).unwrap();
    let dark = palette.get(&900).unwrap();
    
    // Light color should have higher RGB values than dark
    assert!(light.red > dark.red);
    assert!(light.green > dark.green);
    assert!(light.blue > dark.blue);
}