use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandCollection {
    pub name: String,
    pub commands: Vec<CommandItem>,
}

impl CommandCollection {
    pub fn new(name: String) -> Self {
        Self {
            name,
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: CommandItem) {
        self.commands.push(command);
    }

    pub fn remove_command(&mut self, index: usize) {
        self.commands.remove(index);
    }

    pub fn update_command(&mut self, index: usize, command: CommandItem) {
        self.commands[index] = command;
    }

    pub fn get_commands(&self) -> &Vec<CommandItem> {
        &self.commands
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItem {
    pub command_text: String,
    pub description: String,
}
