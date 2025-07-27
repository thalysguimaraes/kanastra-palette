use crate::ColorPalette;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    ColorInput,
    PaletteView,
}


pub struct App {
    pub state: AppState,
    pub color_input: String,
    pub current_palette: Option<ColorPalette>,
    pub should_quit: bool,
    pub message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::ColorInput,
            color_input: String::from("#7953E0"),
            current_palette: None,
            should_quit: false,
            message: None,
        }
    }
    
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match self.state {
            AppState::ColorInput => self.handle_input_keys(key),
            AppState::PaletteView => self.handle_palette_keys(key),
        }
    }
    
    fn handle_input_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Enter => self.generate_palette(),
            KeyCode::Tab => self.generate_random_color(),
            KeyCode::Char(c) => {
                if c.is_alphanumeric() || c == '#' {
                    self.color_input.push(c);
                }
            }
            KeyCode::Backspace => {
                self.color_input.pop();
            }
            _ => {}
        }
    }
    
    fn handle_palette_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.state = AppState::ColorInput,
            KeyCode::Char('c') | KeyCode::Char('C') => self.export_css(),
            KeyCode::Char('3') => self.export_tailwind_v3(),
            KeyCode::Char('4') => self.export_tailwind_v4(),
            KeyCode::Char('j') | KeyCode::Char('J') => self.export_json(),
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.state = AppState::ColorInput;
                self.color_input.clear();
                self.message = None;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
            _ => {}
        }
    }
    
    fn generate_palette(&mut self) {
        self.message = None;
        
        // Clean up input
        let input = self.color_input.trim();
        if input.is_empty() {
            self.message = Some("Please enter a color".to_string());
            return;
        }
        
        match ColorPalette::new(input) {
            Ok(palette) => {
                self.current_palette = Some(palette);
                self.state = AppState::PaletteView;
            }
            Err(_) => {
                self.message = Some("Invalid color format. Use hex format like #7953E0".to_string());
            }
        }
    }
    
    fn export_css(&mut self) {
        if let Some(palette) = &self.current_palette {
            if let Ok(content) = crate::export::ExportFormat::Css.export(palette) {
                self.copy_to_clipboard(content, "CSS");
            }
        }
    }
    
    fn export_tailwind_v3(&mut self) {
        if let Some(palette) = &self.current_palette {
            if let Ok(content) = crate::export::ExportFormat::TailwindV3.export(palette) {
                self.copy_to_clipboard(content, "Tailwind v3");
            }
        }
    }
    
    fn export_tailwind_v4(&mut self) {
        if let Some(palette) = &self.current_palette {
            if let Ok(content) = crate::export::ExportFormat::TailwindV4.export(palette) {
                self.copy_to_clipboard(content, "Tailwind v4");
            }
        }
    }
    
    fn export_json(&mut self) {
        if let Some(palette) = &self.current_palette {
            if let Ok(content) = crate::export::ExportFormat::Json.export(palette) {
                self.copy_to_clipboard(content, "JSON");
            }
        }
    }
    
    
    fn copy_to_clipboard(&mut self, content: String, format: &str) {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                match clipboard.set_text(&content) {
                    Ok(_) => self.message = Some(format!("Copied {} to clipboard!", format)),
                    Err(_) => self.message = Some("Failed to copy to clipboard".to_string()),
                }
            }
            Err(_) => self.message = Some("Clipboard not available".to_string()),
        }
    }
    
    fn generate_random_color(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Generate vibrant colors by using HSL with high saturation
        let hue = rng.gen_range(0..360);
        let saturation = rng.gen_range(60..90); // High saturation for vibrant colors
        let lightness = rng.gen_range(40..60);  // Mid-range lightness
        
        // Convert HSL to RGB
        let h = hue as f32 / 360.0;
        let s = saturation as f32 / 100.0;
        let l = lightness as f32 / 100.0;
        
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.color_input = format!("#{:02X}{:02X}{:02X}", 
            (r * 255.0) as u8, 
            (g * 255.0) as u8, 
            (b * 255.0) as u8
        );
        
        self.message = None;
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = match (h * 6.0) as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    
    (r + m, g + m, b + m)
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}