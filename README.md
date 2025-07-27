# Kanastra Palette 🎨

![Kanastra Palette Banner](public/banner-image.jpg)

A blazing-fast terminal color palette generator built with Rust and Ratatui. Generate beautiful, accessible color palettes with advanced color science algorithms and export them in multiple formats including Tailwind CSS v3/v4.

## ✨ Features

### 🎯 Smart Palette Generation
- **11-step color scales** (50-950) using perceptual color algorithms
- **OKLCH color space** for superior perceptual uniformity
- **Dynamic lightness and chroma curves** for natural-looking palettes
- **Hue shifting** for more vibrant color scales

### 🚀 Modern Export Formats
- **CSS Custom Properties** with RGB values for alpha support
- **Tailwind CSS v3** - Traditional JS config format
- **Tailwind CSS v4** - New CSS `@theme` format with OKLCH values
- **JSON** with hex, RGB, HSL, and OKLCH color spaces

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

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Generate palette |
| `Tab` | Random color |
| `C` | Export CSS |
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
          DEFAULT: '#7953e0',
        },
      },
    },
  },
};
```

### Tailwind CSS v4
```css
@import "tailwindcss";

@theme {
  --color-primary-50: #f0ebff; /* oklch(0.978 0.031 305.2) */
  --color-primary-100: #e2d9ff; /* oklch(0.936 0.062 305.4) */
  /* ... */
}
```

### JSON Format
```json
{
  "name": "primary",
  "colors": {
    "50": {
      "hex": "#f0ebff",
      "rgb": { "r": 240, "g": 235, "b": 255 },
      "hsl": { "h": "255", "s": "100.0", "l": "96.1" },
      "oklch": { "l": "0.978", "c": "0.031", "h": "305.2" }
    },
    // ...
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