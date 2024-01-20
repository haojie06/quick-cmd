use std::{io::stderr, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use self::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

pub mod app;
pub mod ui;

pub fn startup() -> Result<()> {
    // prepare
    enable_raw_mode()?;
    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run(&mut terminal, &mut app);
    // clear
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    // 输出最终结果
    if let Ok(t) = res {
        if t {
            app.print_json()?;
        }
    } else if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
    Ok(())
}

fn run<T: Backend>(terminal: &mut Terminal<T>, app: &mut App) -> Result<bool> {
    loop {
        // 绘制ui
        terminal.draw(|f| ui(f, app))?;
        // 处理输入
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => {
                            app.current_screen = CurrentScreen::Editing;
                            app.currently_editing = Some(CurrentlyEditing::Key);
                        }
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') => {
                            return Ok(false);
                        }
                        _ => {}
                    },
                    CurrentScreen::Editing => match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.save_kv();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => match &app.currently_editing {
                            Some(CurrentlyEditing::Key) => {
                                let _ = &app.key_input.pop();
                            }
                            Some(CurrentlyEditing::Value) => {
                                let _ = &app.value_input.pop();
                            }
                            None => {}
                        },
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Tab => app.toggle_editing(),
                        KeyCode::Char(c) => match &app.currently_editing {
                            Some(CurrentlyEditing::Key) => {
                                app.key_input.push(c);
                            }
                            Some(CurrentlyEditing::Value) => {
                                app.value_input.push(c);
                            }
                            None => {}
                        },
                        _ => {}
                    },
                }
            }
        }
    }
}
