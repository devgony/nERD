use crate::models::{Schema, Column, Entity, Position, Dimensions};
use crate::parser::SqlParser;
use crate::layout::LayoutEngine;
use crate::sync::SchemaSync;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    SqlEditor,
    DiagramView,
    Help,
    EntityCreator,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VimMode {
    Normal,
    Insert,
}

pub struct App {
    pub schema: Schema,
    pub mode: AppMode,
    pub selected_entity: Option<String>,
    pub sql_content: String,
    pub vim_mode: VimMode,
    pub cursor_position: usize,
    pub should_quit: bool,
    pub layout_engine: LayoutEngine,
    pub entity_creator_buffer: String,
    pub schema_sync: SchemaSync,
    pub last_generated_sql: String,
}

impl App {
    pub fn new() -> Self {
        let sample_sql = r#"-- Sample E-commerce Schema with Foreign Keys
CREATE TABLE users (
    id INT PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE categories (
    id INT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT
);

CREATE TABLE products (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    category_id INT NOT NULL,
    stock_quantity INT DEFAULT 0,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE orders (
    id INT PRIMARY KEY,
    user_id INT NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    total_amount DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE order_items (
    id INT PRIMARY KEY,
    order_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);"#.to_string();

        let mut app = Self {
            schema: Schema::new(),
            mode: AppMode::DiagramView,
            selected_entity: None,
            sql_content: sample_sql,
            vim_mode: VimMode::Normal,
            cursor_position: 0,
            should_quit: false,
            layout_engine: LayoutEngine::new(800.0, 600.0),
            entity_creator_buffer: String::new(),
            schema_sync: SchemaSync::new(),
            last_generated_sql: String::new(),
        };

        // Parse the sample SQL and create the initial diagram
        app.parse_and_apply_sql();
        app
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
            KeyCode::Char('s') => {
                self.mode = AppMode::SqlEditor;
                self.vim_mode = VimMode::Normal;
                // Keep cursor at current position or end of content if beyond
                self.cursor_position = self.cursor_position.min(self.sql_content.len());
            }
            KeyCode::Char('?') => self.mode = AppMode::Help,
            KeyCode::Char('n') => self.mode = AppMode::EntityCreator,
            KeyCode::Char('i') => self.import_sql(),
            KeyCode::Char('r') => self.refresh_layout(),
            KeyCode::Char('g') => self.generate_sql_from_schema(),
            KeyCode::Char('v') => self.validate_schema(),
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
        match self.vim_mode {
            VimMode::Normal => self.handle_vim_normal_mode(key),
            VimMode::Insert => self.handle_vim_insert_mode(key),
        }
    }

    fn handle_vim_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.mode = AppMode::DiagramView,
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.sync_sql_changes();
            }
            
            // VIM normal mode commands
            KeyCode::Char('i') => {
                self.vim_mode = VimMode::Insert;
            }
            KeyCode::Char('a') => {
                self.vim_mode = VimMode::Insert;
                self.cursor_position = self.cursor_position.saturating_add(1).min(self.sql_content.len());
            }
            KeyCode::Char('A') => {
                self.vim_mode = VimMode::Insert;
                // Move to end of current line
                self.move_to_end_of_line();
            }
            KeyCode::Char('I') => {
                self.vim_mode = VimMode::Insert;
                // Move to beginning of current line
                self.move_to_beginning_of_line();
            }
            KeyCode::Char('o') => {
                self.vim_mode = VimMode::Insert;
                self.insert_new_line_after();
            }
            KeyCode::Char('O') => {
                self.vim_mode = VimMode::Insert;
                self.insert_new_line_before();
            }
            
            // Movement commands
            KeyCode::Char('h') | KeyCode::Left => {
                self.cursor_position = self.cursor_position.saturating_sub(1);
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.cursor_position = self.cursor_position.saturating_add(1).min(self.sql_content.len());
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_cursor_down();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_cursor_up();
            }
            KeyCode::Char('w') => {
                self.move_word_forward();
            }
            KeyCode::Char('b') => {
                self.move_word_backward();
            }
            KeyCode::Char('0') => {
                self.move_to_beginning_of_line();
            }
            KeyCode::Char('$') => {
                self.move_to_end_of_line();
            }
            
            // Delete commands
            KeyCode::Char('x') => {
                self.delete_char_at_cursor();
            }
            KeyCode::Char('X') => {
                self.delete_char_before_cursor();
            }
            KeyCode::Char('d') => {
                // Simple implementation: dd deletes current line
                // In a full vim implementation, this would handle more complex delete operations
                self.delete_current_line();
            }
            
            _ => {}
        }
    }

