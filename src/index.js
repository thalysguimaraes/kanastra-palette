import chroma from 'chroma-js';

function determineBaseStep(color) {
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

function createPaletteFromColor(name, hex, options = {}) {
  const baseColor = chroma(hex);
  const palette = {};
  const steps = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
  const baseStep = determineBaseStep(baseColor);

  steps.forEach(step => {
    let color;
    if (step < baseStep) {
      const mixRatio = 1 - (step / baseStep);
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

export { createPaletteFromColor };