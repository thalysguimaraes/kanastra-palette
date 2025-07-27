use crate::ColorPalette;
use crate::export::Exporter;
use anyhow::Result;
use palette::IntoColor;

pub struct TailwindV3Exporter;

impl Exporter for TailwindV3Exporter {
    fn export(&self, palette: &ColorPalette) -> Result<String> {
        let mut js = String::from("// tailwind.config.js\nmodule.exports = {\n  theme: {\n    extend: {\n      colors: {\n        primary: {\n");
        let steps = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
        
        for step in steps {
            if let Some(color) = palette.steps.get(&step) {
                let hex = format!("#{:02x}{:02x}{:02x}",
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                );
                
                let default_marker = if step == 500 { " DEFAULT:" } else { "" };
                js.push_str(&format!("         {}{} '{}',\n", step, default_marker, hex));
            }
        }
        
        js.push_str("        },\n      },\n    },\n  },\n};\n");
        Ok(js)
    }
}

pub struct TailwindV4Exporter;

impl Exporter for TailwindV4Exporter {
    fn export(&self, palette: &ColorPalette) -> Result<String> {
        let mut css = String::from("/* app.css - Tailwind CSS v4 */\n@import \"tailwindcss\";\n\n@theme {\n");
        let steps = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
        
        for step in steps {
            if let Some(color) = palette.steps.get(&step) {
                let hex = format!("#{:02x}{:02x}{:02x}",
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                );
                
                // Also provide OKLCH format for better color interpolation
                let oklch: palette::Oklch = (*color).into_color();
                let oklch_str = format!("oklch({:.3} {:.3} {:.3})", 
                    oklch.l,
                    oklch.chroma,
                    oklch.hue.into_positive_degrees()
                );
                
                css.push_str(&format!("  --color-primary-{}: {}; /* {} */\n", step, hex, oklch_str));
            }
        }
        
        // Add default color reference
        if palette.steps.contains_key(&500) {
            css.push_str("  --color-primary: var(--color-primary-500);\n");
        }
        
        css.push_str("}\n");
        Ok(css)
    }
}

// Keep the old exporter for backward compatibility
pub struct TailwindExporter;

impl Exporter for TailwindExporter {
    fn export(&self, palette: &ColorPalette) -> Result<String> {
        TailwindV3Exporter.export(palette)
    }
}