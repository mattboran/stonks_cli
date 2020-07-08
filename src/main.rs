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

    // let start_date = util::est().ymd(2020, 07, 02);
    // let start_date = start_date.and_hms(9, 30, 0);
    // let end_date = util::est().ymd(2020, 07, 02);
    // let end_date = end_date.and_hms(16, 0, 0);
    // let timeseries = api::client::get_time_series_data("SPY".to_string(), start_date, end_date, 15).await?;

    let app = Arc::new(Mutex::new(App::new()));
    let mut terminal = cli::initialize(Arc::clone(&app)).await?;

    let mut events = Events::new();
    loop {
        let mut app = app.lock().await;
        terminal.draw(|mut f| {
            ui::draw(&mut f, &mut app);
        })?;

        if let Some(event) = events.next().await {
            match event {
                Event::Input(k) => match k {
                    Key::Char(c) => {
                        app.on_key(c);
                    },
                    Key::Up => { 
                        app.on_up()
                    },
                    Key::Down => {
                        app.on_down()
                    },
                    _ => {}
                },
                Event::Tick => {
                    app.on_tick()
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}