import { createPaletteFromColor } from './index';

describe('createPaletteFromColor', () => {
  it('should generate a valid palette', () => {
    const palette = createPaletteFromColor('primary', '#7953e0');
    expect(palette).toHaveProperty('primary');
    expect(Object.keys(palette.primary)).toHaveLength(10);
    expect(palette.primary[500]).toBe('#7953e0');
  });

  it('should throw an error for invalid color input', () => {
    expect(() => createPaletteFromColor('primary', 'invalid-color')).toThrow('Invalid color');
  });
});