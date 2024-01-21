use anyhow::Result;
use app::App;
use tui::Tui;
use update::{get_action, update_app};

pub mod counter;
pub mod editor;

pub mod app;
pub mod model;
pub mod tui;
pub mod ui;
pub mod update;

fn main() -> Result<()> {
    // counter::startup()?;
    // editor::startup()?;
    let mut app = App::new();
    let mut tui = Tui::new(60);
    Tui::enter()?;
    while !app.should_quit {
        // 绘制ui
        tui.terminal.draw(|f| ui::ui(f, &mut app))?;
        // 处理事件/输入
        let event = tui.next_event()?;
        if let Some(action) = get_action(&app, event) {
            update_app(&mut app, action);
        };
    }
    Tui::exit()?;
    Ok(())
}
