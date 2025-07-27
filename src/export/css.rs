use crate::ColorPalette;
use crate::export::Exporter;
use anyhow::Result;

pub struct CssExporter;

impl Exporter for CssExporter {
    fn export(&self, palette: &ColorPalette) -> Result<String> {
        let mut css = String::from(":root {\n");
        let steps = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
        
        for step in steps {
            if let Some(color) = palette.steps.get(&step) {
                let r = (color.red * 255.0) as u8;
                let g = (color.green * 255.0) as u8;
                let b = (color.blue * 255.0) as u8;
                
                let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
                
                // Add hex value
                css.push_str(&format!("  --color-primary-{}: {};\n", step, hex));
                
                // Add RGB values for flexibility
                css.push_str(&format!("  --color-primary-{}-rgb: {} {} {};\n", step, r, g, b));
            }
        }
        
        // Add default references
        if palette.steps.contains_key(&500) {
            css.push_str("  \n");
            css.push_str("  /* Default color references */\n");
            css.push_str("  --color-primary: var(--color-primary-500);\n");
            css.push_str("  --color-primary-rgb: var(--color-primary-500-rgb);\n");
        }
        
        css.push_str("}\n\n");
        
        // Add example usage comments
        css.push_str("/* Usage examples:\n");
        css.push_str(" * background-color: var(--color-primary-500);\n");
        css.push_str(" * color: var(--color-primary-700);\n");
        css.push_str(" * background-color: rgb(var(--color-primary-500-rgb));\n");
        css.push_str(" * background-color: rgba(var(--color-primary-500-rgb) / 0.5);\n");
        css.push_str(" */\n");
        
        Ok(css)
    }
}