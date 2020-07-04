mod loader;
pub mod ui;
pub mod event;

use tokio::sync::Mutex;

pub use termion::event::Key;

use std::fmt;
use std::error::Error;
use std::sync::Arc;
use std::collections::HashMap;

use crate::data::{self, Symbol, Quote};
use crate::api::client;
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
    pub active_context: ViewContext,
    pub watchlist: StatefulList<Symbol>,
    pub quote_cache: Arc<Mutex<HashMap<String, Quote>>>,
    pub log: Vec<String>,
    pub should_quit: bool,
}

impl App { 
    pub fn new() -> Self { 
        App {
            title: format!("StonksCLI"),
            options: vec![],
            symbols: vec![],
            active_context: ViewContext::Watchlist,
            watchlist: StatefulList::default(),
            quote_cache: Arc::new(Mutex::new(HashMap::new())),
            log: vec![],
            should_quit: false,
        }
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' { 
            self.should_quit = true;
        }
    }

    pub fn on_up(&mut self) {
        match self.active_context {
            ViewContext::Watchlist => self.watchlist.previous()
        }
    }

    pub fn on_down(&mut self) {
        match self.active_context {
            ViewContext::Watchlist => self.watchlist.next()
        }
    }

    pub fn on_tick(&mut self) {

    }
}

pub async fn initialize(app: Arc<Mutex<App>>) -> Result<ui::Terminal, CliError> {
    let symbols = loader::load::<Symbol>()?;
    {
        let mut app = app.lock().await;
        app.symbols = symbols;

        let watchlist = data::watchlist::get_watch_list(&app.symbols[..]);
        app.watchlist = StatefulList::with_list(watchlist);
        app.watchlist.state.select(Some(0));
        
        let msg = format!("Loaded {} symbols and watchlist.", app.symbols.len());
        app.log.push(msg);
    }

    background_fetch_options(Arc::clone(&app));
    background_fetch_watchlist_quotes(Arc::clone(&app));

    let mut terminal = ui::initialize_terminal()
    .map_err(|_| CliError::InitError { msg: "Failed to initialize terminal." .to_string() })?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(terminal)
}

fn background_fetch_options(app: Arc<Mutex<App>>) {
    tokio::spawn(async move {
        if let Ok(options) = loader::load::<data::Option>() {
            let mut app = app.lock().await;
            app.options = options;
    
            let msg = format!("Loaded {} options.", app.options.len());
            app.log.push(msg);
        }
    });
}

fn background_fetch_watchlist_quotes(app: Arc<Mutex<App>>) { 
    tokio::spawn(async move {
        let mut lock = app.lock().await;
        let symbols = &lock.watchlist.list;
        let tickers = symbols.into_iter().map(|s| s.short_name()).collect();
        lock.log.push("Downloading watchlist quotes".to_string());
        if let Ok(quotes) = client::get_stock_quotes(tickers).await {
            let mut lock = app.lock().await;
            let symbols = &lock.watchlist.list;
            let quotes = quotes.quotes();
            // for i in 0..quotes.len() {
                // let symbol = symbols[i].symbol;
                // let quote = quotes[i];
                // let cache = lock.quote_cache.lock().await;
                // cache.insert(symbol, quote);
            // }
            lock.log.push("Downloaded watchlist quotes".to_string());
            // ???
        }
    });
}