use crate::models::{Entity, Schema};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub struct DiagramRenderer {
    canvas_width: u16,
    canvas_height: u16,
}

impl DiagramRenderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            canvas_width: width,
            canvas_height: height,
        }
    }

    pub fn render(&self, f: &mut Frame, schema: &Schema, area: Rect, selected_entity: &Option<String>) {
        if schema.entities.is_empty() {
            self.render_empty_state(f, area);
            return;
        }

        self.render_entities(f, schema, area, selected_entity);
        self.render_relationships(f, schema, area);
    }

    fn render_empty_state(&self, f: &mut Frame, area: Rect) {
        let empty_message = Paragraph::new("Loading sample schema... Press 's' to view SQL or 'r' to refresh layout.")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("ERD Diagram"));
        f.render_widget(empty_message, area);
    }

    fn render_entities(
        &self,
        f: &mut Frame,
        schema: &Schema,
        area: Rect,
        selected_entity: &Option<String>,
    ) {
        for (entity_name, entity) in &schema.entities {
            let entity_area = self.calculate_entity_area(entity, area);
            let is_selected = selected_entity.as_ref() == Some(entity_name);
            
            self.render_single_entity(f, entity, entity_area, is_selected);
        }
    }

    fn render_single_entity(
        &self,
        f: &mut Frame,
        entity: &Entity,
        area: Rect,
        is_selected: bool,
    ) {
        let border_style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let title_style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        };

        let block = Block::default()
            .title(entity.name.clone())
            .title_style(title_style)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner_area = block.inner(area);

        let columns: Vec<ListItem> = entity
            .columns
            .iter()
            .map(|col| {
                let key_indicator = if col.is_primary_key {
                    "🗝 "
                } else if col.is_foreign_key {
                    "🔗 "
                } else {
                    "  "
                };

                let nullable_indicator = if col.nullable { "?" } else { "" };

                let column_text = format!(
                    "{}{} {}{}",
                    key_indicator,
                    col.name,
                    col.data_type,
                    nullable_indicator
                );

                let style = if col.is_primary_key {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else if col.is_foreign_key {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(column_text).style(style)
            })
            .collect();

        let column_list = List::new(columns).block(Block::default());

        f.render_widget(block, area);
        f.render_widget(column_list, inner_area);
    }

    fn render_relationships(
        &self,
        f: &mut Frame,
        schema: &Schema,
        area: Rect,
    ) {
        for relationship in &schema.relationships {
            if let (Some(from_entity), Some(to_entity)) = (
                schema.entities.get(&relationship.from_table),
                schema.entities.get(&relationship.to_table),
            ) {
                self.render_relationship_line(f, from_entity, to_entity, area);
            }
        }
    }

    fn render_relationship_line(
        &self,
        f: &mut Frame,
        from_entity: &Entity,
        to_entity: &Entity,
        area: Rect,
    ) {
        let from_area = self.calculate_entity_area(from_entity, area);
        let to_area = self.calculate_entity_area(to_entity, area);

        // Calculate connection points on entity edges instead of centers
        let (from_point, to_point) = self.calculate_connection_points(&from_area, &to_area);

        self.draw_connection_line(f, from_point, to_point, area);
    }

    fn calculate_connection_points(&self, from_area: &Rect, to_area: &Rect) -> ((u16, u16), (u16, u16)) {
        let from_center_x = from_area.x + from_area.width / 2;
        let from_center_y = from_area.y + from_area.height / 2;
        let to_center_x = to_area.x + to_area.width / 2;
        let to_center_y = to_area.y + to_area.height / 2;

        // Determine which edges to connect based on relative positions
        let from_point = if to_center_x > from_center_x {
            // Connect from right edge
            (from_area.x + from_area.width - 1, from_center_y)
        } else if to_center_x < from_center_x {
            // Connect from left edge
            (from_area.x, from_center_y)
        } else if to_center_y > from_center_y {
            // Connect from bottom edge
            (from_center_x, from_area.y + from_area.height - 1)
        } else {
            // Connect from top edge
            (from_center_x, from_area.y)
        };

        let to_point = if from_center_x > to_center_x {
            // Connect to right edge
            (to_area.x + to_area.width - 1, to_center_y)
        } else if from_center_x < to_center_x {
            // Connect to left edge
            (to_area.x, to_center_y)
        } else if from_center_y > to_center_y {
            // Connect to bottom edge
            (to_center_x, to_area.y + to_area.height - 1)
        } else {
            // Connect to top edge
            (to_center_x, to_area.y)
        };

        (from_point, to_point)
    }

    fn draw_connection_line(
        &self,
        f: &mut Frame,
        from: (u16, u16),
        to: (u16, u16),
        area: Rect,
    ) {
        // Draw a proper line between the two points
        self.draw_line_between_points(f, from, to, area);
    }

    fn draw_line_between_points(
        &self,
        f: &mut Frame,
        from: (u16, u16),
        to: (u16, u16),
        area: Rect,
    ) {
        let dx = to.0 as i32 - from.0 as i32;
        let dy = to.1 as i32 - from.1 as i32;
        
        // Use Bresenham's line algorithm for proper line drawing
        let steps = dx.abs().max(dy.abs());
        
        if steps == 0 {
            return;
        }
        
        let x_step = dx as f32 / steps as f32;
        let y_step = dy as f32 / steps as f32;
        
        for i in 0..=steps {
            let x = (from.0 as f32 + i as f32 * x_step) as u16;
            let y = (from.1 as f32 + i as f32 * y_step) as u16;
            
            if x < area.x + area.width && y < area.y + area.height {
                let point_area = Rect {
                    x,
                    y,
                    width: 1,
                    height: 1,
                };
                
                // Choose line character based on direction
                let line_char = if dx.abs() > dy.abs() {
                    if dx > 0 { "─" } else { "─" }
                } else if dy.abs() > dx.abs() {
                    if dy > 0 { "│" } else { "│" }
                } else {
                    // Diagonal
                    if (dx > 0 && dy > 0) || (dx < 0 && dy < 0) { "╲" } else { "╱" }
                };
                
                let line_widget = Paragraph::new(line_char)
                    .style(Style::default().fg(Color::Red));
                
                if point_area.intersects(area) {
                    f.render_widget(line_widget, point_area);
                }
            }
        }
        
        // Draw arrow heads to show direction (from FK to PK)
        self.draw_arrow_head(f, to, from, area);
    }
    
    fn draw_arrow_head(
        &self,
        f: &mut Frame,
        tip: (u16, u16),
        from: (u16, u16),
        area: Rect,
    ) {
        let dx = from.0 as i32 - tip.0 as i32;
        let dy = from.1 as i32 - tip.1 as i32;
        
        // Determine arrow character based on direction
        let arrow_char = if dx.abs() > dy.abs() {
            if dx > 0 { "◄" } else { "►" }
        } else {
            if dy > 0 { "▲" } else { "▼" }
        };
        
        let arrow_area = Rect {
            x: tip.0,
            y: tip.1,
            width: 1,
            height: 1,
        };
        
        let arrow_widget = Paragraph::new(arrow_char)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        
        if arrow_area.intersects(area) {
            f.render_widget(arrow_widget, arrow_area);
        }
    }

    fn calculate_entity_area(&self, entity: &Entity, canvas_area: Rect) -> Rect {
        let x_ratio = entity.position.x / (self.canvas_width as f64);
        let y_ratio = entity.position.y / (self.canvas_height as f64);

        let x = (x_ratio * canvas_area.width as f64) as u16 + canvas_area.x;
        let y = (y_ratio * canvas_area.height as f64) as u16 + canvas_area.y;

        let width = entity.dimensions.width.min(canvas_area.width / 4);
        let height = entity.dimensions.height.min(canvas_area.height / 4);

        Rect { x, y, width, height }
    }
}

