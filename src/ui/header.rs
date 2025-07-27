use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_clean_header(f: &mut Frame, area: Rect) {
    let header_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());
    
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("●", Style::default().fg(Color::Rgb(34, 197, 94))),
            Span::raw(" "),
            Span::styled("KANASTRA", Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("PALETTE", Style::default()
                .fg(Color::Rgb(134, 239, 172))
                .add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("●", Style::default().fg(Color::Rgb(34, 197, 94))),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Generate beautiful color palettes from your brand", 
                Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
    ];
    
    let header = Paragraph::new(lines)
        .block(header_block)
        .alignment(Alignment::Center);
    
    f.render_widget(header, area);
}