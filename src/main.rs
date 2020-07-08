mod api;
mod cli;
mod data;
mod util;

use std::sync::Arc;

use tokio::sync::Mutex;
use chrono::{DateTime, Datelike, Timelike, TimeZone, Utc};

use cli::{
    App,
    Key,
    event::{Event, Events},
    ui
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app = Arc::new(Mutex::new(App::new()));
    let mut terminal = cli::initialize(Arc::clone(&app)).await?;

    let mut events = Events::new();
    loop {
        let mut lock = app.lock().await;
        terminal.draw(|mut f| {
            ui::draw(&mut f, &mut lock);
        })?;

        if let Some(event) = events.next().await {
            match event {
                Event::Input(k) => match k {
                    Key::Char(c) => {
                        lock.on_key(c);
                    },
                    Key::Up => { 
                        lock.on_up(Arc::clone(&app))
                    },
                    Key::Down => {
                        lock.on_down(Arc::clone(&app))
                    },
                    _ => {}
                },
                Event::Tick => {
                    lock.on_tick()
                }
            }
        }

        if lock.should_quit {
            break;
        }
    }
    Ok(())
}