use crate::export::{ExportOptions, Exporter};
use crate::{format_hex_color, rgb8, ColorPalette, PALETTE_STEPS};
use anyhow::Result;

pub struct CssExporter;

impl Exporter for CssExporter {
    fn export(&self, palette: &ColorPalette, options: &ExportOptions) -> Result<String> {
        let mut css = String::from(":root {\n");
        let token_name = options.name.as_str();

        for step in PALETTE_STEPS {
            if let Some(color) = palette.steps.get(&step) {
                let (r, g, b) = rgb8(color);
                let hex = format_hex_color(color);

                // Add hex value
                css.push_str(&format!("  --color-{}-{}: {};\n", token_name, step, hex));

                // Add RGB values for flexibility
                css.push_str(&format!(
                    "  --color-{}-{}-rgb: {} {} {};\n",
                    token_name, step, r, g, b
                ));
            }
        }

        // Add default references
        if palette.steps.contains_key(&palette.base_step) {
            css.push_str("  \n");
            css.push_str("  /* Default color references */\n");
            css.push_str(&format!(
                "  --color-{}: var(--color-{}-{});\n",
                token_name, token_name, palette.base_step
            ));
            css.push_str(&format!(
                "  --color-{}-rgb: var(--color-{}-{}-rgb);\n",
                token_name, token_name, palette.base_step
            ));
        }

        css.push_str("}\n\n");

        // Add example usage comments
        css.push_str("/* Usage examples:\n");
        css.push_str(&format!(
            " * background-color: var(--color-{});\n",
            token_name
        ));
        css.push_str(&format!(" * color: var(--color-{}-700);\n", token_name));
        css.push_str(&format!(
            " * background-color: rgb(var(--color-{}-rgb));\n",
            token_name
        ));
        css.push_str(&format!(
            " * background-color: rgba(var(--color-{}-rgb) / 0.5);\n",
            token_name
        ));
        css.push_str(" */\n");

        Ok(css)
    }
}
