use gluesql_core::data::Schema;
use nERD::engine::into_entities;
use nERD::render::{render, render_foreign_key};
use insta::assert_snapshot;

#[test]
fn test_simple_table_rendering() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            email TEXT,
            name TEXT
        );
    "#;
    
    let schemas: Vec<Schema> = sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .filter_map(|sql| Schema::from_ddl(sql).ok())
        .collect();
    
    let entities = into_entities(schemas);
    
    let canvas_width = 100;
    let canvas_height = 50;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; canvas_width]; canvas_height];
    
    let (_, rendered_string) = render(canvas, &entities);
    
    assert_snapshot!(rendered_string);
}

#[test]
fn test_multiple_tables_rendering() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            email TEXT,
            name TEXT,
            created_at TIMESTAMP
        );
        
        CREATE TABLE posts (
            id INT PRIMARY KEY,
            title TEXT,
            content TEXT,
            user_id INT,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
    "#;
    
    let schemas: Vec<Schema> = sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .filter_map(|sql| Schema::from_ddl(sql).ok())
        .collect();
    
    let entities = into_entities(schemas);
    
    let canvas_width = 100;
    let canvas_height = 50;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; canvas_width]; canvas_height];
    
    let (_, rendered_string) = render(canvas, &entities);
    
    assert_snapshot!(rendered_string);
}

#[test]
fn test_foreign_key_rendering() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            email TEXT,
            name TEXT
        );
        
        CREATE TABLE posts (
            id INT PRIMARY KEY,
            title TEXT,
            user_id INT,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE comments (
            id INT PRIMARY KEY,
            content TEXT,
            post_id INT,
            user_id INT,
            FOREIGN KEY (post_id) REFERENCES posts(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
    "#;
    
    let schemas: Vec<Schema> = sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .filter_map(|sql| Schema::from_ddl(sql).ok())
        .collect();
    
    let entities = into_entities(schemas);
    
    let canvas_width = 100;
    let canvas_height = 50;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; canvas_width]; canvas_height];
    
    let (canvas_with_entities, _) = render(canvas, &entities);
    let (_, rendered_with_fks) = render_foreign_key(canvas_with_entities, &entities);
    
    assert_snapshot!(rendered_with_fks);
}

#[test]
fn test_self_referencing_table() {
    let sql = r#"
        CREATE TABLE categories (
            id INT PRIMARY KEY,
            name TEXT,
            parent_id INT,
            FOREIGN KEY (parent_id) REFERENCES categories(id)
        );
    "#;
    
    let schemas: Vec<Schema> = sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .filter_map(|sql| Schema::from_ddl(sql).ok())
        .collect();
    
    let entities = into_entities(schemas);
    
    let canvas_width = 100;
    let canvas_height = 50;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; canvas_width]; canvas_height];
    
    let (canvas_with_entities, _) = render(canvas, &entities);
    let (_, rendered_with_fks) = render_foreign_key(canvas_with_entities, &entities);
    
    assert_snapshot!(rendered_with_fks);
}

#[test]
fn test_complex_schema_rendering() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            email TEXT,
            name TEXT,
            created_at TIMESTAMP
        );
        
        CREATE TABLE categories (
            id INT PRIMARY KEY,
            name TEXT,
            parent_id INT,
            FOREIGN KEY (parent_id) REFERENCES categories(id)
        );
        
        CREATE TABLE products (
            id INT PRIMARY KEY,
            name TEXT,
            category_id INT,
            price INT,
            stock_quantity INT,
            created_by INT,
            FOREIGN KEY (category_id) REFERENCES categories(id),
            FOREIGN KEY (created_by) REFERENCES users(id)
        );
        
        CREATE TABLE orders (
            id INT PRIMARY KEY,
            user_id INT,
            status TEXT,
            total_amount INT,
            created_at TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE order_items (
            id INT PRIMARY KEY,
            order_id INT,
            product_id INT,
            quantity INT,
            price INT,
            FOREIGN KEY (order_id) REFERENCES orders(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
    "#;
    
    let schemas: Vec<Schema> = sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .filter_map(|sql| Schema::from_ddl(sql).ok())
        .collect();
    
    let entities = into_entities(schemas);
    
    let canvas_width = 150;
    let canvas_height = 80;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; canvas_width]; canvas_height];
    
    let (canvas_with_entities, _) = render(canvas, &entities);
    let (_, rendered_with_fks) = render_foreign_key(canvas_with_entities, &entities);
    
    assert_snapshot!(rendered_with_fks);
}

#[test]
fn test_table_with_many_columns() {
    let sql = r#"
        CREATE TABLE detailed_users (
            id INT PRIMARY KEY,
            username TEXT,
            email TEXT,
            first_name TEXT,
            last_name TEXT,
            phone TEXT,
            address TEXT,
            city TEXT,
            country TEXT,
            postal_code TEXT,
            date_of_birth TIMESTAMP,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            is_active BOOLEAN,
            last_login TIMESTAMP
        );
    "#;
    
    let schemas: Vec<Schema> = sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .filter_map(|sql| Schema::from_ddl(sql).ok())
        .collect();
    
    let entities = into_entities(schemas);
    
    let canvas_width = 100;
    let canvas_height = 50;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; canvas_width]; canvas_height];
    
    let (_, rendered_string) = render(canvas, &entities);
    
    assert_snapshot!(rendered_string);
}