use crate::models::{Schema, Column, Entity, Position, Dimensions};
use crate::parser::SqlParser;
use crate::layout::LayoutEngine;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    SqlEditor,
    DiagramView,
    Help,
    EntityCreator,
}

pub struct App {
    pub schema: Schema,
    pub mode: AppMode,
    pub selected_entity: Option<String>,
    pub sql_content: String,
    pub _cursor_position: (u16, u16),
    pub should_quit: bool,
    pub layout_engine: LayoutEngine,
    pub entity_creator_buffer: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            schema: Schema::new(),
            mode: AppMode::DiagramView,
            selected_entity: None,
            sql_content: String::new(),
            _cursor_position: (0, 0),
            should_quit: false,
            layout_engine: LayoutEngine::new(800.0, 600.0),
            entity_creator_buffer: String::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::DiagramView => self.handle_diagram_key(key),
            AppMode::SqlEditor => self.handle_sql_editor_key(key),
            AppMode::Help => self.handle_help_key(key),
            AppMode::EntityCreator => self.handle_entity_creator_key(key),
        }
    }

    fn handle_diagram_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('s') => self.mode = AppMode::SqlEditor,
            KeyCode::Char('?') => self.mode = AppMode::Help,
            KeyCode::Char('n') => self.mode = AppMode::EntityCreator,
            KeyCode::Char('i') => self.import_sql(),
            KeyCode::Char('r') => self.refresh_layout(),
            KeyCode::Tab => self.cycle_selected_entity(),
            KeyCode::BackTab => self.cycle_selected_entity_reverse(),
            KeyCode::Delete | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.delete_selected_entity();
            }
            KeyCode::Up => self.move_selected_entity(0, -10),
            KeyCode::Down => self.move_selected_entity(0, 10),
            KeyCode::Left => self.move_selected_entity(-10, 0),
            KeyCode::Right => self.move_selected_entity(10, 0),
            _ => {}
        }
    }

    fn handle_sql_editor_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.mode = AppMode::DiagramView,
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.parse_and_apply_sql();
            }
            KeyCode::Char(c) => {
                self.sql_content.push(c);
            }
            KeyCode::Backspace => {
                self.sql_content.pop();
            }
            KeyCode::Enter => {
                self.sql_content.push('\n');
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

    fn handle_entity_creator_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.entity_creator_buffer.clear();
                self.mode = AppMode::DiagramView;
            }
            KeyCode::Enter => {
                self.create_entity_from_buffer();
                self.entity_creator_buffer.clear();
                self.mode = AppMode::DiagramView;
            }
            KeyCode::Char(c) => {
                self.entity_creator_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.entity_creator_buffer.pop();
            }
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

    fn cycle_selected_entity_reverse(&mut self) {
        let entities: Vec<String> = self.schema.entities.keys().cloned().collect();
        if entities.is_empty() {
            return;
        }

        self.selected_entity = match &self.selected_entity {
            None => Some(entities.last().unwrap().clone()),
            Some(current) => {
                if let Some(pos) = entities.iter().position(|e| e == current) {
                    let prev_pos = if pos == 0 { entities.len() - 1 } else { pos - 1 };
                    Some(entities[prev_pos].clone())
                } else {
                    Some(entities.last().unwrap().clone())
                }
            }
        };
    }

    fn move_selected_entity(&mut self, dx: i32, dy: i32) {
        if let Some(entity_name) = &self.selected_entity.clone() {
            if let Some(entity) = self.schema.entities.get_mut(entity_name) {
                entity.position.x = (entity.position.x + dx as f64).max(0.0).min(800.0);
                entity.position.y = (entity.position.y + dy as f64).max(0.0).min(600.0);
            }
        }
    }

    fn delete_selected_entity(&mut self) {
        if let Some(entity_name) = &self.selected_entity.clone() {
            self.schema.entities.remove(entity_name);
            self.schema.relationships.retain(|r| {
                r.from_table != *entity_name && r.to_table != *entity_name
            });
            self.selected_entity = None;
        }
    }

    fn import_sql(&mut self) {
        if !self.sql_content.is_empty() {
            self.parse_and_apply_sql();
        } else {
            self.mode = AppMode::SqlEditor;
        }
    }

    fn parse_and_apply_sql(&mut self) {
        let parser = SqlParser::new();
        if let Ok(mut new_schema) = parser.parse_sql(&self.sql_content) {
            self.layout_engine.layout_entities(&mut new_schema);
            self.schema = new_schema;
            self.mode = AppMode::DiagramView;
        }
    }

    fn refresh_layout(&mut self) {
        self.layout_engine.layout_entities(&mut self.schema);
    }

    fn create_entity_from_buffer(&mut self) {
        if !self.entity_creator_buffer.trim().is_empty() {
            let entity_name = self.entity_creator_buffer.trim().to_string();
            let new_entity = Entity {
                name: entity_name.clone(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: "INT".to_string(),
                        nullable: false,
                        is_primary_key: true,
                        is_foreign_key: false,
                        references: None,
                    }
                ],
                position: Position {
                    x: 400.0 + (self.schema.entities.len() as f64 * 50.0),
                    y: 300.0 + (self.schema.entities.len() as f64 * 30.0),
                },
                dimensions: Dimensions { width: 20, height: 8 },
            };
            
            self.schema.entities.insert(entity_name.clone(), new_entity);
            self.selected_entity = Some(entity_name);
        }
    }
}