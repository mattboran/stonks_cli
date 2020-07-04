mod api;
mod cli;
mod data;

use std::sync::Arc;

use tokio::sync::Mutex;

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

    let events = Events::new();
    loop {
        let mut app = app.lock().await;
        terminal.draw(|mut f| {
            ui::draw(&mut f, &mut app);
        })?;

        match events.next()? {
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

        if app.should_quit {
            break;
        }
    }
    Ok(())
}