    fn handle_vim_insert_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.vim_mode = VimMode::Normal;
                // Move cursor back one position when exiting insert mode (vim behavior)
                self.cursor_position = self.cursor_position.saturating_sub(1);
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.sync_sql_changes();
            }
            KeyCode::Char(c) => {
                self.insert_char_at_cursor(c);
            }
            KeyCode::Backspace => {
                self.delete_char_before_cursor();
            }
            KeyCode::Enter => {
                self.insert_char_at_cursor('\n');
            }
            KeyCode::Left => {
                self.cursor_position = self.cursor_position.saturating_sub(1);
            }
            KeyCode::Right => {
                self.cursor_position = self.cursor_position.saturating_add(1).min(self.sql_content.len());
            }
            KeyCode::Up => {
                self.move_cursor_up();
            }
            KeyCode::Down => {
                self.move_cursor_down();
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
            self.vim_mode = VimMode::Normal;
            self.cursor_position = self.sql_content.len();
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

    fn sync_sql_changes(&mut self) {
        if let Ok(has_changes) = self.schema_sync.merge_sql_changes(&mut self.schema, &self.sql_content) {
            if has_changes {
                self.layout_engine.layout_entities(&mut self.schema);
                self.mode = AppMode::DiagramView;
            }
        }
    }

    fn generate_sql_from_schema(&mut self) {
        self.sql_content = self.schema_sync.generate_sql(&self.schema);
        self.last_generated_sql = self.sql_content.clone();
        self.mode = AppMode::SqlEditor;
        self.vim_mode = VimMode::Normal;
        self.cursor_position = 0; // Start at beginning of generated content
    }

    fn validate_schema(&mut self) {
        let _errors = self.schema_sync.validate_schema(&self.schema);
        // For now, just refresh layout - in a full implementation we'd show validation errors
        self.refresh_layout();
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

    // VIM helper methods for cursor movement and text manipulation
    fn insert_char_at_cursor(&mut self, c: char) {
        if self.cursor_position <= self.sql_content.len() {
            self.sql_content.insert(self.cursor_position, c);
            self.cursor_position += c.len_utf8();
        }
    }

    fn delete_char_at_cursor(&mut self) {
        if self.cursor_position < self.sql_content.len() {
            self.sql_content.remove(self.cursor_position);
        }
    }

    fn delete_char_before_cursor(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            if self.cursor_position < self.sql_content.len() {
                self.sql_content.remove(self.cursor_position);
            }
        }
    }

    fn move_cursor_up(&mut self) {
        let lines: Vec<&str> = self.sql_content.split('\n').collect();
        if lines.is_empty() {
            return;
        }

        let (current_line, col_in_line) = self.get_line_and_column();
        if current_line > 0 {
            let prev_line_len = lines[current_line - 1].len();
            let new_col = col_in_line.min(prev_line_len);
            self.cursor_position = self.get_position_from_line_col(current_line - 1, new_col);
        }
    }

    fn move_cursor_down(&mut self) {
        let lines: Vec<&str> = self.sql_content.split('\n').collect();
        let (current_line, col_in_line) = self.get_line_and_column();
        
        if current_line < lines.len() - 1 {
            let next_line_len = lines[current_line + 1].len();
            let new_col = col_in_line.min(next_line_len);
            self.cursor_position = self.get_position_from_line_col(current_line + 1, new_col);
        }
    }

    fn move_to_beginning_of_line(&mut self) {
        let (current_line, _) = self.get_line_and_column();
        self.cursor_position = self.get_position_from_line_col(current_line, 0);
    }

    fn move_to_end_of_line(&mut self) {
        let lines: Vec<&str> = self.sql_content.split('\n').collect();
        let (current_line, _) = self.get_line_and_column();
        if current_line < lines.len() {
            let line_len = lines[current_line].len();
            self.cursor_position = self.get_position_from_line_col(current_line, line_len);
        }
    }

    fn move_word_forward(&mut self) {
        let chars: Vec<char> = self.sql_content.chars().collect();
        let mut pos = self.cursor_position;
        
        // Skip current word
        while pos < chars.len() && chars[pos].is_alphanumeric() {
            pos += 1;
        }
        
        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }
        
        self.cursor_position = pos.min(self.sql_content.len());
    }

    fn move_word_backward(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        
        let chars: Vec<char> = self.sql_content.chars().collect();
        let mut pos = self.cursor_position.saturating_sub(1);
        
        // Skip whitespace
        while pos > 0 && chars[pos].is_whitespace() {
            pos = pos.saturating_sub(1);
        }
        
        // Skip to beginning of word
        while pos > 0 && chars[pos.saturating_sub(1)].is_alphanumeric() {
            pos = pos.saturating_sub(1);
        }
        
        self.cursor_position = pos;
    }

    fn insert_new_line_after(&mut self) {
        self.move_to_end_of_line();
        self.insert_char_at_cursor('\n');
    }

    fn insert_new_line_before(&mut self) {
        self.move_to_beginning_of_line();
        self.insert_char_at_cursor('\n');
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    fn delete_current_line(&mut self) {
        let (current_line, _) = self.get_line_and_column();
        let lines: Vec<String> = self.sql_content.split('\n').map(|s| s.to_string()).collect();
        
        if current_line < lines.len() {
            let mut new_lines = lines;
            new_lines.remove(current_line);
            self.sql_content = new_lines.join("\n");
            
            // Adjust cursor position
            if current_line > 0 && !new_lines.is_empty() {
                self.cursor_position = self.get_position_from_line_col(current_line.saturating_sub(1), 0);
            } else {
                self.cursor_position = 0;
            }
        }
    }

    fn get_line_and_column(&self) -> (usize, usize) {
        let mut line = 0;
        let mut col = 0;
        let mut pos = 0;
        
        for ch in self.sql_content.chars() {
            if pos >= self.cursor_position {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            pos += ch.len_utf8();
        }
        
        (line, col)
    }

    fn get_position_from_line_col(&self, target_line: usize, target_col: usize) -> usize {
        let mut line = 0;
        let mut col = 0;
        let mut pos = 0;
        
        for ch in self.sql_content.chars() {
            if line == target_line && col == target_col {
                return pos;
            }
            
            if line > target_line {
                return pos;
            }
            
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            pos += ch.len_utf8();
        }
        
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vim_mode_initialization() {
        let app = App::new();
        assert_eq!(app.vim_mode, VimMode::Normal);
        assert_eq!(app.cursor_position, 0);
    }

    #[test]
    fn test_cursor_movement_basic() {
        let mut app = App::new();
        app.sql_content = "Hello\nWorld".to_string();
        app.cursor_position = 0;
        
        // Test basic movement
        let (line, col) = app.get_line_and_column();
        assert_eq!(line, 0);
        assert_eq!(col, 0);
        
        // Move to position 3 (middle of "Hello")
        app.cursor_position = 3;
        let (line, col) = app.get_line_and_column();
        assert_eq!(line, 0);
        assert_eq!(col, 3);
        
        // Move to second line
        app.cursor_position = 6; // After newline
        let (line, col) = app.get_line_and_column();
        assert_eq!(line, 1);
        assert_eq!(col, 0);
    }

    #[test]
    fn test_insert_char_at_cursor() {
        let mut app = App::new();
        app.sql_content = "Hello".to_string();
        app.cursor_position = 2;
        
        app.insert_char_at_cursor('X');
        assert_eq!(app.sql_content, "HeXllo");
        assert_eq!(app.cursor_position, 3); // Cursor moved after inserted char
    }

    #[test]
    fn test_delete_char_at_cursor() {
        let mut app = App::new();
        app.sql_content = "Hello".to_string();
        app.cursor_position = 2;
        
        app.delete_char_at_cursor();
        assert_eq!(app.sql_content, "Helo");
        assert_eq!(app.cursor_position, 2); // Cursor stays at same position
    }
}