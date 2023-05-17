use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};
use std::{error::Error, io};
use thiserror::Error;

use crate::service::command_service::{CommandService, CommandServiceError};

struct App {
    pub executables: Vec<String>,
    pub index: usize,
    pub command_service: CommandService,
}

#[derive(Debug, Error)]
enum ApplicationError {
    #[error("Command Service failed: {0}")]
    CommandService(#[from] CommandServiceError),
}

impl App {
    async fn new() -> Result<App, ApplicationError> {
        let command_service = CommandService::new("commands.db").await?;
        let executables: Vec<String> = command_service
            .get_all_commands()
            .await?
            .into_iter()
            .map(|command| command.executable)
            .collect();

        // get titles from the db
        Ok(App {
            executables,
            index: 0,
            command_service,
        })
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.executables.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.executables.len() - 1;
        }
    }
}

pub async fn run_terminal() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new().await?;
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::LightYellow));
    f.render_widget(block, size);

    let titles = app
        .executables
        .iter()
        .map(|t| Spans::from(Span::styled(t, Style::default().fg(Color::Cyan))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Executables"))
        .select(app.index)
        .style(Style::default().fg(Color::Rgb(255, 213, 128)))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    match app.index {
        0 => draw_commands_pane(f, app, chunks[1]),
        // 1 => Block::default().title("Inner 1").borders(Borders::ALL),
        // 2 => Block::default().title("Inner 2").borders(Borders::ALL),
        // 3 => Block::default().title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    // f.render_widget(inner, chunks[1]);
}

fn draw_commands_pane<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);
    draw_alias_pane(f, app, chunks[0]);
    draw_description_pane(f, app, chunks[1]);
}

fn draw_alias_pane<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let aliases = Block::default().title("Inner 1").borders(Borders::ALL);
    f.render_widget(aliases, area);
}

fn draw_description_pane<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let descriptions = Block::default().title("Inner 2").borders(Borders::ALL);
    f.render_widget(descriptions, area);
}
