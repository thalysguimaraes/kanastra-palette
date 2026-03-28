use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use kanastra_palette_rs::export::{ExportFormat, ExportOptions};
use kanastra_palette_rs::ui::{App, AppState};
use kanastra_palette_rs::{
    format_hex_color, is_palette_step, parse_hex_color, rgb8, ColorPalette, PaletteAlgorithm,
    PaletteOptions, PALETTE_STEPS,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::{fs, io, path::PathBuf, time::Duration};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliFormat {
    Css,
    DesignTokens,
    Json,
    TailwindV3,
    TailwindV4,
}

impl From<CliFormat> for ExportFormat {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Css => Self::Css,
            CliFormat::DesignTokens => Self::DesignTokens,
            CliFormat::Json => Self::Json,
            CliFormat::TailwindV3 => Self::TailwindV3,
            CliFormat::TailwindV4 => Self::TailwindV4,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliAlgorithm {
    Oklch,
}

impl From<CliAlgorithm> for PaletteAlgorithm {
    fn from(value: CliAlgorithm) -> Self {
        match value {
            CliAlgorithm::Oklch => Self::Oklch,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Base color in hex format (#RGB or #RRGGBB)")]
    color: Option<String>,
    #[arg(long, value_enum, help = "Run in non-interactive export mode")]
    format: Option<CliFormat>,
    #[arg(
        long,
        default_value = "primary",
        help = "Token name used in the exported output"
    )]
    name: String,
    #[arg(long, value_enum, default_value_t = CliAlgorithm::Oklch, help = "Palette generation algorithm")]
    algorithm: CliAlgorithm,
    #[arg(long, default_value_t = 500, value_parser = parse_base_step, help = "Step that should preserve the input color")]
    base_step: u16,
    #[arg(long, help = "Print export output to stdout")]
    stdout: bool,
    #[arg(long, value_name = "FILE", help = "Write export output to a file")]
    out: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(format) = args.format {
        return run_export(args, format);
    }

    run_tui(args)
}

fn run_tui(args: Args) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let palette_options = palette_options_from_args(&args);
    let export_options = export_options_from_args(&args);
    let mut app = App::with_options(palette_options, export_options);
    if let Some(color) = args.color {
        app.color_input = color;
    }

    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_export(args: Args, format: CliFormat) -> Result<()> {
    let color = args
        .color
        .as_deref()
        .context("--color is required when using --format")?;
    let palette = ColorPalette::new_with_options(color, &palette_options_from_args(&args))?;
    let output = ExportFormat::from(format)
        .export_with_options(&palette, &export_options_from_args(&args))?;

    if let Some(path) = &args.out {
        fs::write(path, &output).with_context(|| format!("failed to write {}", path.display()))?;
    }

    if args.stdout || args.out.is_none() {
        println!("{output}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key_event(key);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    match app.state {
        AppState::ColorInput => render_input_screen(f, app),
        AppState::PaletteView => render_palette_screen(f, app),
    }
}

fn render_input_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(6), // Logo (increased for ASCII art)
            Constraint::Length(2), // Description
            Constraint::Length(2), // Spacing
            Constraint::Length(5), // Input field
            Constraint::Length(3), // Live preview
            Constraint::Min(0),    // Spacing
            Constraint::Length(3), // Help text
        ])
        .split(f.area());

    // Logo
    kanastra_palette_rs::ui::logo::render_kanastra_logo(f, chunks[0]);

    // Description
    let description = Paragraph::new("Generate beautiful white-label palettes from brand colors")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(description, chunks[1]);

    // Input field
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(" Enter Color ")
        .style(Style::default().fg(Color::Cyan));

    let input = Paragraph::new(app.color_input.as_str())
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .block(input_block);
    f.render_widget(input, chunks[3]);

    // Live preview
    if let Ok(color) = parse_hex_color(&app.color_input) {
        let (r, g, b) = rgb8(&color);
        let preview = Block::default().style(Style::default().bg(Color::Rgb(r, g, b)));
        f.render_widget(preview, chunks[4]);
    }

    // Message
    if let Some(msg) = &app.message {
        let message = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        f.render_widget(message, chunks[5]);
    }

    // Help
    let help = Paragraph::new("[Enter] Generate   [Tab] Random Color   [Esc] Exit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[6]);
}

fn render_palette_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(15),   // Palette
            Constraint::Length(5), // Export options
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("GENERATED PALETTE")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Palette display
    if let Some(palette) = &app.current_palette {
        render_palette_colors(f, palette, chunks[1]);
    }

    // Export options
    let export_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let export_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[C]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" CSS   "),
            Span::styled(
                "[D]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Design Tokens   "),
            Span::styled(
                "[3]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Tailwind v3   "),
            Span::styled(
                "[4]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Tailwind v4   "),
            Span::styled(
                "[J]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" JSON"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[N]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" New Color   "),
            Span::styled("[Esc]", Style::default().fg(Color::Red)),
            Span::raw(" Back to Input"),
        ]),
    ];

    let export_paragraph = Paragraph::new(export_text)
        .block(export_block)
        .alignment(Alignment::Center);
    f.render_widget(export_paragraph, chunks[2]);

    // Message
    if let Some(msg) = &app.message {
        let msg_area = Rect {
            x: chunks[2].x,
            y: chunks[2].y + chunks[2].height,
            width: chunks[2].width,
            height: 1,
        };
        let message = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center);
        f.render_widget(message, msg_area);
    }
}

