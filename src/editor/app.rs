use std::collections::HashMap;

use anyhow::Result;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub pairs: HashMap<String, String>,
    pub key_input: String,
    pub value_input: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
            key_input: String::new(),
            value_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_kv(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        self.currently_editing = match self.currently_editing {
            Some(CurrentlyEditing::Key) => Some(CurrentlyEditing::Value),
            Some(CurrentlyEditing::Value) => Some(CurrentlyEditing::Key),
            None => Some(CurrentlyEditing::Key),
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
