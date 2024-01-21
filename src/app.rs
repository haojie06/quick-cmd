use ratatui::widgets::ListState;

use crate::model::{CommandCollection, CommandItem};

pub struct App {
    pub current_screen: CurrentScreen,
    pub should_quit: bool,
    pub command_list: StatefulList<CommandItem>,
    pub command_to_execute: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let test_list: Vec<_> = (0..100)
            .map(|n| CommandItem {
                command_text: format!("echo {}", n),
                description: format!("print {}", n),
            })
            .collect();
        Self {
            current_screen: CurrentScreen::Main,
            should_quit: false,
            command_list: StatefulList::with_items(test_list),
            command_to_execute: None,
            // main_screen_status: MainScreenStatus {
            //     command_collections: vec![],
            //     command_collections_index: 0,
            //     command_index: 0,
            // },
        }
    }

    pub fn toggle_editing(&mut self) {
        if let CurrentScreen::Edit(editing) = &self.current_screen {
            match editing {
                CurrentlyEditing::Command => {
                    self.current_screen = CurrentScreen::Edit(CurrentlyEditing::Description)
                }
                CurrentlyEditing::Description => {
                    self.current_screen = CurrentScreen::Edit(CurrentlyEditing::Command)
                }
            }
        }
    }

    pub fn execute_command(&mut self, command_index: usize) {
        self.should_quit = true;
        self.command_to_execute = Some(self.command_list.items[command_index].command_text.clone());
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

pub enum CurrentScreen {
    Main, // command viewer
    Edit(CurrentlyEditing),
    EditConfirm,
    ExecuteConfirm,
}

pub enum CurrentlyEditing {
    Command,
    Description,
}

// pub struct MainScreenStatus {
//     pub command_collections: Vec<CommandCollection>,
//     pub command_collections_index: u16,
//     pub command_index: u16,
// }

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub last_selected: Option<usize>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        StatefulList {
            state,
            items,
            last_selected: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}
