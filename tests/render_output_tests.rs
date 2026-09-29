use insta::assert_snapshot;
use nerd_core::{parser::SqlParser, render::DiagramRenderer};
use ratatui::{Terminal, backend::TestBackend, buffer::Buffer, layout::Rect};

fn buffer_contents(buffer: &Buffer) -> String {
    let mut output = String::new();

    for y in buffer.area.y..buffer.area.y + buffer.area.height {
        let mut line = String::new();
        for x in buffer.area.x..buffer.area.x + buffer.area.width {
            line.push_str(buffer[(x, y)].symbol());
        }

        let line = line.trim_end();
        if !line.is_empty() {
            output.push_str(line);
            output.push('\n');
        }
    }

    output
}

#[test]
fn test_render_simple_foreign_key_relationship() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            name VARCHAR(100)
        );
        
        CREATE TABLE posts (
            id INT PRIMARY KEY,
            user_id INT,
            title VARCHAR(255),
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
    "#;

    let parser = SqlParser::new();
    let mut schema = parser.parse_sql(sql).expect("Failed to parse SQL");

    // Layout entities (skip automatic layout, we'll position manually)

    // Manually position entities for consistent output
    if let Some(users) = schema.entities.get_mut("users") {
        users.position.x = 5.0;
        users.position.y = 5.0;
    }
    if let Some(posts) = schema.entities.get_mut("posts") {
        posts.position.x = 35.0;
        posts.position.y = 5.0;
    }

    // Create a test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 30);
            let renderer = DiagramRenderer::new(80.0, 30.0);
            renderer.render(f, &schema, area, &None);
        })
        .unwrap();

    let output = buffer_contents(terminal.backend().buffer());

    assert_snapshot!(output);
}

#[test]
fn test_render_multiple_foreign_keys() {
    let sql = r#"
        CREATE TABLE categories (
            id INT PRIMARY KEY,
            name VARCHAR(100)
        );
        
        CREATE TABLE products (
            id INT PRIMARY KEY,
            name VARCHAR(255),
            category_id INT,
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );
        
        CREATE TABLE order_items (
            id INT PRIMARY KEY,
            product_id INT,
            quantity INT,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
    "#;

    let parser = SqlParser::new();
    let mut schema = parser.parse_sql(sql).expect("Failed to parse SQL");

    // Layout entities (skip automatic layout, we'll position manually)

    // Position entities in a triangle formation
    if let Some(categories) = schema.entities.get_mut("categories") {
        categories.position.x = 40.0;
        categories.position.y = 5.0;
    }
    if let Some(products) = schema.entities.get_mut("products") {
        products.position.x = 10.0;
        products.position.y = 15.0;
    }
    if let Some(order_items) = schema.entities.get_mut("order_items") {
        order_items.position.x = 60.0;
        order_items.position.y = 15.0;
    }

    // Create a test terminal
    let backend = TestBackend::new(100, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 40);
            let renderer = DiagramRenderer::new(100.0, 40.0);
            renderer.render(f, &schema, area, &None);
        })
        .unwrap();

    let output = buffer_contents(terminal.backend().buffer());

    assert_snapshot!(output);
}

#[test]
fn test_render_complex_schema_with_relationships() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            name VARCHAR(100),
            email VARCHAR(255)
        );
        
        CREATE TABLE categories (
            id INT PRIMARY KEY,
            name VARCHAR(100)
        );
        
        CREATE TABLE products (
            id INT PRIMARY KEY,
            name VARCHAR(255),
            category_id INT,
            price DECIMAL(10,2),
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );
        
        CREATE TABLE orders (
            id INT PRIMARY KEY,
            user_id INT,
            status VARCHAR(50),
            total DECIMAL(10,2),
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE order_items (
            id INT PRIMARY KEY,
            order_id INT,
            product_id INT,
            quantity INT,
            FOREIGN KEY (order_id) REFERENCES orders(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
    "#;

    let parser = SqlParser::new();
    let mut schema = parser.parse_sql(sql).expect("Failed to parse SQL");

    // Position entities with better spacing to test line routing
    if let Some(users) = schema.entities.get_mut("users") {
        users.position.x = 5.0;
        users.position.y = 2.0;
    }
    if let Some(categories) = schema.entities.get_mut("categories") {
        categories.position.x = 70.0;
        categories.position.y = 2.0;
    }
    if let Some(products) = schema.entities.get_mut("products") {
        products.position.x = 70.0;
        products.position.y = 15.0;
    }
    if let Some(orders) = schema.entities.get_mut("orders") {
        orders.position.x = 5.0;
        orders.position.y = 15.0;
    }
    if let Some(order_items) = schema.entities.get_mut("order_items") {
        order_items.position.x = 35.0;
        order_items.position.y = 30.0;
    }

    // Create a test terminal
    let backend = TestBackend::new(100, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 100, 40);
            let renderer = DiagramRenderer::new(100.0, 40.0);
            renderer.render(f, &schema, area, &None);
        })
        .unwrap();

    let output = buffer_contents(terminal.backend().buffer());

    assert_snapshot!(output);
}
