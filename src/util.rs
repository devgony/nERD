use std::fmt::{Debug, Display};

pub fn assert_text<T: PartialEq + Display + Debug>(actual: T, expected: T) {
    if actual != expected {
        println!("Actual:\n{}", actual);
        println!("Expected:\n{}", expected);
    }
    assert_eq!(actual, expected);
}

// use gluesql_core::data::Schema;

// pub fn draw_entity(schema: Schema) -> String {
//     let entity_name = schema.table_name;

//     match schema.column_defs {
//         Some(column_defs) => {
//             let columns = column_defs
//                 .iter()
//                 .map(|column_def| format!("│ {:<7}│\n", column_def.name))
//                 .collect::<String>();

//             format!(
//                 "┌────────┐\n│ {:<7}│\n├────────┤\n{}└────────┘",
//                 entity_name, columns
//             )
//         }
//         None => format!("┌───────┐\n│ {:<7}│\n└───────┘", entity_name),
//     }
// }

// pub fn merge_horizontal(art1: &str, art2: &str) -> String {
//     let lines1: Vec<&str> = art1.trim().lines().collect();
//     let lines2: Vec<&str> = art2.trim().lines().collect();
//     let max_lines = lines1.len().max(lines2.len());

//     // if (lines1.is_empty()) {
//     //     return;
//     // }

//     let mut combined = String::new();

//     for i in 0..max_lines {
//         let line1 = lines1.get(i).unwrap_or(&"");
//         let line2 = lines2.get(i).unwrap_or(&"");
//         combined.push_str(&format!("{}   {}\n", line1, line2));
//     }

//     combined.trim().to_owned()
// }

// #[cfg(test)]
// mod tests {
//     use std::fmt::{Debug, Display};

//     use gluesql_core::{
//         ast::{ColumnDef, DataType},
//         data::Schema,
//     };

//     use crate::util::merge_horizontal;

//     #[test]
//     fn test_draw_entity() {
//         let schema = Schema {
//             table_name: "Table1".to_owned(),
//             column_defs: Some(vec![
//                 ColumnDef {
//                     name: "Col1".to_owned(),
//                     data_type: DataType::Boolean,
//                     nullable: false,
//                     default: None,
//                     unique: None,
//                 },
//                 ColumnDef {
//                     name: "Col2".to_owned(),
//                     data_type: DataType::Boolean,
//                     nullable: false,
//                     default: None,
//                     unique: None,
//                 },
//             ]),
//             indexes: vec![],
//             engine: None,
//         };

//         let actual = super::draw_entity(schema);
//         let expected = "
// ┌────────┐
// │ Table1 │
// ├────────┤
// │ Col1   │
// │ Col2   │
// └────────┘"
//             .trim();

//         assert_text(actual, expected.to_owned());
//     }

//     #[test]
//     fn test_merge_horizontal() {
//         let art1 = "
// ┌────────┐
// │ Table1 │
// ├────────┤
// │ Col1   │
// │ Col2   │
// └────────┘";
//         let art2 = "
// ┌────────┐
// │ Table2 │
// ├────────┤
// │ Col1   │
// │ Col2   │
// └────────┘";
//         let actual = merge_horizontal(art1, art2);
//         let expected = "
// ┌────────┐   ┌────────┐
// │ Table1 │   │ Table2 │
// ├────────┤   ├────────┤
// │ Col1   │   │ Col1   │
// │ Col2   │   │ Col2   │
// └────────┘   └────────┘"
//             .trim();

//         assert_text(actual, expected.to_owned());
//     }

// }
