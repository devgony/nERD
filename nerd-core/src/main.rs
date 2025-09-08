mod app;
mod models;
mod parser;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') && app.mode == app::AppMode::DiagramView {
                return Ok(());
            }
            app.handle_key(key);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    let main_block = Block::default()
        .title(match app.mode {
            app::AppMode::DiagramView => "ERD Diagram View",
            app::AppMode::SqlEditor => "SQL Editor",
            app::AppMode::Help => "Help",
        })
        .borders(Borders::ALL);

    let content = match app.mode {
        app::AppMode::DiagramView => {
            if app.schema.entities.is_empty() {
                "No entities to display. Press 's' to enter SQL editor."
            } else {
                "Entities will be displayed here. Press Tab to select, 's' for SQL editor, '?' for help."
            }
        }
        app::AppMode::SqlEditor => {
            "SQL Editor - Press Esc to return to diagram view"
        }
        app::AppMode::Help => {
            "Help:\n\
             q - Quit\n\
             s - SQL Editor\n\
             Tab - Select next entity\n\
             ? - This help\n\
             Esc - Return to diagram view"
        }
    };

    let paragraph = Paragraph::new(content)
        .block(main_block)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, chunks[0]);

    let status_bar = Paragraph::new(format!(
        "Mode: {:?} | Press '?' for help | Press 'q' to quit",
        app.mode
    ))
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::Yellow));
    
    f.render_widget(status_bar, chunks[1]);
}
