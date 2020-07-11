mod loader;
mod cache;
pub mod ui;
pub mod event;

use std::fmt;
use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;
use chrono::{Datelike, Date, Timelike, DateTime, TimeZone};
pub use termion::event::Key;

use crate::data::{self, Symbol, Quote};
use crate::api::client;
use crate::util;
use cache::{QuoteCache, GraphCache};
use ui::{StatefulList, ViewContext, Listable};

#[derive(Debug)]
pub enum CliError {
    InitError { msg: String },
}

impl Error for CliError { }

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::InitError { ref msg } => write!(f, "Initialization error: {}", msg)
        }
    }
}

pub struct App { 
    pub title: String,
    pub symbols: Vec<Symbol>,
    pub options: Vec<data::Option>,
    pub watchlist: StatefulList<Symbol>,
    pub log: Vec<String>,
    pub should_quit: bool,
    quote_cache: QuoteCache,
    graph_cache: GraphCache,
    active_context: ViewContext,
}

impl App { 
    pub fn new() -> Self { 
        App {
            title: format!("StonksCLI"),
            options: vec![],
            symbols: vec![],
            active_context: ViewContext::Watchlist,
            watchlist: StatefulList::default(),
            quote_cache: QuoteCache::new(),
            graph_cache: GraphCache::new(),
            log: vec![],
            should_quit: false,
        }
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' { 
            self.should_quit = true;
        }
    }

    pub fn on_up(&mut self, app: Arc<Mutex<App>>) {
        match self.active_context {
            ViewContext::Watchlist => {
                self.watchlist.previous();
                tokio::spawn(async move {
                    background_fetch_graph(app).await;
                });
            }
        }
    }

    pub fn on_down(&mut self, app: Arc<Mutex<App>>) {
        match self.active_context {
            ViewContext::Watchlist => {
                self.watchlist.next();
                tokio::spawn(async move {
                    background_fetch_graph(app).await;
                });
            }
        }
    }

    pub fn on_tick(&mut self) {

    }

    pub fn get_quote(&self, ticker: &str) -> Option<&Quote> { 
        self.quote_cache.get(ticker)
    }

    pub fn selected_ticker(&self) -> &str {
        match self.active_context {
            ViewContext::Watchlist => {
                let index = self.watchlist.state.selected().unwrap();
                return &self.watchlist.list[index].symbol;
            }
        }
    }

}

pub async fn initialize(app: Arc<Mutex<App>>) -> Result<ui::Terminal, CliError> {
    let symbols = loader::load_symbols()?;
    {
        let mut app = app.lock().await;
        app.symbols = symbols;

        let watchlist = data::watchlist::get_watch_list(&app.symbols[..]);
        app.watchlist = StatefulList::with_list(watchlist);
        app.watchlist.state.select(Some(0));
        
        let msg = format!("Loaded {} symbols and watchlist.", app.symbols.len());
        app.log.push(msg);
    }

    // Background tasks
    tokio::spawn(async move {
        background_fetch_options(Arc::clone(&app)).await;
        background_fetch_watchlist_quotes(Arc::clone(&app)).await;
        background_fetch_graph(Arc::clone(&app)).await;
    });

    let mut terminal = ui::initialize_terminal()
        .map_err(|_| CliError::InitError { msg: "Failed to initialize terminal." .to_string() })?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(terminal)
}

async fn background_fetch_options(app: Arc<Mutex<App>>) {
    if let Ok(options) = loader::load_options() {
        let mut app = app.lock().await;
        app.options = options;

        let msg = format!("Loaded {} options.", app.options.len());
        app.log.push(msg);
    }
}

async fn background_fetch_watchlist_quotes(app: Arc<Mutex<App>>) { 
    let mut lock = app.lock().await;
    let symbols = &lock.watchlist.list;
    let tickers = symbols.into_iter().map(|s| s.short_name()).collect();
    if let Ok(quotes) = client::get_stock_quotes(tickers).await {
        for quote in quotes.quotes() {
            lock.quote_cache.insert(quote.symbol.clone(), quote.clone());
        }
        lock.log.push("Downloaded watchlist quotes".to_string());
    }
}

async fn background_fetch_graph(app: Arc<Mutex<App>>) {
    let mut lock = app.lock().await;
    let idx = lock.watchlist.state.selected().unwrap();
    let symbol = lock.watchlist.list[idx].symbol.clone();
    if let Some(_) = lock.graph_cache.get(&symbol) {
        return;
    }
    let start_day = util::last_market_open_day();
    let start_date = start_day.and_hms(9,30, 0);
    let end_day = util::last_market_open_day();
    let end_date = end_day.and_hms(16, 0, 1);
    let result = client::get_time_series_data(
        symbol.to_string(), start_date, end_date, 5
    ).await;
    match result {
        Ok(series) => {
            let log = format!("Got timeseries data for ${}.", &symbol);
            lock.log.push(log);
            lock.graph_cache.insert(symbol, series); 
        },
        Err(_) => {
            lock.log.push("Failed to get timeseries data".to_string());
        }
    }
}