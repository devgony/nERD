use gluesql_core::data::Schema;
use nERD::engine::into_entities;

fn main() {
    // Example using the actual into_entities function to generate positions
    println!("Drawing entities from SQL schemas");
    println!("==================================\n");
    
    // Create schemas from SQL DDL statements
    let sql = r#"
CREATE TABLE Users (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(100),
    created_at TIMESTAMP
);
CREATE TABLE Posts (
    id INT PRIMARY KEY,
    user_id INT,
    title VARCHAR(200),
    content TEXT,
    published BOOLEAN,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);
CREATE TABLE Comments (
    id INT PRIMARY KEY,
    post_id INT,
    user_id INT,
    comment TEXT,
    created_at TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES Posts(id),
    FOREIGN KEY (user_id) REFERENCES Users(id)
);
CREATE TABLE Tags (
    id INT PRIMARY KEY,
    name VARCHAR(50)
);
CREATE TABLE PostTags (
    post_id INT,
    tag_id INT,
    FOREIGN KEY (post_id) REFERENCES Posts(id),
    FOREIGN KEY (tag_id) REFERENCES Tags(id)
);"#;

    // Parse SQL into schemas
    let schemas: Vec<Schema> = sql
        .split(";")
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Schema::from_ddl(trimmed).ok()
            } else {
                None
            }
        })
        .collect();
    
    // Convert schemas to entities with calculated positions
    let entities = into_entities(schemas);
    
    // Draw a simple representation
    println!("Entity positions calculated by into_entities:");
    println!("----------------------------------------------");
    for entity in &entities {
        println!(
            "Table: {:15} Position: ({:3}, {:3}) Columns: {}",
            entity.name,
            entity.x,
            entity.y,
            entity.attributes.len()
        );
    }
    
    // Draw ASCII representation on a grid
    println!("\nASCII Grid Visualization:");
    println!("-------------------------\n");
    
    let mut grid = vec![vec![' '; 80]; 30];
    
    for entity in &entities {
        draw_entity_on_grid(&mut grid, &entity);
    }
    
    // Print the grid
    for row in &grid {
        let line: String = row.iter().collect();
        if line.trim().len() > 0 {
            println!("{}", line);
        }
    }
}

fn draw_entity_on_grid(grid: &mut Vec<Vec<char>>, entity: &nERD::engine::Entity) {
    let x = entity.x;
    let y = entity.y;
    let width = 12; // Fixed width for entities
    let height = entity.attributes.len() + 3; // Header + separator + attributes + bottom
    
    // Skip if out of bounds
    if y >= grid.len() || x >= grid[0].len() {
        return;
    }
    
    // Draw top border
    if y < grid.len() {
        draw_horizontal_line(grid, x, y, width, '─', '┌', '┐');
    }
    
    // Draw title
    if y + 1 < grid.len() {
        draw_text_in_box(grid, x, y + 1, width, &entity.name);
    }
    
    // Draw separator
    if y + 2 < grid.len() {
        draw_horizontal_line(grid, x, y + 2, width, '─', '├', '┤');
    }
    
    // Draw attributes
    for (i, attr) in entity.attributes.iter().enumerate() {
        if y + 3 + i < grid.len() {
            let attr_name = if attr.name.len() > 10 {
                &attr.name[..10]
            } else {
                &attr.name
            };
            draw_text_in_box(grid, x, y + 3 + i, width, attr_name);
        }
    }
    
    // Draw bottom border
    if y + height - 1 < grid.len() {
        draw_horizontal_line(grid, x, y + height - 1, width, '─', '└', '┘');
    }
}

fn draw_horizontal_line(
    grid: &mut Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    line_char: char,
    left_corner: char,
    right_corner: char,
) {
    if y >= grid.len() {
        return;
    }
    
    if x < grid[0].len() {
        grid[y][x] = left_corner;
    }
    
    for i in 1..width - 1 {
        if x + i < grid[0].len() {
            grid[y][x + i] = line_char;
        }
    }
    
    if x + width - 1 < grid[0].len() {
        grid[y][x + width - 1] = right_corner;
    }
}

fn draw_text_in_box(
    grid: &mut Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    text: &str,
) {
    if y >= grid.len() {
        return;
    }
    
    // Draw left border
    if x < grid[0].len() {
        grid[y][x] = '│';
    }
    
    // Draw text (padded)
    let padded_text = format!("{:<width$}", text, width = width - 2);
    for (i, ch) in padded_text.chars().take(width - 2).enumerate() {
        if x + i + 1 < grid[0].len() {
            grid[y][x + i + 1] = ch;
        }
    }
    
    // Draw right border
    if x + width - 1 < grid[0].len() {
        grid[y][x + width - 1] = '│';
    }
}