pub fn render_help_screen(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from("nERD - Terminal ERD Tool"),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  q          - Quit application"),
        Line::from("  s          - Switch to SQL editor mode"),
        Line::from("  n          - Create new entity"),
        Line::from("  i          - Import SQL (or switch to editor)"),
        Line::from("  g          - Generate SQL from diagram"),
        Line::from("  r          - Refresh/re-layout diagram"),
        Line::from("  v          - Validate schema"),
        Line::from("  ?          - Show this help screen"),
        Line::from("  Esc        - Return to diagram view"),
        Line::from(""),
        Line::from("Entity Selection:"),
        Line::from("  Tab        - Select next entity"),
        Line::from("  Shift+Tab  - Select previous entity"),
        Line::from("  ↑↓←→       - Move selected entity"),
        Line::from("  Ctrl+D/Del - Delete selected entity"),
        Line::from(""),
        Line::from("SQL Editor:"),
        Line::from("  Ctrl+Enter - Sync SQL changes to diagram"),
        Line::from("  Type       - Enter/edit SQL statements"),
        Line::from("  g          - Generate SQL from current diagram"),
        Line::from(""),
        Line::from("Symbols:"),
        Line::from("  🗝         - Primary key column"),
        Line::from("  🔗         - Foreign key column"),
        Line::from("  ?          - Nullable column"),
        Line::from("  ─│╲╱►◄▲▼   - Red relationship lines with arrows"),
        Line::from(""),
        Line::from("Selected entities are highlighted in yellow."),
        Line::from("Relationship lines are drawn in red between connected tables."),
        Line::from("Press any key to return."),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("Help")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));

    let centered_area = centered_rect(60, 80, area);
    f.render_widget(Clear, centered_area);
    f.render_widget(help, centered_area);
}

