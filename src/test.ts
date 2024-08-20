import { createPaletteFromColor } from './index';

const primaryPalette = createPaletteFromColor('primary', '#7953e0');
console.log(JSON.stringify(primaryPalette, null, 2));

const secondaryPalette = createPaletteFromColor('secondary', '#50C878');
console.log(JSON.stringify(secondaryPalette, null, 2));