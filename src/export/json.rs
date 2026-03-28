use crate::export::{ExportOptions, Exporter};
use crate::{format_hex_color, rgb8, ColorPalette, PALETTE_STEPS};
use anyhow::Result;
use palette::{Hsl, IntoColor, Oklch};
use serde_json::json;

pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, palette: &ColorPalette, options: &ExportOptions) -> Result<String> {
        let mut colors = serde_json::Map::new();

        for step in PALETTE_STEPS {
            if let Some(color) = palette.steps.get(&step) {
                let (r, g, b) = rgb8(color);
                let hex = format_hex_color(color);

                // Convert to HSL
                let hsl: Hsl = (*color).into_color();
                let h = hsl.hue.into_positive_degrees();
                let s = hsl.saturation * 100.0;
                let l = hsl.lightness * 100.0;

                // Convert to OKLCH
                let oklch: Oklch = (*color).into_color();
                let oklch_l = oklch.l;
                let oklch_c = oklch.chroma;
                let oklch_h = oklch.hue.into_positive_degrees();

                let color_data = json!({
                    "hex": hex,
                    "rgb": {
                        "r": r,
                        "g": g,
                        "b": b,
                        "string": format!("rgb({}, {}, {})", r, g, b)
                    },
                    "hsl": {
                        "h": format!("{:.0}", h),
                        "s": format!("{:.1}", s),
                        "l": format!("{:.1}", l),
                        "string": format!("hsl({:.0}, {:.1}%, {:.1}%)", h, s, l)
                    },
                    "oklch": {
                        "l": format!("{:.3}", oklch_l),
                        "c": format!("{:.3}", oklch_c),
                        "h": format!("{:.1}", oklch_h),
                        "string": format!("oklch({:.3} {:.3} {:.1})", oklch_l, oklch_c, oklch_h)
                    }
                });

                colors.insert(step.to_string(), color_data);
            }
        }

        let output = json!({
            "name": options.name,
            "defaultStep": palette.base_step,
            "colors": colors,
            "metadata": {
                "format": "v2",
                "algorithm": palette.algorithm.as_str(),
                "colorSpaces": ["hex", "rgb", "hsl", "oklch"],
                "generator": "kanastra-palette-rs"
            }
        });

        Ok(serde_json::to_string_pretty(&output)?)
    }
}