fn render_palette_colors(f: &mut Frame, palette: &kanastra_palette_rs::ColorPalette, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Color Palette (* base) ")
        .style(Style::default().fg(Color::Cyan));

    let mut rows = Vec::new();

    for step in PALETTE_STEPS {
        if let Some(color) = palette.steps.get(&step) {
            let accessibility = palette.accessibility.steps.get(&step).unwrap();
            let hex = format_hex_color(color);
            let (r, g, b) = rgb8(color);
            let color_preview = Color::Rgb(r, g, b);
            let on_hex = accessibility.suggested_foreground.hex();
            let (on_r, on_g, on_b) = rgb8(&accessibility.suggested_foreground.color);
            let on_preview = Color::Rgb(on_r, on_g, on_b);
            let step_label = if step == palette.base_step {
                format!("*{:>3}", step)
            } else {
                format!("{:>4}", step)
            };
            let contrast_label = format!(
                "{:.2}:1 {}",
                accessibility.suggested_foreground.contrast_ratio,
                accessibility.suggested_foreground.compliance.rating()
            );

            // Create color block with proper spacing
            let color_block = "████████████████";

            let row_style = if step == palette.base_step {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            rows.push(
                Row::new(vec![
                    Cell::from(step_label).style(Style::default().fg(Color::Gray)),
                    Cell::from(color_block)
                        .style(Style::default().fg(color_preview).bg(color_preview)),
                    Cell::from(hex.clone()).style(
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Cell::from("██████").style(Style::default().fg(on_preview).bg(on_preview)),
                    Cell::from(on_hex).style(Style::default().fg(Color::White)),
                    Cell::from(contrast_label).style(Style::default().fg(Color::DarkGray)),
                ])
                .style(row_style),
            );
        }
    }

    let table = Table::new(
        rows,
        vec![
            Constraint::Length(6),  // Step number
            Constraint::Length(18), // Color block
            Constraint::Length(9),  // Hex
            Constraint::Length(8),  // On color swatch
            Constraint::Length(9),  // On color hex
            Constraint::Length(14), // Contrast and WCAG rating
        ],
    )
    .block(block)
    .column_spacing(2);

    f.render_widget(table, area);
}

fn parse_base_step(step: &str) -> Result<u16, String> {
    let step = step
        .parse::<u16>()
        .map_err(|_| format!("invalid step: {step}"))?;

    if !is_palette_step(step) {
        return Err(format!(
            "unsupported step {step}. Use one of {:?}",
            PALETTE_STEPS
        ));
    }

    Ok(step)
}

fn palette_options_from_args(args: &Args) -> PaletteOptions {
    PaletteOptions {
        anchor_step: args.base_step,
        algorithm: args.algorithm.into(),
    }
}

fn export_options_from_args(args: &Args) -> ExportOptions {
    ExportOptions {
        name: args.name.clone(),
    }
}