pub fn render_sql_editor(f: &mut Frame, content: &str, area: Rect) {
    let instructions = if content.is_empty() {
        "Enter SQL CREATE TABLE statements here.\nPress Ctrl+Enter to parse and apply.\nPress Esc to return to diagram view."
    } else {
        ""
    };
    
    let display_content = if content.is_empty() {
        instructions
    } else {
        content
    };

    let sql_text = Paragraph::new(display_content)
        .block(
            Block::default()
                .title("SQL Editor")
                .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(if content.is_empty() { 
            Style::default().fg(Color::DarkGray) 
        } else { 
            Style::default().fg(Color::White) 
        });

    f.render_widget(sql_text, area);
}

pub fn render_entity_creator(f: &mut Frame, buffer: &str, area: Rect) {
    let instructions = "Enter entity name and press Enter to create.\nPress Esc to cancel.";
    let display_text = if buffer.is_empty() {
        format!("{}\n\nEntity name: _", instructions)
    } else {
        format!("{}\n\nEntity name: {}_", instructions, buffer)
    };

    let text = Paragraph::new(display_text)
        .block(
            Block::default()
                .title("Create New Entity")
                .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .style(Style::default().fg(Color::White));

    let centered_area = centered_rect(50, 30, area);
    f.render_widget(Clear, centered_area);
    f.render_widget(text, centered_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Dimensions, Position};

    #[test]
    fn test_calculate_entity_area() {
        let renderer = DiagramRenderer::new(800, 600);
        
        let entity = Entity {
            name: "test".to_string(),
            columns: vec![],
            position: Position { x: 400.0, y: 300.0 },
            dimensions: Dimensions { width: 20, height: 10 },
        };

        let canvas_area = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 50,
        };

        let entity_area = renderer.calculate_entity_area(&entity, canvas_area);
        
        assert_eq!(entity_area.x, 50);  // 400/800 * 100
        assert_eq!(entity_area.y, 25);  // 300/600 * 50
        assert_eq!(entity_area.width, 20);
        assert_eq!(entity_area.height, 10);
    }

    #[test]
    fn test_centered_rect() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 50,
        };

        let centered = centered_rect(50, 60, area);
        
        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 30);
        assert_eq!(centered.x, 25);
        assert_eq!(centered.y, 10);
    }
}