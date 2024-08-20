import chroma from 'chroma-js';

/**
 * Determines the base step for a given color based on its luminance.
 * @param color - The chroma color object
 * @returns The base step (100-800)
 */
export function determineBaseStep(color: chroma.Color): number {
  const luminance = color.luminance();
  if (luminance > 0.8) return 100;
  if (luminance > 0.6) return 200;
  if (luminance > 0.4) return 300;
  if (luminance > 0.3) return 400;
  if (luminance > 0.2) return 500;
  if (luminance > 0.1) return 600;
  if (luminance > 0.05) return 700;
  return 800;
}