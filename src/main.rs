use anyhow::{Result, Ok};
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

mod app;
mod ui;
mod update;
mod tui;
mod event;

// All the main log initialized here
#[tokio::main]
async fn main() -> Result<()> {

    let mut app = App { should_quit: false, item_counter: 80, cursor: 0 };
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Error => {},
            Event::Tick => {},
            Event::Key(key_event) => update(&mut app, key_event),
        };
    }

    tui.exit()?;
    Ok(())

}
