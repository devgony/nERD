use crate::models::Schema;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    SqlEditor,
    DiagramView,
    Help,
}

pub struct App {
    pub schema: Schema,
    pub mode: AppMode,
    pub selected_entity: Option<String>,
    pub _sql_content: String,
    pub _cursor_position: (u16, u16),
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            schema: Schema::new(),
            mode: AppMode::DiagramView,
            selected_entity: None,
            _sql_content: String::new(),
            _cursor_position: (0, 0),
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::DiagramView => self.handle_diagram_key(key),
            AppMode::SqlEditor => self.handle_sql_editor_key(key),
            AppMode::Help => self.handle_help_key(key),
        }
    }

    fn handle_diagram_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('s') => self.mode = AppMode::SqlEditor,
            KeyCode::Char('?') => self.mode = AppMode::Help,
            KeyCode::Tab => self.cycle_selected_entity(),
            _ => {}
        }
    }

    fn handle_sql_editor_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.mode = AppMode::DiagramView,
            KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_help_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.mode = AppMode::DiagramView,
            _ => {}
        }
    }

    fn cycle_selected_entity(&mut self) {
        let entities: Vec<String> = self.schema.entities.keys().cloned().collect();
        if entities.is_empty() {
            return;
        }

        self.selected_entity = match &self.selected_entity {
            None => Some(entities[0].clone()),
            Some(current) => {
                if let Some(pos) = entities.iter().position(|e| e == current) {
                    let next_pos = (pos + 1) % entities.len();
                    Some(entities[next_pos].clone())
                } else {
                    Some(entities[0].clone())
                }
            }
        };
    }
}