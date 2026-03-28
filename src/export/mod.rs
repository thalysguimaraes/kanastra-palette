pub mod css;
pub mod design_tokens;
pub mod json;
pub mod tailwind;

use crate::ColorPalette;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub name: String,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            name: String::from("primary"),
        }
    }
}

pub trait Exporter {
    fn export(&self, palette: &ColorPalette, options: &ExportOptions) -> Result<String>;
}

pub enum ExportFormat {
    Css,
    DesignTokens,
    Json,
    TailwindV3,
    TailwindV4,
}

impl ExportFormat {
    pub fn export(&self, palette: &ColorPalette) -> Result<String> {
        self.export_with_options(palette, &ExportOptions::default())
    }

    pub fn export_with_options(
        &self,
        palette: &ColorPalette,
        options: &ExportOptions,
    ) -> Result<String> {
        match self {
            ExportFormat::Css => css::CssExporter.export(palette, options),
            ExportFormat::DesignTokens => {
                design_tokens::DesignTokensExporter.export(palette, options)
            }
            ExportFormat::Json => json::JsonExporter.export(palette, options),
            ExportFormat::TailwindV3 => tailwind::TailwindV3Exporter.export(palette, options),
            ExportFormat::TailwindV4 => tailwind::TailwindV4Exporter.export(palette, options),
        }
    }
}
