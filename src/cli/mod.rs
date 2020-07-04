mod loader;
pub mod ui;
pub mod event;

use tokio::sync::Mutex;

use tui::widgets::ListState;
pub use termion::event::Key;

use std::fmt;
use std::error::Error;
use std::sync::Arc;

use crate::data::{self, Symbol};

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

pub struct Watchlist { 
    pub list: Vec<Symbol>,
    pub state: ListState
}
pub struct App { 
    pub title: String,
    pub symbols: Vec<Symbol>,
    pub options: Vec<data::Option>,
    pub watchlist: Watchlist,
    pub log: Vec<String>,
    pub should_quit: bool,
}

impl App { 
    pub fn new() -> Self { 
        let watchlist = Watchlist { 
            list: vec![],
            state: ListState::default(),
        };
        App {
            title: format!("StonksCLI"),
            options: vec![],
            symbols: vec![],
            watchlist,
            log: vec![],
            should_quit: false,
        }
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' { 
            self.should_quit = true;
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
        app.watchlist = Watchlist {
            list: data::watchlist::get_watch_list(&app.symbols[..]), 
            state: ListState::default()
        };
        
        let msg = format!("Loaded {} symbols and watchlist.", app.symbols.len());
        app.log.push(msg);
    }
    
    tokio::spawn(async move {
        if let Ok(options) = loader::load::<data::Option>() {
            let mut app = app.lock().await;
            app.options = options;

            let msg = format!("Loaded {} options.", app.options.len());
            app.log.push(msg);
        }
    });

    let mut terminal = ui::initialize_terminal()
        .map_err(|_| CliError::InitError { msg: "Failed to initialize terminal." .to_string() })?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(terminal)
}

