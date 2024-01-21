use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    app::{App, CurrentScreen},
    tui::Event,
};

// 更新应用状态
pub enum Action {
    Tick,
    NextCommand,
    PrevCommand,
    ChangeScreen(CurrentScreen),
    ExecuteCommand,
    Exit,
    // ...
}

// impl From<Event> for Action {}

/// 根据app状态以及输入的事件获取行为
pub fn get_action(_app: &App, event: Event) -> Option<Action> {
    match event {
        Event::Tick => Some(Action::Tick),
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char('c') => {
                if let KeyModifiers::CONTROL = key_event.modifiers {
                    Some(Action::Exit)
                } else {
                    None
                }
            }
            KeyCode::Char('q') => Some(Action::Exit),
            KeyCode::Char('j') => Some(Action::NextCommand),
            KeyCode::Char('k') => Some(Action::PrevCommand),
            KeyCode::Enter => Some(Action::ExecuteCommand),
            _ => None,
        },
        // _ => None,
    }
}

/// 根据行为更新app状态
pub fn update_app(app: &mut App, action: Action) {
    match action {
        Action::NextCommand => app.command_list.next(),
        Action::PrevCommand => {
            app.command_list.previous();
        }
        Action::Exit => {
            app.quit();
        }
        Action::ExecuteCommand => {
            let command_index = app.command_list.state.selected().unwrap();
            app.execute_command(command_index);
        }
        _ => {}
    }
}
