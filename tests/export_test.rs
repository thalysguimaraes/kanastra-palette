use kanastra_palette_rs::export::{ExportFormat, ExportOptions};
use kanastra_palette_rs::{parse_hex_color, ColorPalette, PaletteAlgorithm, PALETTE_STEPS};
use serde_json::Value;
use std::collections::BTreeMap;

fn sample_palette(base_step: u16) -> ColorPalette {
    let values = [
        (50, "#f8fafc"),
        (100, "#f1f5f9"),
        (200, "#e2e8f0"),
        (300, "#cbd5e1"),
        (400, "#94a3b8"),
        (500, "#64748b"),
        (600, "#475569"),
        (700, "#334155"),
        (800, "#1e293b"),
        (900, "#0f172a"),
        (950, "#020617"),
    ];

    let steps = values
        .into_iter()
        .map(|(step, hex)| (step, parse_hex_color(hex).unwrap()))
        .collect::<BTreeMap<_, _>>();

    ColorPalette {
        base_color: *steps.get(&base_step).unwrap(),
        base_step,
        algorithm: PaletteAlgorithm::Oklch,
        steps,
    }
}

#[test]
fn test_tailwind_v3_export_has_valid_default_property() {
    let palette = sample_palette(600);
    let output = ExportFormat::TailwindV3
        .export_with_options(
            &palette,
            &ExportOptions {
                name: String::from("brand"),
            },
        )
        .unwrap();

    for step in PALETTE_STEPS {
        assert!(output.contains(&format!("          {}: '", step)));
    }

    assert!(output.contains("        brand: {\n"));
    assert!(output.contains("          DEFAULT: '#475569',\n"));
    assert!(!output.contains("500 DEFAULT"));
}

#[test]
fn test_tailwind_v4_export_uses_oklch_values_and_base_alias() {
    let palette = sample_palette(600);
    let output = ExportFormat::TailwindV4
        .export_with_options(
            &palette,
            &ExportOptions {
                name: String::from("brand"),
            },
        )
        .unwrap();

    assert!(output.contains("--color-brand-50: oklch("));
    assert!(output.contains("/* #f8fafc */"));
    assert!(output.contains("--color-brand: var(--color-brand-600);"));
}

#[test]
fn test_css_export_tracks_custom_name_and_default_step() {
    let palette = sample_palette(600);
    let output = ExportFormat::Css
        .export_with_options(
            &palette,
            &ExportOptions {
                name: String::from("brand"),
            },
        )
        .unwrap();

    assert!(output.contains("--color-brand-600: #475569;"));
    assert!(output.contains("--color-brand: var(--color-brand-600);"));
    assert!(output.contains("--color-brand-rgb: var(--color-brand-600-rgb);"));
}

#[test]
fn test_json_export_includes_metadata_and_default_step() {
    let palette = sample_palette(600);
    let output = ExportFormat::Json
        .export_with_options(
            &palette,
            &ExportOptions {
                name: String::from("brand"),
            },
        )
        .unwrap();
    let json: Value = serde_json::from_str(&output).unwrap();

    assert_eq!(json["name"], "brand");
    assert_eq!(json["defaultStep"], 600);
    assert_eq!(json["metadata"]["algorithm"], "oklch");
    assert_eq!(json["colors"]["600"]["hex"], "#475569");
}
