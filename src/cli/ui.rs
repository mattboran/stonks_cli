use std::io;

use termion::raw::IntoRawMode;
use tui::{
    backend::{Backend, TermionBackend}, 
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Widget, GraphType, Dataset, Block, Borders, List, Paragraph, ListState, Text, Chart, Axis},
    Frame
};

use crate::cli::App;
use crate::data::{self, Symbol, Quote};

pub type Terminal = tui::Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>;

pub trait Listable {
    fn short_name(&self) -> String;
}

impl Listable for Symbol {
    fn short_name(&self) -> String { self.symbol.clone() }
}

impl Listable for data::Option {
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
            Constraint::Length(50),
            Constraint::Min(0)
        ].as_ref())
        .split(area);
    {
        let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(10),
                    Constraint::Min(0)
                ].as_ref())
                .split(chunks[0]);
            draw_watchlist(f, app, chunks[0]);
            draw_quote_section(f, app, chunks[1]);
    }
    draw_graph_section(f, app, chunks[1]);
}

fn draw_watchlist<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let items = app.watchlist.list.iter().map(|i| {
        let title = i.short_name();
        Text::raw(title)
    });
    let tasks = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Watchlist"))
        .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(tasks, area, &mut app.watchlist.state);
}

fn draw_quote_section<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let symbol = app.selected_ticker();
    let text = quote_section_text(app.get_quote(symbol));
    let block: Block = Block::default()
        .borders(Borders::ALL)
        .title("Quote");
    let paragraph = Paragraph::new(text.iter()).block(block).wrap(false);
    f.render_widget(paragraph, area);
}

fn quote_section_text(quote: Option<&Quote>) -> Vec<Text> { 
    let mut text = vec![];
    text.push(Text::styled(
        if let Some(q) = quote { q.description.to_string() } else { "...".to_string() },
        Style::default().modifier(Modifier::BOLD)
    ));
    text.push(Text::raw("\n\nLast Price: $"));
    text.push(Text::raw(if let Some(q) = quote { format!("{:.2}", q.last) } else { "...".to_string() }));
    text.push(Text::raw("\nVolume: "));
    text.push(Text::raw(if let Some(q) = quote { q.volume.to_string() } else { "...".to_string() }));
    text.push(Text::raw("\n\nBid:"));
    text.push(Text::raw(format!(" ${} ({})", 
        if let Some(q) = quote { format!("{:.2}", q.bid) } else { "...".to_string() },
        if let Some(q) = quote { q.bid_size.to_string() } else { "...".to_string() }))
    );
    text.push(Text::raw("\nAsk:"));
    text.push(Text::raw(format!(" ${} ({})", 
        if let Some(q) = quote { format!("{:.2}", q.ask) } else { "...".to_string() },
        if let Some(q) = quote { q.ask_size.to_string() } else { "...".to_string() }))
    );

    text.push(Text::raw("\n\nChange "));
    text.push(Text::styled(format!("${} : {}%", 
        if let Some(q) = quote { format!("{:.2}", q.change_points) } else { "...".to_string() },
        if let Some(q) = quote { q.change_percentage.to_string() } else { "...".to_string() }),
        if let Some(q) = quote { 
            if q.change_points > 0.0 { 
                Style::default().fg(Color::Green)
            } else if q.change_points == 0.0 {
                Style::default()
            } else {
                Style::default().fg(Color::Red)
            }
        } else { Style::default() }
    ));
    text.push(Text::raw("\nOpen: "));
    text.push(Text::styled(
        quote.map_or("...".to_string(), |q| {
            q.open.map_or("nil".to_string(), |o| format!("{:.2}", o))
        }), quote.map_or(Style::default(), |q| {
            q.open.map_or(Style::default().fg(Color::Magenta), |_| Style::default())
        })
    ));
    text.push(Text::raw("\nClose: "));
    text.push(Text::styled(
        quote.map_or("...".to_string(), |q| {
            q.close.map_or("nil".to_string(), |c| format!("{:.2}", c))
        }), quote.map_or(Style::default(), |q| {
            q.close.map_or(Style::default().fg(Color::Magenta), |_| Style::default())
        })
    ));
    text.push(Text::raw("\n\n52 Week High: $"));
    text.push(Text::raw(if let Some(q) = quote { format!("{:.2}", q.week_52_high) } else { "...".to_string() }));
    text.push(Text::raw("\n52 Week Low: $"));
    text.push(Text::raw(if let Some(q) = quote { format!("{:.2}", q.week_52_low) } else { "...".to_string() }));
    text
}

fn draw_graph_section<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // let dataset: Dataset;
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Graph");

    let selected_symbol = app.selected_ticker();
    let graph_data = app.graph_cache.get(selected_symbol);
    if let Some(timeseries) = graph_data {
        let data = timeseries.to_graph_data(area.width);
        let color = if timeseries.went_up() { Color::Green } else { Color::Red };
        let dataset = &[Dataset::default()
            .graph_type(GraphType::Line)
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(color))
            .data(&data[..])];
        let (min_bound, max_bound) = timeseries.min_max();
        let labels = &[format!("{:.2}", min_bound),
            format!("{:.2}", (max_bound + min_bound) * 0.5),
            format!("{:.2}", max_bound)];
        let chart = Chart::default()
            .block(block)
        .x_axis(Axis::default()
            .style(Style::default().fg(Color::Gray))
            .bounds([0.0, area.width as f64])
            .labels(data::series::TIME_MARKERS))
        .y_axis(Axis::default()
            .style(Style::default().fg(Color::Gray))
            .bounds([min_bound, max_bound])
            .labels(labels))
        .datasets(dataset);
        f.render_widget(chart, area);
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