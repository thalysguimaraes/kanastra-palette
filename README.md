# Kanastra Palette

Kanastra Palette is a TypeScript library for generating color palettes with dynamic base tone determination. Inspired by [Palettey](https://github.com/bartbergmans/Palettey), it offers a robust solution for creating balanced color schemes in modern web applications.

## Features

- Generates complete color palettes from a single color
- Automatically determines the base tone based on color luminosity
- Creates balanced lighter and darker shades
- Compatible with design frameworks like Tailwind CSS
- Written in TypeScript for enhanced type safety and developer experience
- Includes unit tests for reliability

## Installation

```bash
npm install kanastra-palette
```

## Usage

```typescript
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

## Example Output

```json
{
  "primary": {
    "50": "#fafafa",
    "100": "#f5f5f5",
    "200": "#e5e5e5",
    "300": "#d4d4d4",
    "400": "#a3a3a3",
    "500": "#737373",
    "600": "#525252",
    "700": "#3f3f3f",
    "800": "#262626",
    "900": "#171717"
  }
}
```


## Development

To set up the project for development:

1. Clone the repository
2. Install dependencies: `npm install`
3. Run tests: `npm test`
4. Build the project: `npm run build`

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/AmazingFeature`
3. Commit your changes: `git commit -m 'Add some AmazingFeature'`
4. Push to the branch: `git push origin feature/AmazingFeature`
5. Open a pull request

Please make sure to update tests as appropriate and adhere to the existing coding style.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Palettey](https://github.com/bartbergmans/Palettey)
- Uses [chroma.js](https://github.com/gka/chroma.js) for color manipulations

## Support

If you encounter any issues or have questions, please file an issue on the [GitHub repository](https://github.com/thalysguimaraes/kanastra-palette/issues).