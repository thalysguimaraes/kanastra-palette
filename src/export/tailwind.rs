use crate::export::{ExportOptions, Exporter};
use crate::{format_hex_color, ColorPalette, PALETTE_STEPS};
use anyhow::Result;
use palette::IntoColor;

pub struct TailwindV3Exporter;

impl Exporter for TailwindV3Exporter {
    fn export(&self, palette: &ColorPalette, options: &ExportOptions) -> Result<String> {
        let token_name = options.name.as_str();
        let mut js = format!(
            "// tailwind.config.js\nmodule.exports = {{\n  theme: {{\n    extend: {{\n      colors: {{\n        {}: {{\n",
            token_name
        );

        for step in PALETTE_STEPS {
            if let Some(color) = palette.steps.get(&step) {
                js.push_str(&format!(
                    "          {}: '{}',\n",
                    step,
                    format_hex_color(color)
                ));
            }
        }

        if let Some(color) = palette.steps.get(&palette.base_step) {
            js.push_str(&format!(
                "          DEFAULT: '{}',\n",
                format_hex_color(color)
            ));
        }

        js.push_str("        },\n      },\n    },\n  },\n};\n");
        Ok(js)
    }
}

pub struct TailwindV4Exporter;

impl Exporter for TailwindV4Exporter {
    fn export(&self, palette: &ColorPalette, options: &ExportOptions) -> Result<String> {
        let token_name = options.name.as_str();
        let mut css =
            String::from("/* app.css - Tailwind CSS v4 */\n@import \"tailwindcss\";\n\n@theme {\n");

        for step in PALETTE_STEPS {
            if let Some(color) = palette.steps.get(&step) {
                let oklch: palette::Oklch = (*color).into_color();
                let oklch_str = format!(
                    "oklch({:.3} {:.3} {:.3})",
                    oklch.l,
                    oklch.chroma,
                    oklch.hue.into_positive_degrees()
                );

                css.push_str(&format!(
                    "  --color-{}-{}: {}; /* {} */\n",
                    token_name,
                    step,
                    oklch_str,
                    format_hex_color(color)
                ));
            }
        }

        // Add default color reference
        if palette.steps.contains_key(&palette.base_step) {
            css.push_str(&format!(
                "  --color-{}: var(--color-{}-{});\n",
                token_name, token_name, palette.base_step
            ));
        }

        css.push_str("}\n");
        Ok(css)
    }
}

// Keep the old exporter for backward compatibility
pub struct TailwindExporter;

impl Exporter for TailwindExporter {
    fn export(&self, palette: &ColorPalette, options: &ExportOptions) -> Result<String> {
        TailwindV3Exporter.export(palette, options)
    }
}
