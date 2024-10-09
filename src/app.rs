use anyhow::Result;
use gluesql_core::data::Schema;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tui_scrollview::ScrollViewState;

#[derive(Default)]
pub struct App {
    pub sql_text: String,
    pub scroll_view_state: ScrollViewState,
    pub state: AppState,
}

#[derive(Default, PartialEq)]
pub(crate) enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn new(sql_text: String) -> App {
        App {
            sql_text,
            ..Default::default()
        }
    }

    pub fn get_schemas(&self) -> Vec<Schema> {
        self.sql_text
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect::<Vec<_>>()
    }

    pub fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('j') | Down => self.scroll_view_state.scroll_down(),
                Char('k') | Up => self.scroll_view_state.scroll_up(),
                Char('h') | Left => self.scroll_view_state.scroll_left(),
                Char('l') | Right => self.scroll_view_state.scroll_right(),
                Char('f') | PageDown => self.scroll_view_state.scroll_page_down(),
                Char('b') | PageUp => self.scroll_view_state.scroll_page_up(),
                Char('g') | Home => self.scroll_view_state.scroll_to_top(),
                Char('G') | End => self.scroll_view_state.scroll_to_bottom(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}
