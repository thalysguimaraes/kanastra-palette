pub mod css;
pub mod json;
pub mod tailwind;

use crate::ColorPalette;
use anyhow::Result;

pub trait Exporter {
    fn export(&self, palette: &ColorPalette) -> Result<String>;
}

pub enum ExportFormat {
    Css,
    Json,
    TailwindV3,
    TailwindV4,
}

impl ExportFormat {
    pub fn export(&self, palette: &ColorPalette) -> Result<String> {
        match self {
            ExportFormat::Css => css::CssExporter.export(palette),
            ExportFormat::Json => json::JsonExporter.export(palette),
            ExportFormat::TailwindV3 => tailwind::TailwindV3Exporter.export(palette),
            ExportFormat::TailwindV4 => tailwind::TailwindV4Exporter.export(palette),
        }
    }
}