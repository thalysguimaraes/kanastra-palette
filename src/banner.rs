use ratatui::style::Color;
use ratatui::text::{Line, Span};
use ratatui::style::Style;

pub fn get_compact_banner() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled("██╗  ██╗ █████╗ ███╗   ██╗ █████╗ ███████╗████████╗██████╗  █████╗", Style::default().fg(Color::Rgb(74, 222, 128)))]),
        Line::from(vec![Span::styled("██║ ██╔╝██╔══██╗████╗  ██║██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔══██╗", Style::default().fg(Color::Rgb(74, 222, 128)))]),
        Line::from(vec![Span::styled("█████╔╝ ███████║██╔██╗ ██║███████║███████╗   ██║   ██████╔╝███████║", Style::default().fg(Color::Rgb(134, 239, 172)))]),
        Line::from(vec![Span::styled("██╔═██╗ ██╔══██║██║╚██╗██║██╔══██║╚════██║   ██║   ██╔══██╗██╔══██║", Style::default().fg(Color::Rgb(134, 239, 172)))]),
        Line::from(vec![Span::styled("██║  ██╗██║  ██║██║ ╚████║██║  ██║███████║   ██║   ██║  ██║██║  ██║", Style::default().fg(Color::Rgb(74, 222, 128)))]),
        Line::from(vec![Span::styled("╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝", Style::default().fg(Color::Rgb(34, 197, 94)))]),
    ]
}

pub fn get_simple_logo() -> Vec<&'static str> {
    vec![
        r#" ██╗  ██╗"#,
        r#" ██║ ██╔╝"#,
        r#" █████╔╝ "#,
        r#" ██╔═██╗ "#,
        r#" ██║  ██╗"#,
        r#" ╚═╝  ╚═╝"#,
    ]
}

pub fn get_banner_colors() -> Vec<Color> {
    vec![
        Color::Rgb(34, 197, 94),   // Green 500
        Color::Rgb(74, 222, 128),  // Green 400
        Color::Rgb(134, 239, 172), // Green 300
    ]
}