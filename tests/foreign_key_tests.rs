use insta::assert_yaml_snapshot;
use nerd_core::parser::SqlParser;

#[test]
fn test_parse_foreign_keys_in_table_definition() {
    let sql = r#"
        CREATE TABLE users (
            id INT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE
        );
        
        CREATE TABLE orders (
            id INT PRIMARY KEY,
            user_id INT NOT NULL,
            total DECIMAL(10,2),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE order_items (
            id INT PRIMARY KEY,
            order_id INT NOT NULL,
            product_id INT NOT NULL,
            quantity INT DEFAULT 1,
            price DECIMAL(10,2),
            FOREIGN KEY (order_id) REFERENCES orders(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
        
        CREATE TABLE products (
            id INT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            category_id INT,
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );
        
        CREATE TABLE categories (
            id INT PRIMARY KEY,
            name VARCHAR(100) NOT NULL
        );
    "#;

    let parser = SqlParser::new();
    let schema = parser.parse_sql(sql).expect("Failed to parse SQL");

    // Sort relationships for a stable snapshot of the parsed foreign keys.
    let mut relationships = schema.relationships.clone();
    relationships.sort_by(|a, b| {
        a.from_table
            .cmp(&b.from_table)
            .then(a.from_column.cmp(&b.from_column))
    });

    assert_yaml_snapshot!(relationships, @r###"
    - from_table: order_items
      from_column: order_id
      to_table: orders
      to_column: id
      relationship_type: OneToMany
    - from_table: order_items
      from_column: product_id
      to_table: products
      to_column: id
      relationship_type: OneToMany
    - from_table: orders
      from_column: user_id
      to_table: users
      to_column: id
      relationship_type: OneToMany
    - from_table: products
      from_column: category_id
      to_table: categories
      to_column: id
      relationship_type: OneToMany
    "###);
}

#[test]
fn test_parse_inline_foreign_keys() {
    let sql = r#"
        CREATE TABLE posts (
            id INT PRIMARY KEY,
            author_id INT REFERENCES users(id),
            category_id INT REFERENCES categories(id) ON DELETE CASCADE,
            content TEXT
        );
    "#;

    let parser = SqlParser::new();
    let schema = parser.parse_sql(sql).expect("Failed to parse SQL");

    let posts = schema.entities.get("posts").expect("posts table not found");

    // Check that inline foreign keys are parsed
    let author_id_col = posts
        .columns
        .iter()
        .find(|c| c.name == "author_id")
        .expect("author_id column not found");

    assert!(author_id_col.is_foreign_key);
    assert_eq!(
        author_id_col.references,
        Some(nerd_core::models::ForeignKeyReference {
            table: "users".to_string(),
            column: "id".to_string(),
        })
    );

    // Check relationships
    let relationships: Vec<_> = schema
        .relationships
        .iter()
        .filter(|r| r.from_table == "posts")
        .collect();

    assert_eq!(relationships.len(), 2);

    assert_yaml_snapshot!(relationships, @r###"
    - from_table: posts
      from_column: author_id
      to_table: users
      to_column: id
      relationship_type: OneToMany
    - from_table: posts
      from_column: category_id
      to_table: categories
      to_column: id
      relationship_type: OneToMany
    "###);
}

#[test]
fn test_parse_composite_foreign_keys() {
    let sql = r#"
        CREATE TABLE composite_parent (
            id1 INT,
            id2 INT,
            name VARCHAR(100),
            PRIMARY KEY (id1, id2)
        );
        
        CREATE TABLE composite_child (
            id INT PRIMARY KEY,
            parent_id1 INT,
            parent_id2 INT,
            value VARCHAR(50),
            FOREIGN KEY (parent_id1, parent_id2) REFERENCES composite_parent(id1, id2)
        );
    "#;

    let parser = SqlParser::new();
    let schema = parser.parse_sql(sql).expect("Failed to parse SQL");

    // Composite foreign keys should create multiple relationships
    let relationships: Vec<_> = schema
        .relationships
        .iter()
        .filter(|r| r.from_table == "composite_child")
        .collect();

    // We expect one relationship per column in the composite key
    assert_eq!(relationships.len(), 2);

    assert_yaml_snapshot!(relationships, @r"
    - from_table: composite_child
      from_column: parent_id1
      to_table: composite_parent
      to_column: id1
      relationship_type: OneToMany
    - from_table: composite_child
      from_column: parent_id2
      to_table: composite_parent
      to_column: id2
      relationship_type: OneToMany
    ");
}
