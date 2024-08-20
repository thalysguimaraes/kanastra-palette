# Kanastra Palette

Kanastra Palette is a JavaScript library for generating color palettes with dynamic base tone determination. Inspired by [Palettey](https://github.com/bartbergmans/Palettey)

## Features

- Generates complete color palettes from a single color
- Automatically determines the base tone based on color luminosity
- Creates balanced lighter and darker shades
- Compatible with design frameworks like Tailwind CSS

## Installation

```bash
npm install kanastra-palette
```

## Usage

```javascript
import { createPaletteFromColor } from 'kanastra-palette';

// Generate a palette from a color
const primaryPalette = createPaletteFromColor('primary', '#7953e0');
console.log(primaryPalette);
```

## API

### `createPaletteFromColor(name: string, hex: string, options?: PaletteOptions): Record<string, Record<number, string>>`

Generates a complete color palette from a single color.

- `name`: Name of the palette (e.g., 'primary', 'secondary')
- `hex`: Base color in hexadecimal format
- `options`: Additional options (reserved for future expansions)

Returns an object with the color palette, containing shades from 50 to 900.


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on our [GitHub repository](link-to-your-repository).

## License

MIT