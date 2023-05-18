use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};
use std::{error::Error, io};

use super::app::App;

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
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.tabs.next(),
                KeyCode::Left => app.tabs.previous(),
                KeyCode::Down => {
                    let selected_executable_tab = app.tabs.titles.get(app.tabs.index).unwrap(); // This should not fail
                    app.commands.next(selected_executable_tab)
                }
                KeyCode::Up => {
                    let selected_executable_tab = app.tabs.titles.get(app.tabs.index).unwrap(); // This should not fail
                    app.commands.previous(selected_executable_tab);
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::LightYellow));
    f.render_widget(block, size);

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|executable| Spans::from(Span::styled(executable, Style::default().fg(Color::Cyan))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Executables"))
        .select(app.tabs.index)
        .style(Style::default().fg(Color::Rgb(255, 213, 128)))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    draw_commands_pane(f, app, chunks[1])

    // f.render_widget(inner, chunks[1]);
}

fn draw_commands_pane<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    draw_alias_pane(f, app, area);
    // draw_description_and_command_pane(f, app, chunks[1]);
}

fn draw_alias_pane<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);

    let exes = app.executables();
    let current_command_tab = exes.get(app.tabs.index).unwrap(); // This should not fail
    let commands = app.get_by_executable(current_command_tab);

    let aliases: Vec<ListItem> = commands
        .into_iter()
        .map(|command| ListItem::new(vec![Spans::from(Span::raw(command.clone().alias))]))
        .collect();

    let aliases = List::new(aliases)
        .block(Block::default().borders(Borders::ALL).title("Alias list"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(aliases, chunks[0], &mut app.commands.state);

    draw_description_and_command_pane(f, app, chunks[1]);
}

fn draw_description_and_command_pane<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);

    let selected_command = app.get_selected_command();

    let description = selected_command.description.unwrap_or(String::new());

    let description = Paragraph::new(description)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command Description"),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let command = selected_command.command;
    let command =
        Paragraph::new(command).block(Block::default().borders(Borders::ALL).title("Command"));

    f.render_widget(description, chunks[0]);
    f.render_widget(command, chunks[1]);
}
