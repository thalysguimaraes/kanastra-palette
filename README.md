# Kanastra Palette 🎨

![Kanastra Palette Banner](public/banner-image.jpg)

A blazing-fast terminal color palette generator built with Rust and Ratatui. Generate beautiful, accessible color palettes with advanced color science algorithms and export them in multiple formats including Tailwind CSS v3/v4.

## ✨ Features

### 🎯 Smart Palette Generation
- **11-step color scales** (50-950) using perceptual color algorithms
- **Input color preserved at step `500` by default** with configurable anchoring via `--base-step`
- **OKLCH color space** for superior perceptual uniformity
- **Dynamic lightness and chroma curves** for natural-looking palettes
- **Hue shifting** for more vibrant color scales
- **WCAG contrast hints** with suggested `on-primary` foreground tokens

### 🚀 Modern Export Formats
- **CSS Custom Properties** with RGB values for alpha support
- **Design Tokens JSON** for token pipelines and multi-platform systems
- **Tailwind CSS v3** - Traditional JS config format
- **Tailwind CSS v4** - New CSS `@theme` format with real OKLCH tokens
- **JSON** with hex, RGB, HSL, OKLCH, and accessibility metadata
- **Automation-friendly CLI** with `stdout` and file export support

### 💻 Beautiful Terminal UI
- Interactive color input with live preview
- Keyboard-driven interface
- Real-time palette visualization
- Clipboard integration for quick exports

## 🏗️ Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│                 │     │                  │     │                 │
│   Terminal UI   │────▶│  Color Engine    │────▶│  Export Module  │
│   (Ratatui)     │     │  (OKLCH/Lab)     │     │  (CSS/JS/JSON)  │
│                 │     │                  │     │                 │
└─────────────────┘     └──────────────────┘     └─────────────────┘
         │                       │                         │
         ▼                       ▼                         ▼
    User Input            Palette Generation         Multiple Formats
```

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/thalysguimaraes/kanastra-palette
cd kanastra-palette

# Build and run
cargo run

# Or install globally
cargo install --path .
```

### Basic Usage

1. **Launch the app**:
   ```bash
   cargo run
   ```

2. **Enter a color** (e.g., `#7953E0`)

3. **Press Enter** to generate palette

4. **Export your palette**:
   - `[C]` - CSS Variables
   - `[D]` - Design Tokens JSON
   - `[3]` - Tailwind v3
   - `[4]` - Tailwind v4
   - `[J]` - JSON

## 📖 Usage Examples

### Interactive Mode

```bash
# Start with default color
cargo run

# Start with specific color
cargo run -- --color "#FF6B6B"
```

### Non-Interactive Mode

```bash
# Print CSS tokens to stdout
cargo run -- --color "#FF6B6B" --format css

# Print design tokens JSON
cargo run -- --color "#FF6B6B" --format design-tokens

# Write Tailwind v4 tokens to a file
cargo run -- --color "#FF6B6B" --format tailwind-v4 --out ./palette.css

# Export a custom token name and preserve the input at step 600
cargo run -- --color "#FF6B6B" --format tailwind-v3 --name brand --base-step 600
```

By default, the input color is preserved at step `500`. If you override `--base-step`, the exported default token/alias follows that configured step.

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Generate palette |
| `Tab` | Random color |
| `C` | Export CSS |
| `D` | Export Design Tokens |
| `3` | Export Tailwind v3 |
| `4` | Export Tailwind v4 |
| `J` | Export JSON |
| `N` | New color |
| `Esc` | Back/Exit |

## 🎨 Export Formats

### CSS Variables
```css
:root {
  --color-primary-50: #f0ebff;
  --color-primary-50-rgb: 240 235 255;
  --color-primary-100: #e2d9ff;
  --color-primary-100-rgb: 226 217 255;
  /* ... */
  --color-primary: var(--color-primary-500);
  --color-primary-rgb: var(--color-primary-500-rgb);
  --color-on-primary: #ffffff;
  --color-on-primary-rgb: 255 255 255;
}
```

### Tailwind CSS v3
```javascript
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0ebff',
          100: '#e2d9ff',
          // ...
          500: '#7953e0',
          DEFAULT: '#7953e0',
        },
        'on-primary': '#ffffff',
      },
    },
  },
};
```

### Design Tokens JSON
```json
{
  "color": {
    "primary": {
      "500": { "$value": "#7953e0", "$type": "color" }
    },
    "semantic": {
      "primary": { "$value": "{color.primary.500}", "$type": "color" },
      "on-primary": { "$value": "#ffffff", "$type": "color" }
    }
  }
}
```

### Tailwind CSS v4
```css
@import "tailwindcss";

@theme {
  --color-primary-50: oklch(0.978 0.031 305.2); /* #f0ebff */
  --color-primary-100: oklch(0.936 0.062 305.4); /* #e2d9ff */
  /* ... */
  --color-primary: var(--color-primary-500);
  --color-on-primary: oklch(1.000 0.000 0.000); /* #ffffff 6.43:1 AA */
}
```

### JSON Format
```json
{
  "name": "primary",
  "defaultStep": 500,
  "colors": {
    "50": {
      "hex": "#f0ebff",
      "rgb": { "r": 240, "g": 235, "b": 255 },
      "hsl": { "h": "255", "s": "100.0", "l": "96.1" },
      "oklch": { "l": "0.978", "c": "0.031", "h": "305.2" },
      "accessibility": {
        "recommendedForeground": { "hex": "#000000", "rating": "AAA" }
      }
    },
    // ...
  },
  "semanticTokens": {
    "primary": { "step": 500, "hex": "#7953e0" },
    "on-primary": { "hex": "#ffffff", "rating": "AA" }
  }
}
```

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Cargo

### Building from Source
```bash
# Development build
cargo build

# Format and lint
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings

# Release build
cargo build --release

# Run tests
cargo test
```

### Project Structure
```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library exports
├── color/
│   └── palette.rs    # Color generation algorithms
├── export/
│   ├── css.rs        # CSS exporter
│   ├── json.rs       # JSON exporter
│   └── tailwind.rs   # Tailwind CSS exporters
└── ui/
    └── app.rs        # Terminal UI logic
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🔒 Security

This tool runs entirely locally and does not send any data to external servers. All color processing happens on your machine.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses the [palette](https://github.com/Ogeon/palette) crate for color science
- Inspired by modern color systems and design tools

---

Made with 💜 by [Thalys Guimarães](https://github.com/thalysguimaraes)
