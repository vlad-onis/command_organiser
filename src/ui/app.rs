use ratatui::widgets::ListState;
use std::collections::HashMap;
use thiserror::Error;

use crate::model::command::Command;
use crate::service::command_service::{CommandService, CommandServiceError};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
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
            None => 0,
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
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App {
    pub index: usize,
    pub commands: StatefulList<Command>,
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Command Service failed: {0}")]
    CommandService(#[from] CommandServiceError),
}

impl App {
    pub async fn new() -> Result<App, ApplicationError> {
        let command_service = CommandService::new("commands.db").await?;
        let commands = command_service.get_all_commands().await?;

        // get titles from the db
        Ok(App {
            index: 0,
            commands: StatefulList::with_items(commands),
        })
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.commands.items.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.commands.items.len() - 1;
        }
    }

    pub fn get_by_executable(&self, executable: &str) -> Vec<Command> {
        self.commands
            .items
            .iter()
            .filter(|command| command.executable == executable)
            .cloned()
            .collect()
    }
}
