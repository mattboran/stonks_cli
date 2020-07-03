use std::io;

use termion::raw::IntoRawMode;
use tui::{
    backend::{Backend, TermionBackend}, 
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Widget, Block, Borders, Paragraph, Text},
    Frame
};

use crate::cli::App;

pub type Terminal = tui::Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>;

pub fn initialize_terminal() -> Result<Terminal, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), 
            Constraint::Min(0),
            Constraint::Length(3),
        ].as_ref())
        .split(f.size());
        
    draw_header(f, app, chunks[0]);
    draw_main_area(f, app, chunks[1]);
    draw_log_section(f, app, chunks[2]);
}
    
fn draw_header<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let block = Block::default()
        .title(&app.title)
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_main_area<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).split(area);
    
}

fn draw_log_section<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let default =  &"".to_string();
    let log = if let Some(log) = app.log.last() { log } else { default };
    let text = [
        Text::styled(log, Style::default().modifier(Modifier::ITALIC))
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Log");
    let paragraph = Paragraph::new(text.iter()).block(block).wrap(false);
    f.render_widget(paragraph, area);
}