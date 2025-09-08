use crate::models::{Schema, Entity, Column, Relationship};
use anyhow::Result;

pub struct SchemaSync {
}

impl SchemaSync {
    pub fn new() -> Self {
        Self {}
    }

    /// Generate SQL DDL from the current schema
    pub fn generate_sql(&self, schema: &Schema) -> String {
        let mut sql_statements = Vec::new();

        // Generate CREATE TABLE statements
        for (_table_name, entity) in &schema.entities {
            let table_sql = self.generate_create_table(entity);
            sql_statements.push(table_sql);
        }

        // Generate ALTER TABLE statements for foreign keys
        for relationship in &schema.relationships {
            let fk_sql = self.generate_foreign_key_constraint(relationship);
            sql_statements.push(fk_sql);
        }

        sql_statements.join("\n\n")
    }

    fn generate_create_table(&self, entity: &Entity) -> String {
        let mut lines = Vec::new();
        lines.push(format!("CREATE TABLE {} (", entity.name));

        let column_definitions: Vec<String> = entity.columns.iter().map(|col| {
            let mut parts = Vec::new();
            parts.push(format!("    {}", col.name));
            parts.push(col.data_type.clone());
            
            if !col.nullable {
                parts.push("NOT NULL".to_string());
            }
            
            if col.is_primary_key {
                parts.push("PRIMARY KEY".to_string());
            }

            parts.join(" ")
        }).collect();

        lines.push(column_definitions.join(",\n"));
        lines.push(");".to_string());

        lines.join("\n")
    }

    fn generate_foreign_key_constraint(&self, relationship: &Relationship) -> String {
        let constraint_name = format!("fk_{}_{}", relationship.from_table, relationship.from_column);
        format!(
            "ALTER TABLE {} ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({});",
            relationship.from_table,
            constraint_name,
            relationship.from_column,
            relationship.to_table,
            relationship.to_column
        )
    }

