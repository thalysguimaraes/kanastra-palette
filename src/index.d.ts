declare module 'kanastra-palette' {
    interface PaletteOptions {
      // Add any options you want to support
    }
  
    function createPaletteFromColor(
      name: string,
      hex: string,
      options?: PaletteOptions
    ): Record<string, Record<number, string>>;
  
    export { createPaletteFromColor };
  }