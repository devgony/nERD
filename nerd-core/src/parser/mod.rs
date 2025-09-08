use crate::models::{Column, Entity, Schema, Position, Dimensions, Relationship, RelationshipType, ForeignKeyReference};
use anyhow::Result;
use sqlparser::ast::{ColumnDef, ColumnOption, DataType, Statement, TableConstraint};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashMap;

pub struct SqlParser {
    dialect: GenericDialect,
}

impl SqlParser {
    pub fn new() -> Self {
        Self {
            dialect: GenericDialect {},
        }
    }

    pub fn parse_sql(&self, sql: &str) -> Result<Schema> {
        let statements = Parser::parse_sql(&self.dialect, sql)?;
        let mut entities = HashMap::new();
        let mut relationships = Vec::new();
        
        for statement in statements {
            match statement {
                Statement::CreateTable(create_table) => {
                    let table_name = create_table.name.to_string();
                    let (entity, table_relationships) = self.parse_table(&table_name, &create_table.columns, &create_table.constraints)?;
                    entities.insert(table_name, entity);
                    relationships.extend(table_relationships);
                }
                _ => continue,
            }
        }
        
        Ok(Schema {
            entities,
            relationships,
        })
    }

    fn parse_table(&self, table_name: &str, columns: &[ColumnDef], constraints: &Vec<TableConstraint>) -> Result<(Entity, Vec<Relationship>)> {
        let mut parsed_columns = Vec::new();
        let mut relationships = Vec::new();
        
        // Parse columns
        for column_def in columns {
            let column_name = column_def.name.to_string();
            let data_type = self.format_data_type(&column_def.data_type);
            let mut nullable = true;
            let mut is_primary_key = false;
            let mut is_foreign_key = false;
            let mut references = None;
            
            for option_def in &column_def.options {
                match &option_def.option {
                    ColumnOption::NotNull => nullable = false,
                    ColumnOption::Unique { is_primary, .. } if *is_primary => {
                        is_primary_key = true;
                        nullable = false;
                    }
                    ColumnOption::ForeignKey { foreign_table, referred_columns, .. } => {
                        is_foreign_key = true;
                        let ref_table = foreign_table.to_string();
                        let ref_column = referred_columns.first()
                            .map(|c| c.to_string())
                            .unwrap_or_else(|| "id".to_string());
                        
                        references = Some(ForeignKeyReference {
                            table: ref_table.clone(),
                            column: ref_column.clone(),
                        });
                        
                        relationships.push(Relationship {
                            from_table: table_name.to_string(),
                            from_column: column_name.clone(),
                            to_table: ref_table,
                            to_column: ref_column,
                            relationship_type: RelationshipType::OneToMany,
                        });
                    }
                    _ => {}
                }
            }
            
            parsed_columns.push(Column {
                name: column_name,
                data_type,
                nullable,
                is_primary_key,
                is_foreign_key,
                references,
            });
        }
        
        // Parse table-level foreign key constraints
        for constraint in constraints {
            match constraint {
                TableConstraint::ForeignKey { columns: fk_columns, foreign_table, referred_columns, .. } => {
                    for fk_col in fk_columns {
                        let fk_col_name = fk_col.to_string();
                        let ref_table = foreign_table.to_string();
                        let ref_column = referred_columns.first()
                            .map(|c| c.to_string())
                            .unwrap_or_else(|| "id".to_string());
                        
                        // Mark the column as foreign key
                        if let Some(column) = parsed_columns.iter_mut().find(|c| c.name == fk_col_name) {
                            column.is_foreign_key = true;
                            column.references = Some(ForeignKeyReference {
                                table: ref_table.clone(),
                                column: ref_column.clone(),
                            });
                        }
                        
                        relationships.push(Relationship {
                            from_table: table_name.to_string(),
                            from_column: fk_col_name,
                            to_table: ref_table,
                            to_column: ref_column,
                            relationship_type: RelationshipType::OneToMany,
                        });
                    }
                }
                TableConstraint::PrimaryKey { columns: pk_columns, .. } => {
                    for pk_col in pk_columns {
                        let pk_col_name = pk_col.to_string();
                        if let Some(column) = parsed_columns.iter_mut().find(|c| c.name == pk_col_name) {
                            column.is_primary_key = true;
                            column.nullable = false;
                        }
                    }
                }
                _ => {}
            }
        }
        
        let entity = Entity {
            name: table_name.to_string(),
            columns: parsed_columns,
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 10 },
        };
        
