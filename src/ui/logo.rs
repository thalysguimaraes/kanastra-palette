use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render_kanastra_logo(f: &mut Frame, area: Rect) {
    // ASCII art KANASTRA banner
    let logo_lines = vec![
        Line::from(vec![Span::styled("██╗  ██╗ █████╗ ███╗   ██╗ █████╗ ███████╗████████╗██████╗  █████╗", Style::default().fg(Color::Rgb(74, 222, 128)))]),
        Line::from(vec![Span::styled("██║ ██╔╝██╔══██╗████╗  ██║██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔══██╗", Style::default().fg(Color::Rgb(74, 222, 128)))]),
        Line::from(vec![Span::styled("█████╔╝ ███████║██╔██╗ ██║███████║███████╗   ██║   ██████╔╝███████║", Style::default().fg(Color::Rgb(134, 239, 172)))]),
        Line::from(vec![Span::styled("██╔═██╗ ██╔══██║██║╚██╗██║██╔══██║╚════██║   ██║   ██╔══██╗██╔══██║", Style::default().fg(Color::Rgb(134, 239, 172)))]),
        Line::from(vec![Span::styled("██║  ██╗██║  ██║██║ ╚████║██║  ██║███████║   ██║   ██║  ██║██║  ██║", Style::default().fg(Color::Rgb(74, 222, 128)))]),
        Line::from(vec![Span::styled("╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝", Style::default().fg(Color::Rgb(34, 197, 94)))]),
    ];
    
    let logo = Paragraph::new(logo_lines)
        .alignment(Alignment::Center)
        .block(Block::default());
    
    f.render_widget(logo, area);
}

pub fn render_simple_kanastra_logo(f: &mut Frame, area: Rect) {
    // Alternative simple logo with just the K and text
    let simple_logo = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("    "),
            Span::styled("╦╔═", Style::default().fg(Color::Rgb(34, 197, 94)).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("KANASTRA PALETTE", Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::styled("╠╩╗", Style::default().fg(Color::Rgb(74, 222, 128)).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("●", Style::default().fg(Color::Rgb(220, 252, 231))),
            Span::raw(" "),
            Span::styled("●", Style::default().fg(Color::Rgb(187, 247, 208))),
            Span::raw(" "),
            Span::styled("●", Style::default().fg(Color::Rgb(134, 239, 172))),
            Span::raw(" "),
            Span::styled("●", Style::default().fg(Color::Rgb(74, 222, 128))),
            Span::raw(" "),
            Span::styled("●", Style::default().fg(Color::Rgb(34, 197, 94))),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::styled("╩ ╩", Style::default().fg(Color::Rgb(134, 239, 172)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];
    
    let logo = Paragraph::new(simple_logo)
        .alignment(Alignment::Center)
        .block(Block::default());
    
    f.render_widget(logo, area);
}