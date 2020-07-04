use std::io;

use termion::raw::IntoRawMode;
use tui::{
    backend::{Backend, TermionBackend}, 
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Widget, Block, Borders, List, Paragraph, ListState, Text},
    Frame
};

use crate::cli::App;
use crate::data::{Symbol, Option};

pub type Terminal = tui::Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>;

pub trait Listable {
    fn short_name(&self) -> String;
}

impl Listable for Symbol {
    fn short_name(&self) -> String { self.symbol.clone() }
}

impl Listable for Option {
    fn short_name(&self) -> String { self.underlying_symbol.clone() }
}

#[derive(Clone)]
pub struct StatefulList<T: Listable> { 
    pub list: Vec<T>,
    pub state: ListState
}

impl<T: Listable> Default for StatefulList<T> {
    fn default() -> Self { 
        StatefulList {
            list: vec![],
            state: ListState::default()
        }
    }
}

impl<T: Listable> StatefulList<T> {
    pub fn with_list(list: Vec<T>) -> Self {
        StatefulList {
            state: ListState::default(),
            list,
        }
    }
}

impl<T: Listable> StatefulList<T> { 
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub enum ViewContext {
    Watchlist
}

pub fn initialize_terminal() -> Result<Terminal, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {

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

fn draw_main_area<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(area);
    {
        let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(12),
                    Constraint::Min(0)
                ].as_ref())
                .split(chunks[0]);
            let items = app.watchlist.list.iter().map(|i| {
                let title = i.short_name();
                Text::raw(title)
            });
            let tasks = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Watchlist"))
                .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(tasks, chunks[0], &mut app.watchlist.state);
    }
    
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