        Ok((entity, relationships))
    }

    fn format_data_type(&self, data_type: &DataType) -> String {
        match data_type {
            DataType::Char(size) | DataType::Character(size) => {
                match size {
                    Some(len) => format!("CHAR({})", len),
                    None => "CHAR".to_string(),
                }
            }
            DataType::Varchar(size) | DataType::CharacterVarying(size) => {
                match size {
                    Some(len) => format!("VARCHAR({})", len),
                    None => "VARCHAR".to_string(),
                }
            }
            DataType::Text => "TEXT".to_string(),
            DataType::TinyInt(_) => "TINYINT".to_string(),
            DataType::SmallInt(_) => "SMALLINT".to_string(),
            DataType::Int(_) | DataType::Integer(_) => "INT".to_string(),
            DataType::BigInt(_) => "BIGINT".to_string(),
            DataType::Float(_) => "FLOAT".to_string(),
            DataType::Real => "REAL".to_string(),
            DataType::Double | DataType::DoublePrecision => "DOUBLE".to_string(),
            DataType::Decimal(_) | DataType::Dec(_) | DataType::Numeric(_) => "DECIMAL".to_string(),
            DataType::Boolean => "BOOLEAN".to_string(),
            DataType::Date => "DATE".to_string(),
            DataType::Time(_, _) => "TIME".to_string(),
            DataType::Timestamp(_, _) => "TIMESTAMP".to_string(),
            DataType::Datetime(_) => "DATETIME".to_string(),
            DataType::Uuid => "UUID".to_string(),
            DataType::JSON => "JSON".to_string(),
            DataType::Blob(_) => "BLOB".to_string(),
            _ => format!("{:?}", data_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_table() {
        let sql = "CREATE TABLE users (
            id INT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255)
        );";
        
        let parser = SqlParser::new();
        let schema = parser.parse_sql(sql).unwrap();
        
        assert_eq!(schema.entities.len(), 1);
        assert!(schema.entities.contains_key("users"));
        
        let users = &schema.entities["users"];
        assert_eq!(users.columns.len(), 3);
        
        let id_col = &users.columns[0];
        assert_eq!(id_col.name, "id");
        assert!(id_col.is_primary_key);
        assert!(!id_col.nullable);
        
        let name_col = &users.columns[1];
        assert_eq!(name_col.name, "name");
        assert!(!name_col.nullable);
        assert_eq!(name_col.data_type, "VARCHAR(100)");
    }

    #[test]
    fn test_parse_tables_with_foreign_keys() {
        let sql = "
            CREATE TABLE departments (
                id INT PRIMARY KEY,
                name VARCHAR(100) NOT NULL
            );
            
            CREATE TABLE employees (
                id INT PRIMARY KEY,
                name VARCHAR(100) NOT NULL,
                dept_id INT,
                FOREIGN KEY (dept_id) REFERENCES departments(id)
            );
        ";
        
        let parser = SqlParser::new();
        let schema = parser.parse_sql(sql).unwrap();
        
        assert_eq!(schema.entities.len(), 2);
        assert!(schema.entities.contains_key("departments"));
        assert!(schema.entities.contains_key("employees"));
        
        // Check relationships were created
        assert_eq!(schema.relationships.len(), 1);
        let rel = &schema.relationships[0];
        assert_eq!(rel.from_table, "employees");
        assert_eq!(rel.from_column, "dept_id");
        assert_eq!(rel.to_table, "departments");
        assert_eq!(rel.to_column, "id");
        
        // Check foreign key column is marked correctly
        let employees = &schema.entities["employees"];
        let dept_id_col = employees.columns.iter().find(|c| c.name == "dept_id").unwrap();
        assert!(dept_id_col.is_foreign_key);
        assert!(dept_id_col.references.is_some());
        
        let fk_ref = dept_id_col.references.as_ref().unwrap();
        assert_eq!(fk_ref.table, "departments");
        assert_eq!(fk_ref.column, "id");
    }
}