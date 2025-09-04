use gluesql_core::data::Schema;
use nERD::engine::into_entities;

#[cfg(test)]
mod complex_relationships_e2e {
    use super::*;

    #[test]
    fn test_complex_ecommerce_schema_with_crossing_lines() {
        // Design a complex e-commerce schema with 10 tables and multiple FKs
        // Using only GlueSQL supported types: INT, TEXT, BOOLEAN, TIMESTAMP
        let complex_sql = r#"
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
            shipping_address_id INT,
            created_at TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (shipping_address_id) REFERENCES addresses(id)
        );
        
        CREATE TABLE order_items (
            id INT PRIMARY KEY,
            order_id INT,
            product_id INT,
            quantity INT,
            price_at_time INT,
            FOREIGN KEY (order_id) REFERENCES orders(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
        
        CREATE TABLE addresses (
            id INT PRIMARY KEY,
            user_id INT,
            street TEXT,
            city TEXT,
            country TEXT,
            is_default BOOLEAN,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE reviews (
            id INT PRIMARY KEY,
            product_id INT,
            user_id INT,
            rating INT,
            comment TEXT,
            created_at TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE carts (
            id INT PRIMARY KEY,
            user_id INT,
            created_at TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE cart_items (
            id INT PRIMARY KEY,
            cart_id INT,
            product_id INT,
            quantity INT,
            added_at TIMESTAMP,
            FOREIGN KEY (cart_id) REFERENCES carts(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
        
        CREATE TABLE suppliers (
            id INT PRIMARY KEY,
            name TEXT,
            contact_email TEXT,
            contact_phone TEXT,
            address_id INT,
            FOREIGN KEY (address_id) REFERENCES addresses(id)
        );
        "#;

        // Parse the SQL into schemas
        let schemas: Vec<Schema> = complex_sql
            .split(";")
            .map(|sql| sql.trim())
            .filter(|sql| !sql.is_empty())
            .filter_map(|sql| {
                println!("Parsing SQL: {}", sql);
                match Schema::from_ddl(sql) {
                    Ok(schema) => {
                        println!("Successfully parsed: {}", schema.table_name);
                        Some(schema)
                    }
                    Err(e) => {
                        println!("Failed to parse SQL: {}", e);
                        None
                    }
                }
            })
            .collect();

        println!("Parsed {} schemas", schemas.len());

        // Verify we have the expected number of tables
        assert_eq!(schemas.len(), 10, "Should have 10 tables");

        // Convert to entities with positions
        let entities = into_entities(schemas);
        
        // Verify all tables are positioned
        assert_eq!(entities.len(), 10, "Should have 10 entities");

        // Check that tables are distributed across multiple rows
        let y_positions: Vec<usize> = entities.iter().map(|e| e.y).collect();
        let min_y = *y_positions.iter().min().unwrap();
        let max_y = *y_positions.iter().max().unwrap();
        
        assert!(max_y > min_y, "Tables should be on different rows");
        
        // Verify foreign key relationships exist
        let total_fks: usize = entities
            .iter()
            .map(|entity| {
                entity.attributes.iter().filter(|attr| {
                    attr.reffering_to.is_some() || attr.reffered_by.is_some()
                }).count()
            })
            .sum();
            
        assert!(total_fks > 10, "Should have many foreign key relationships");
        
        println!("E2E Test Results:");
        println!("================");
        println!("Tables created: {}", entities.len());
        println!("Foreign key relationships: {}", total_fks);
        println!("Y position range: {} to {}", min_y, max_y);
        
        // Print entity layout for visual verification
        for entity in &entities {
            let fk_count = entity.attributes.iter().filter(|attr| {
                attr.reffering_to.is_some() || attr.reffered_by.is_some()
            }).count();
            
            println!("  {} at ({}, {}) - {} FKs", 
                     entity.name, entity.x, entity.y, fk_count);
        }
    }

    #[test]
    fn test_many_to_many_relationships_with_junction_tables() {
        // Create a scenario with many junction tables that will cause line crossing
        let junction_sql = r#"
        CREATE TABLE students (
            id INT PRIMARY KEY,
            name TEXT,
            email TEXT
        );
        
        CREATE TABLE courses (
            id INT PRIMARY KEY,
            name TEXT,
            credits INT,
            department_id INT,
            FOREIGN KEY (department_id) REFERENCES departments(id)
        );
        
        CREATE TABLE departments (
            id INT PRIMARY KEY,
            name TEXT,
            building TEXT
        );
        
        CREATE TABLE professors (
            id INT PRIMARY KEY,
            name TEXT,
            department_id INT,
            office TEXT,
            FOREIGN KEY (department_id) REFERENCES departments(id)
        );
        
        CREATE TABLE enrollments (
            student_id INT,
            course_id INT,
            grade TEXT,
            enrolled_at TIMESTAMP,
            FOREIGN KEY (student_id) REFERENCES students(id),
            FOREIGN KEY (course_id) REFERENCES courses(id)
        );
        
        CREATE TABLE course_professors (
            course_id INT,
            professor_id INT,
            role TEXT,
            FOREIGN KEY (course_id) REFERENCES courses(id),
            FOREIGN KEY (professor_id) REFERENCES professors(id)
        );
        
        CREATE TABLE student_advisors (
            student_id INT,
            professor_id INT,
            start_year INT,
            end_year INT,
            FOREIGN KEY (student_id) REFERENCES students(id),
            FOREIGN KEY (professor_id) REFERENCES professors(id)
        );
        
        CREATE TABLE prerequisites (
            course_id INT,
            prerequisite_course_id INT,
            FOREIGN KEY (course_id) REFERENCES courses(id),
            FOREIGN KEY (prerequisite_course_id) REFERENCES courses(id)
        );
        
        CREATE TABLE textbooks (
            id INT PRIMARY KEY,
            title TEXT,
            isbn TEXT,
            publisher TEXT
        );
        
        CREATE TABLE course_textbooks (
            course_id INT,
            textbook_id INT,
            required BOOLEAN,
            FOREIGN KEY (course_id) REFERENCES courses(id),
            FOREIGN KEY (textbook_id) REFERENCES textbooks(id)
        );
        "#;

        let schemas: Vec<Schema> = junction_sql
            .split(";")
            .map(|sql| sql.trim())
            .filter(|sql| !sql.is_empty())
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect();

        assert_eq!(schemas.len(), 10, "Should have 10 tables including junction tables");

        let entities = into_entities(schemas);
        
        // Analyze the complexity of relationships
        let junction_tables: Vec<&str> = entities
            .iter()
            .filter(|entity| {
                // Junction tables typically have multiple FKs and few other columns
                let fk_count = entity.attributes.iter().filter(|attr| {
                    attr.reffering_to.is_some()
                }).count();
                fk_count >= 2 && entity.attributes.len() <= 5
            })
            .map(|entity| entity.name.as_str())
            .collect();

        assert!(!junction_tables.is_empty(), "Should have junction tables");
        
        println!("Junction Tables Analysis:");
        println!("=========================");
        for table_name in &junction_tables {
            let entity = entities.iter().find(|e| e.name == *table_name).unwrap();
            let relationships: Vec<String> = entity.attributes.iter()
                .filter_map(|attr| attr.reffering_to.as_ref())
                .map(|(table, column)| format!("{}({})", table, column))
                .collect();
                
            println!("  {}: connects to {}", table_name, relationships.join(", "));
        }
        
        // Verify that relationships create potential for line crossings
        // This happens when junction tables connect entities that are far apart
        let mut crossing_potential = 0;
        for entity in &entities {
            for attr in &entity.attributes {
                if let Some((ref_table, _)) = &attr.reffering_to {
                    if let Some(target_entity) = entities.iter().find(|e| &e.name == ref_table) {
                        let distance = ((entity.x as i32 - target_entity.x as i32).abs() + 
                                       (entity.y as i32 - target_entity.y as i32).abs()) as usize;
                        if distance > 30 {
                            crossing_potential += 1;
                        }
                    }
                }
            }
        }
        
        assert!(crossing_potential > 0, "Should have relationships that could cross");
        println!("Potential crossing relationships: {}", crossing_potential);
    }

    #[test]
    fn test_circular_relationships_complex_diagram() {
        // Create a schema with circular dependencies and self-references  
        let circular_sql = r#"
        CREATE TABLE companies (
            id INT PRIMARY KEY,
            name TEXT,
            parent_company_id INT,
            ceo_employee_id INT,
            FOREIGN KEY (parent_company_id) REFERENCES companies(id),
            FOREIGN KEY (ceo_employee_id) REFERENCES employees(id)
        );
        
        CREATE TABLE employees (
            id INT PRIMARY KEY,
            name TEXT,
            company_id INT,
            manager_id INT,
            department_id INT,
            position_id INT,
            FOREIGN KEY (company_id) REFERENCES companies(id),
            FOREIGN KEY (manager_id) REFERENCES employees(id),
            FOREIGN KEY (department_id) REFERENCES departments(id),
            FOREIGN KEY (position_id) REFERENCES positions(id)
        );
        
        CREATE TABLE departments (
            id INT PRIMARY KEY,
            name TEXT,
            company_id INT,
            head_employee_id INT,
            parent_dept_id INT,
            FOREIGN KEY (company_id) REFERENCES companies(id),
            FOREIGN KEY (head_employee_id) REFERENCES employees(id),
            FOREIGN KEY (parent_dept_id) REFERENCES departments(id)
        );
        
        CREATE TABLE positions (
            id INT PRIMARY KEY,
            title TEXT,
            department_id INT,
            reports_to_position_id INT,
            FOREIGN KEY (department_id) REFERENCES departments(id),
            FOREIGN KEY (reports_to_position_id) REFERENCES positions(id)
        );
        
        CREATE TABLE projects (
            id INT PRIMARY KEY,
            name TEXT,
            company_id INT,
            lead_employee_id INT,
            department_id INT,
            FOREIGN KEY (company_id) REFERENCES companies(id),
            FOREIGN KEY (lead_employee_id) REFERENCES employees(id),
            FOREIGN KEY (department_id) REFERENCES departments(id)
        );
        
        CREATE TABLE project_assignments (
            project_id INT,
            employee_id INT,
            role TEXT,
            start_year INT,
            end_year INT,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (employee_id) REFERENCES employees(id)
        );
        
        CREATE TABLE skills (
            id INT PRIMARY KEY,
            name TEXT,
            category TEXT
        );
        
        CREATE TABLE employee_skills (
            employee_id INT,
            skill_id INT,
            proficiency_level INT,
            certified BOOLEAN,
            FOREIGN KEY (employee_id) REFERENCES employees(id),
            FOREIGN KEY (skill_id) REFERENCES skills(id)
        );
        
        CREATE TABLE project_requirements (
            project_id INT,
            skill_id INT,
            required_level INT,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (skill_id) REFERENCES skills(id)
        );
        
        CREATE TABLE performance_reviews (
            id INT PRIMARY KEY,
            employee_id INT,
            reviewer_id INT,
            project_id INT,
            score INT,
            review_year INT,
            FOREIGN KEY (employee_id) REFERENCES employees(id),
            FOREIGN KEY (reviewer_id) REFERENCES employees(id),
            FOREIGN KEY (project_id) REFERENCES projects(id)
        );
        "#;

        let schemas: Vec<Schema> = circular_sql
            .split(";")
            .map(|sql| sql.trim())
            .filter(|sql| !sql.is_empty())
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect();

        assert_eq!(schemas.len(), 10, "Should have 10 tables");

        let entities = into_entities(schemas);

        // Analyze the circular relationships
        let self_referencing_tables: Vec<&str> = entities
            .iter()
            .filter(|entity| {
                entity.attributes.iter().any(|attr| {
                    attr.reffering_to.as_ref()
                        .map(|(table, _)| table == &entity.name)
                        .unwrap_or(false)
                })
            })
            .map(|e| e.name.as_str())
            .collect();

        assert!(!self_referencing_tables.is_empty(), "Should have self-referencing tables");

        // Count total relationships that will create crossing lines
        let mut relationship_matrix = vec![vec![false; entities.len()]; entities.len()];
        let mut total_relationships = 0;

        for (i, entity) in entities.iter().enumerate() {
            for attr in &entity.attributes {
                if let Some((ref_table, _)) = &attr.reffering_to {
                    if let Some(j) = entities.iter().position(|e| &e.name == ref_table) {
                        relationship_matrix[i][j] = true;
                        total_relationships += 1;
                    }
                }
            }
        }

        println!("Circular Relationships Analysis:");
        println!("================================");
        println!("Total tables: {}", entities.len());
        println!("Total FK relationships: {}", total_relationships);
        println!("Self-referencing tables: {:?}", self_referencing_tables);
        
        // Detect potential crossing lines
        let mut crossing_count = 0;
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if relationship_matrix[i][j] {
                    for k in 0..entities.len() {
                        for l in 0..entities.len() {
                            if relationship_matrix[k][l] && i != k && j != l {
                                // Check if lines from (i,j) and (k,l) could cross
                                let entity1 = &entities[i];
                                let entity2 = &entities[j];
                                let entity3 = &entities[k];
                                let entity4 = &entities[l];
                                
                                if lines_could_cross(
                                    (entity1.x, entity1.y),
                                    (entity2.x, entity2.y),
                                    (entity3.x, entity3.y),
                                    (entity4.x, entity4.y),
                                ) {
                                    crossing_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Potential line crossings: {}", crossing_count);
        assert!(crossing_count > 0, "Should have potential line crossings");
        
        // Print detailed entity positions and their relationships
        println!("\nDetailed Entity Analysis:");
        println!("=========================");
        for entity in &entities {
            let outgoing_fks: Vec<String> = entity.attributes.iter()
                .filter_map(|attr| attr.reffering_to.as_ref())
                .map(|(table, col)| format!("{}({})", table, col))
                .collect();
                
            let incoming_fks: Vec<String> = entity.attributes.iter()
                .filter_map(|attr| attr.reffered_by.as_ref())
                .map(|(table, col)| format!("{}({})", table, col))
                .collect();
            
            println!(
                "{:15} at ({:2},{:2}) -> Out: [{}] In: [{}]",
                entity.name,
                entity.x,
                entity.y,
                outgoing_fks.join(", "),
                incoming_fks.join(", ")
            );
        }
    }
    
    #[test]
    fn test_render_complex_relationships() {
        // Test that the engine can handle complex relationships
        // Note: render function is not public, so we test the entity creation instead
        
        // Create a smaller but complex schema for rendering test
        let render_sql = r#"
        CREATE TABLE authors (
            id INT PRIMARY KEY,
            name TEXT
        );
        
        CREATE TABLE publishers (
            id INT PRIMARY KEY,
            name TEXT,
            country TEXT
        );
        
        CREATE TABLE books (
            id INT PRIMARY KEY,
            title TEXT,
            author_id INT,
            publisher_id INT,
            isbn TEXT,
            FOREIGN KEY (author_id) REFERENCES authors(id),
            FOREIGN KEY (publisher_id) REFERENCES publishers(id)
        );
        
        CREATE TABLE readers (
            id INT PRIMARY KEY,
            name TEXT,
            email TEXT
        );
        
        CREATE TABLE book_reviews (
            id INT PRIMARY KEY,
            book_id INT,
            reader_id INT,
            rating INT,
            review_text TEXT,
            FOREIGN KEY (book_id) REFERENCES books(id),
            FOREIGN KEY (reader_id) REFERENCES readers(id)
        );
        "#;

        let schemas: Vec<Schema> = render_sql
            .split(";")
            .map(|sql| sql.trim())
            .filter(|sql| !sql.is_empty())
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect();

        let entities = into_entities(schemas);
        assert_eq!(entities.len(), 5);

        // Test entity creation and relationships
        println!("Render Test Results:");
        println!("===================");
        println!("Created {} entities", entities.len());
        
        // Verify all entities have proper positioning
        for entity in &entities {
            assert!(entity.x > 0, "Entity {} should have valid x position", entity.name);
            assert!(entity.y > 0, "Entity {} should have valid y position", entity.name);
            assert!(!entity.attributes.is_empty(), "Entity {} should have attributes", entity.name);
        }

        // Count relationships
        let mut total_relationships = 0;
        for entity in &entities {
            for attr in &entity.attributes {
                if attr.reffering_to.is_some() {
                    total_relationships += 1;
                }
            }
            
            println!(
                "{:15} at ({:2},{:2}) with {} attributes", 
                entity.name, 
                entity.x, 
                entity.y,
                entity.attributes.len()
            );
        }
        
        println!("Total FK relationships: {}", total_relationships);
        assert!(total_relationships > 0, "Should have foreign key relationships");
    }
}

// Helper function to determine if two lines could potentially cross
fn lines_could_cross(
    line1_start: (usize, usize),
    line1_end: (usize, usize),
    line2_start: (usize, usize),
    line2_end: (usize, usize),
) -> bool {
    // Simple geometric check for potential line crossing
    // This is a simplified version - real line intersection is more complex
    
    let (x1a, y1a) = line1_start;
    let (x1b, y1b) = line1_end;
    let (x2a, y2a) = line2_start;
    let (x2b, y2b) = line2_end;
    
    // Check if the bounding rectangles of the two lines overlap
    let line1_min_x = x1a.min(x1b);
    let line1_max_x = x1a.max(x1b);
    let line1_min_y = y1a.min(y1b);
    let line1_max_y = y1a.max(y1b);
    
    let line2_min_x = x2a.min(x2b);
    let line2_max_x = x2a.max(x2b);
    let line2_min_y = y2a.min(y2b);
    let line2_max_y = y2a.max(y2b);
    
    // Lines could cross if their bounding rectangles overlap
    !(line1_max_x < line2_min_x || 
      line2_max_x < line1_min_x || 
      line1_max_y < line2_min_y || 
      line2_max_y < line1_min_y)
}