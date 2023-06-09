use clipboard::{ClipboardContext, ClipboardProvider};
use ratatui::widgets::ListState;
use std::collections::HashMap;
use thiserror::Error;

use crate::model::command::Command;
use crate::service::command_service::{CommandService, CommandServiceError};

pub struct TabState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabState {
    pub fn new(titles: Vec<String>) -> TabState {
        TabState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: HashMap<String, Vec<T>>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: HashMap<String, Vec<T>>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self, selected_executable_tab: &str) {
        let i = match self.state.selected() {
            Some(i) => {
                let items = &self.items[selected_executable_tab];
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, selected_executable_tab: &str) {
        let i = match self.state.selected() {
            Some(i) => {
                let items = &self.items[selected_executable_tab];
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_index(&self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => 0,
        }
    }
}

pub struct App {
    pub commands: StatefulList<Command>,
    pub tabs: TabState,
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Command Service failed: {0}")]
    CommandService(#[from] CommandServiceError),

    #[error("Failed to create clipboard context: {0}")]
    ClipBoardError(#[from] Box<dyn std::error::Error>),
}

impl App {
    pub async fn new() -> Result<App, ApplicationError> {
        let command_service = CommandService::new("commands.db").await?;
        let db_commands = command_service.get_all_commands().await?;

        let mut commands: HashMap<String, Vec<Command>> = HashMap::new();
        db_commands.into_iter().for_each(|command| {
            let entry = commands
                .entry(command.clone().executable)
                .or_insert(Vec::new());
            entry.push(command)
        });

        let tabs = TabState::new(commands.keys().cloned().collect());

        // get titles from the db
        Ok(App {
            commands: StatefulList::with_items(commands),
            tabs,
        })
    }

    pub fn executables(&self) -> Vec<String> {
        self.commands.items.keys().cloned().collect()
    }

    pub fn get_by_executable(&self, executable: &str) -> Vec<Command> {
        self.commands.items[executable].clone()
    }

    pub fn get_selected_command(&self) -> Command {
        let executable = self.get_selected_executable();
        let selected_command_index = self.commands.selected_index();

        // This should not fail
        self.commands.items[&executable]
            .get(selected_command_index)
            .unwrap()
            .clone()
    }

    pub fn get_selected_executable(&self) -> String {
        self.tabs.titles.get(self.tabs.index).unwrap().clone()
    }

    pub fn save_command_to_clipboard(&self) -> Result<(), ApplicationError> {
        let command = self.get_selected_command().command.to_owned();
        let mut clipboard_context: ClipboardContext = ClipboardProvider::new()?;
        clipboard_context.set_contents(command.clone());

        Ok(())
    }
}
