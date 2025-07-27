use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub foreground: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Magenta,
            background: Color::Black,
            foreground: Color::White,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
        }
    }
    
    pub fn dark() -> Self {
        Self {
            primary: Color::Rgb(0, 188, 212),
            secondary: Color::Rgb(156, 39, 176),
            background: Color::Rgb(18, 18, 18),
            foreground: Color::Rgb(238, 238, 238),
            success: Color::Rgb(76, 175, 80),
            error: Color::Rgb(244, 67, 54),
            warning: Color::Rgb(255, 193, 7),
            info: Color::Rgb(33, 150, 243),
        }
    }
}

pub struct Styles {
    pub theme: Theme,
}

impl Styles {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
    
    pub fn title(&self) -> Style {
        Style::default()
            .fg(self.theme.primary)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn normal(&self) -> Style {
        Style::default().fg(self.theme.foreground)
    }
    
    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.theme.background)
            .bg(self.theme.primary)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn error(&self) -> Style {
        Style::default().fg(self.theme.error)
    }
    
    pub fn success(&self) -> Style {
        Style::default().fg(self.theme.success)
    }
    
    pub fn warning(&self) -> Style {
        Style::default().fg(self.theme.warning)
    }
    
    pub fn info(&self) -> Style {
        Style::default().fg(self.theme.info)
    }
}

impl Default for Styles {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}