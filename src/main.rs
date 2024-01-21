use std::process::{self, Stdio};

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
    if let Some(command) = app.command_to_execute {
        println!("{}", command);
        let command_output = process::Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()?;
        println!("{}", String::from_utf8_lossy(&command_output.stdout));
    }
    Ok(())
}
