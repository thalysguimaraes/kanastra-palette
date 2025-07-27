use crate::ColorPalette;
use crate::export::Exporter;
use anyhow::Result;
use serde_json::json;
use palette::{IntoColor, Hsl, Oklch};

pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, palette: &ColorPalette) -> Result<String> {
        let mut colors = serde_json::Map::new();
        let steps = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
        
        for step in steps {
            if let Some(color) = palette.steps.get(&step) {
                let r = (color.red * 255.0) as u8;
                let g = (color.green * 255.0) as u8;
                let b = (color.blue * 255.0) as u8;
                
                let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
                
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
            "name": "primary",
            "colors": colors,
            "metadata": {
                "format": "v2",
                "colorSpaces": ["hex", "rgb", "hsl", "oklch"],
                "generator": "kanastra-palette-rs"
            }
        });
        
        Ok(serde_json::to_string_pretty(&output)?)
    }
}