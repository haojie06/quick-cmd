pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use self::{app::App, event::EventHandler, tui::Tui};
use anyhow::Result;
use event::Event;
use ratatui::{backend::CrosstermBackend, Terminal};
use update::update;

pub fn startup() -> Result<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(50);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;
    while !app.should_quit {
        tui.draw(&mut app)?;
        loop {
            match tui.events.next()? {
                Event::Tick => {
                    // 固定tickrate刷新ui
                    break;
                }
                Event::Key(key_event) => update(&mut app, key_event),
                Event::Mouse(_mouse_event) => {}
                Event::Resize(_width, _height) => {}
            };
        }
    }
    tui.exit()?;
    Ok(())
}
