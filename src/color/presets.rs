#[derive(Debug, Clone, Copy)]
pub struct PalettePreset {
    pub name: &'static str,
    pub hex: &'static str,
    pub description: &'static str,
}

pub const PALETTE_PRESETS: [PalettePreset; 6] = [
    PalettePreset {
        name: "kanastra-green",
        hex: "#22C55E",
        description: "Bright product green with strong accessibility headroom",
    },
    PalettePreset {
        name: "royal-violet",
        hex: "#7953E0",
        description: "Bold violet close to the app's original default seed",
    },
    PalettePreset {
        name: "ocean-blue",
        hex: "#0EA5E9",
        description: "Clean blue for fintech and infrastructure palettes",
    },
    PalettePreset {
        name: "ember-orange",
        hex: "#F97316",
        description: "Warm orange for accent-heavy brands",
    },
    PalettePreset {
        name: "rose-magenta",
        hex: "#E11D48",
        description: "High-energy magenta for expressive identities",
    },
    PalettePreset {
        name: "slate-neutral",
        hex: "#334155",
        description: "Neutral slate suited for sober UIs and enterprise brands",
    },
];

pub fn get_preset(name: &str) -> Option<&'static PalettePreset> {
    PALETTE_PRESETS
        .iter()
        .find(|preset| preset.name.eq_ignore_ascii_case(name))
}