    /// Merge changes from SQL back into the schema
    pub fn merge_sql_changes(&self, current_schema: &mut Schema, sql_content: &str) -> Result<bool> {
        let parser = crate::parser::SqlParser::new();
        match parser.parse_sql(sql_content) {
            Ok(new_schema) => {
                let has_changes = self.detect_changes(current_schema, &new_schema);
                if has_changes {
                    self.apply_changes(current_schema, new_schema);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn detect_changes(&self, current: &Schema, new: &Schema) -> bool {
        // Check if entities have changed
        if current.entities.len() != new.entities.len() {
            return true;
        }

        for (name, new_entity) in &new.entities {
            match current.entities.get(name) {
                None => return true, // New entity
                Some(current_entity) => {
                    if self.entity_changed(current_entity, new_entity) {
                        return true;
                    }
                }
            }
        }

        // Check if relationships have changed
        if current.relationships.len() != new.relationships.len() {
            return true;
        }

        for new_rel in &new.relationships {
            if !current.relationships.iter().any(|current_rel| {
                self.relationships_equal(current_rel, new_rel)
            }) {
                return true;
            }
        }

        false
    }

    fn entity_changed(&self, current: &Entity, new: &Entity) -> bool {
        if current.name != new.name || current.columns.len() != new.columns.len() {
            return true;
        }

        for (i, new_col) in new.columns.iter().enumerate() {
            if let Some(current_col) = current.columns.get(i) {
                if self.column_changed(current_col, new_col) {
                    return true;
                }
            } else {
                return true;
            }
        }

        false
    }

    fn column_changed(&self, current: &Column, new: &Column) -> bool {
        current.name != new.name ||
        current.data_type != new.data_type ||
        current.nullable != new.nullable ||
        current.is_primary_key != new.is_primary_key ||
        current.is_foreign_key != new.is_foreign_key ||
        current.references != new.references
    }

    fn relationships_equal(&self, a: &Relationship, b: &Relationship) -> bool {
        a.from_table == b.from_table &&
        a.from_column == b.from_column &&
        a.to_table == b.to_table &&
        a.to_column == b.to_column &&
        a.relationship_type == b.relationship_type
    }

    fn apply_changes(&self, current_schema: &mut Schema, new_schema: Schema) {
        // Preserve positions of existing entities
        let mut preserved_entities = std::collections::HashMap::new();
        
        for (name, new_entity) in new_schema.entities {
            let mut updated_entity = new_entity;
            
            // Preserve position and dimensions if entity existed before
            if let Some(current_entity) = current_schema.entities.get(&name) {
                updated_entity.position = current_entity.position;
                updated_entity.dimensions = current_entity.dimensions;
            }
            
            preserved_entities.insert(name, updated_entity);
        }

        current_schema.entities = preserved_entities;
        current_schema.relationships = new_schema.relationships;
    }

    /// Validate schema consistency
    pub fn validate_schema(&self, schema: &Schema) -> Vec<String> {
        let mut errors = Vec::new();

        // Check for orphaned foreign keys
        for relationship in &schema.relationships {
            if !schema.entities.contains_key(&relationship.from_table) {
                errors.push(format!(
                    "Foreign key references non-existent table: {}",
                    relationship.from_table
                ));
            }
            
            if !schema.entities.contains_key(&relationship.to_table) {
                errors.push(format!(
                    "Foreign key references non-existent table: {}",
                    relationship.to_table
                ));
            }

            // Check if referenced columns exist
            if let Some(from_entity) = schema.entities.get(&relationship.from_table) {
                if !from_entity.columns.iter().any(|col| col.name == relationship.from_column) {
                    errors.push(format!(
                        "Foreign key column '{}' not found in table '{}'",
                        relationship.from_column,
                        relationship.from_table
                    ));
                }
            }

            if let Some(to_entity) = schema.entities.get(&relationship.to_table) {
                if !to_entity.columns.iter().any(|col| col.name == relationship.to_column) {
                    errors.push(format!(
                        "Referenced column '{}' not found in table '{}'",
                        relationship.to_column,
                        relationship.to_table
                    ));
                }
            }
        }

        // Check for entities without primary keys
        for (name, entity) in &schema.entities {
            if !entity.columns.iter().any(|col| col.is_primary_key) {
                errors.push(format!("Table '{}' has no primary key", name));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Position, Dimensions};

    #[test]
    fn test_generate_simple_table_sql() {
        let sync = SchemaSync::new();
        
        let entity = Entity {
            name: "users".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: "INT".to_string(),
                    nullable: false,
                    is_primary_key: true,
                    is_foreign_key: false,
                    references: None,
                },
                Column {
                    name: "name".to_string(),
                    data_type: "VARCHAR(100)".to_string(),
                    nullable: true,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
            ],
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 5 },
        };

        let sql = sync.generate_create_table(&entity);
        assert!(sql.contains("CREATE TABLE users"));
        assert!(sql.contains("id INT NOT NULL PRIMARY KEY"));
        assert!(sql.contains("name VARCHAR(100)"));
    }

    #[test]
    fn test_detect_schema_changes() {
        let sync = SchemaSync::new();
        let schema1 = Schema::new();
        let mut schema2 = Schema::new();

        // Identical schemas should show no changes
        assert!(!sync.detect_changes(&schema1, &schema2));

        // Add an entity to schema2
        let entity = Entity {
            name: "test".to_string(),
            columns: vec![],
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 5 },
        };
        schema2.entities.insert("test".to_string(), entity);

        assert!(sync.detect_changes(&schema1, &schema2));
    }

    #[test]
    fn test_validate_schema() {
        let sync = SchemaSync::new();
        let mut schema = Schema::new();

        // Add entity without primary key
        let entity = Entity {
            name: "invalid".to_string(),
            columns: vec![
                Column {
                    name: "name".to_string(),
                    data_type: "VARCHAR(50)".to_string(),
                    nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                    references: None,
                },
            ],
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 5 },
        };
        schema.entities.insert("invalid".to_string(), entity);

        let errors = sync.validate_schema(&schema);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("has no primary key"));
    }
}