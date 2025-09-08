use crate::models::{Column, Entity, Schema, Position, Dimensions};
use anyhow::Result;
use sqlparser::ast::{ColumnDef, ColumnOption, DataType, Statement};
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
        
        for statement in statements {
            match statement {
                Statement::CreateTable(create_table) => {
                    let table_name = create_table.name.to_string();
                    let entity = self.parse_table(&table_name, &create_table.columns)?;
                    entities.insert(table_name, entity);
                }
                _ => continue,
            }
        }
        
        Ok(Schema {
            entities,
            relationships: Vec::new(),
        })
    }

    fn parse_table(&self, table_name: &str, columns: &[ColumnDef]) -> Result<Entity> {
        let mut parsed_columns = Vec::new();
        
        for column_def in columns {
            let column_name = column_def.name.to_string();
            let data_type = self.format_data_type(&column_def.data_type);
            let mut nullable = true;
            let mut is_primary_key = false;
            
            for option_def in &column_def.options {
                match &option_def.option {
                    ColumnOption::NotNull => nullable = false,
                    ColumnOption::Unique { is_primary, .. } if *is_primary => {
                        is_primary_key = true;
                        nullable = false;
                    }
                    _ => {}
                }
            }
            
            parsed_columns.push(Column {
                name: column_name,
                data_type,
                nullable,
                is_primary_key,
                is_foreign_key: false,
                references: None,
            });
        }
        
        Ok(Entity {
            name: table_name.to_string(),
            columns: parsed_columns,
            position: Position::default(),
            dimensions: Dimensions { width: 20, height: 10 },
        })
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
    fn test_parse_multiple_tables() {
        let sql = "
            CREATE TABLE departments (
                id INT PRIMARY KEY,
                name VARCHAR(100) NOT NULL
            );
            
            CREATE TABLE employees (
                id INT PRIMARY KEY,
                name VARCHAR(100) NOT NULL,
                dept_id INT
            );
        ";
        
        let parser = SqlParser::new();
        let schema = parser.parse_sql(sql).unwrap();
        
        assert_eq!(schema.entities.len(), 2);
        assert!(schema.entities.contains_key("departments"));
        assert!(schema.entities.contains_key("employees"));
        
        let departments = &schema.entities["departments"];
        assert_eq!(departments.columns.len(), 2);
        
        let employees = &schema.entities["employees"];
        assert_eq!(employees.columns.len(), 3);
    }
}