import chroma from 'chroma-js';
import { determineBaseStep } from './utils';
import { PaletteOptions } from './types';

/**
 * Creates a color palette from a single color.
 * @param name - The name of the palette (e.g., 'primary', 'secondary')
 * @param hex - The base color in hexadecimal format
 * @param options - Additional options for palette generation
 * @returns An object with the color palette, containing shades from 50 to 900
 */
export function createPaletteFromColor(
  name: string,
  hex: string,
  options: PaletteOptions = {}
): Record<string, Record<number, string>> {
  if (!chroma.valid(hex)) {
    throw new Error(`Invalid color: ${hex}`);
  }

  const baseColor = chroma(hex);
  const palette: Record<number, string> = {};
  const steps = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
  const baseStep = determineBaseStep(baseColor);

  steps.forEach(step => {
    let color: chroma.Color;
    if (step < baseStep) {
      const mixRatio = 1 - step / baseStep;
      color = chroma.mix(baseColor, 'white', mixRatio, 'rgb');
    } else if (step > baseStep) {
      const mixRatio = (step - baseStep) / (900 - baseStep);
      color = chroma.mix(baseColor, 'black', mixRatio, 'rgb');
      if (step === 900 && color.hex() === '#000000') {
        color = chroma.mix(baseColor, 'black', mixRatio * 0.9, 'rgb');
      }
    } else {
      color = baseColor;
    }

    palette[step] = color.hex();
  });

  return { [name]: palette };
}