mod app;
mod models;
mod parser;
mod layout;
mod render;
mod sync;

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
    use render::{DiagramRenderer, render_help_screen, render_sql_editor, render_entity_creator};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    match app.mode {
        app::AppMode::DiagramView => {
            let renderer = DiagramRenderer::new(800, 600);
            renderer.render(f, &app.schema, chunks[0], &app.selected_entity);
        }
        app::AppMode::SqlEditor => {
            render_sql_editor(f, &app.sql_content, chunks[0]);
        }
        app::AppMode::EntityCreator => {
            render_entity_creator(f, &app.entity_creator_buffer, chunks[0]);
        }
        app::AppMode::Help => {
            render_help_screen(f, chunks[0]);
        }
    }

    let mode_text = match app.mode {
        app::AppMode::DiagramView => "Diagram",
        app::AppMode::SqlEditor => "SQL Editor", 
        app::AppMode::Help => "Help",
        app::AppMode::EntityCreator => "New Entity",
    };

    let status_message = if app.schema.entities.len() > 0 {
        format!(
            "Mode: {} | Entities: {} | Sample E-commerce Schema Loaded | Press '?' for help | Press 'q' to quit",
            mode_text,
            app.schema.entities.len()
        )
    } else {
        format!(
            "Mode: {} | Entities: {} | Press '?' for help | Press 'q' to quit",
            mode_text,
            app.schema.entities.len()
        )
    };

    let status_bar = Paragraph::new(status_message)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));
    
    f.render_widget(status_bar, chunks[1]);
}
