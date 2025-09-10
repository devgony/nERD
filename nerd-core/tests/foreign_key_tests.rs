use nerd_core::parser::SqlParser;
use insta::assert_yaml_snapshot;

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
    
    // Sort entities and relationships for consistent snapshots
    let mut entities: Vec<_> = schema.entities.iter().collect();
    entities.sort_by_key(|(name, _)| name.as_str());
    
    let mut relationships = schema.relationships.clone();
    relationships.sort_by(|a, b| {
        a.from_table.cmp(&b.from_table)
            .then(a.from_column.cmp(&b.from_column))
    });
    
    assert_yaml_snapshot!(entities, @r"
    - - categories
      - name: categories
        columns:
          - name: id
            data_type: INT
            nullable: false
            is_primary_key: true
            is_foreign_key: false
            references: ~
          - name: name
            data_type: VARCHAR(100)
            nullable: false
            is_primary_key: false
            is_foreign_key: false
            references: ~
        position:
          x: 0
          y: 0
        dimensions:
          width: 20
          height: 10
    - - order_items
      - name: order_items
        columns:
          - name: id
            data_type: INT
            nullable: false
            is_primary_key: true
            is_foreign_key: false
            references: ~
          - name: order_id
            data_type: INT
            nullable: false
            is_primary_key: false
            is_foreign_key: true
            references:
              table: orders
              column: id
          - name: product_id
            data_type: INT
            nullable: false
            is_primary_key: false
            is_foreign_key: true
            references:
              table: products
              column: id
          - name: quantity
            data_type: INT
            nullable: true
            is_primary_key: false
            is_foreign_key: false
            references: ~
          - name: price
            data_type: DECIMAL
            nullable: true
            is_primary_key: false
            is_foreign_key: false
            references: ~
        position:
          x: 0
          y: 0
        dimensions:
          width: 20
          height: 10
    - - orders
      - name: orders
        columns:
          - name: id
            data_type: INT
            nullable: false
            is_primary_key: true
            is_foreign_key: false
            references: ~
          - name: user_id
            data_type: INT
            nullable: false
            is_primary_key: false
            is_foreign_key: true
            references:
              table: users
              column: id
          - name: total
            data_type: DECIMAL
            nullable: true
            is_primary_key: false
            is_foreign_key: false
            references: ~
          - name: created_at
            data_type: TIMESTAMP
            nullable: true
            is_primary_key: false
            is_foreign_key: false
            references: ~
        position:
          x: 0
          y: 0
        dimensions:
          width: 20
          height: 10
    - - products
      - name: products
        columns:
          - name: id
            data_type: INT
            nullable: false
            is_primary_key: true
            is_foreign_key: false
            references: ~
          - name: name
            data_type: VARCHAR(255)
            nullable: false
            is_primary_key: false
            is_foreign_key: false
            references: ~
          - name: category_id
            data_type: INT
            nullable: true
            is_primary_key: false
            is_foreign_key: true
            references:
              table: categories
              column: id
        position:
          x: 0
          y: 0
        dimensions:
          width: 20
          height: 10
    - - users
      - name: users
        columns:
          - name: id
            data_type: INT
            nullable: false
            is_primary_key: true
            is_foreign_key: false
            references: ~
          - name: name
            data_type: VARCHAR(100)
            nullable: false
            is_primary_key: false
            is_foreign_key: false
            references: ~
          - name: email
            data_type: VARCHAR(255)
            nullable: true
            is_primary_key: false
            is_foreign_key: false
            references: ~
        position:
          x: 0
          y: 0
        dimensions:
          width: 20
          height: 10
    ");
    
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
    let author_id_col = posts.columns.iter()
        .find(|c| c.name == "author_id")
        .expect("author_id column not found");
    
    assert!(author_id_col.is_foreign_key);
    assert_eq!(author_id_col.references, Some(nerd_core::models::ForeignKeyReference {
        table: "users".to_string(),
        column: "id".to_string(),
    }));
    
    // Check relationships
    let relationships: Vec<_> = schema.relationships.iter()
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
    let relationships: Vec<_> = schema.relationships.iter()
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
      to_column: id1
      relationship_type: OneToMany
    ");
}
