use crate::engine::Entity;
// pub struct Entity {
//     pub name: String,
//     pub attributes: Vec<Attribute>,
//     pub x: usize,
//     pub y: usize,
// }
// pub struct Attribute {
//     pub name: String,
//     pub data_type: DataType,
//     pub reffered_by: Option<(String, String)>,
//     pub reffering_to: Option<(String, String)>,
// }

// ┌───────┐       ┌──────┐
// │ Table1│       │Table2│
// ├───────┤       ├──────┤
// │ Col1  │       │Col1  │
// │ Col2  │       │Col2  │
// │       │       │      │
// └───────┘       └──────┘
fn render(entities: Vec<Entity>) -> String {
    let mut canvas = vec![vec![' '; 100]; 100]; // Assuming a 100x100 canvas for simplicity

    for entity in entities {
        let Entity {
            x,
            y,
            name,
            attributes,
        } = entity;

        // Draw top border
        for i in 0..9 {
            canvas[y][x + i] = if i == 0 {
                '┌'
            } else if i == 8 {
                '┐'
            } else {
                '─'
            };
        }

        // Draw name
        canvas[y + 1][x] = '│';
        for (i, c) in name.chars().enumerate() {
            canvas[y + 1][x + 1 + i] = c;
        }
        canvas[y + 1][x + 8] = '│';

        // Draw middle border
        for i in 0..9 {
            canvas[y + 2][x + i] = if i == 0 {
                '├'
            } else if i == 8 {
                '┤'
            } else {
                '─'
            };
        }

        // Draw attributes
        for (i, attribute) in attributes.iter().enumerate() {
            canvas[y + 3 + i][x] = '│';
            for (j, c) in attribute.name.chars().enumerate() {
                canvas[y + 3 + i][x + 1 + j] = c;
            }
            canvas[y + 3 + i][x + 8] = '│';
        }

        // Draw bottom border
        let bottom_y = y + 3 + attributes.len();
        for i in 0..9 {
            canvas[bottom_y][x + i] = if i == 0 {
                '└'
            } else if i == 8 {
                '┘'
            } else {
                '─'
            };
        }
    }

    // Convert canvas to string
    canvas
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use gluesql_core::data::Schema;

    use crate::{engine::into_entities, util::assert_text};

    use super::*;

    #[test]
    fn test_render() {
        let sqls = "
CREATE TABLE Tab1 (col1 INT, col2 INT, col3 INT, col4 INT);
CREATE TABLE Tab2 (col1 INT, col2 INT);
CREATE TABLE Tab3 (col1 INT, col2 INT, col3 INT, col4 INT, col5 INT, col6 INT);
CREATE TABLE Tab4 (col1 INT, col2 INT);
CREATE TABLE Tab5 (col1 INT, col2 INT);
CREATE TABLE Tab6 (col1 INT, col2 INT);
CREATE TABLE Tab7 (col1 INT, col2 INT);
CREATE TABLE Tab8 (col1 INT, col2 INT);
CREATE TABLE Tab9 (col1 INT, col2 INT);
CREATE TABLE Tab10 (col1 INT, col2 INT);
CREATE TABLE Tab11 (col1 INT, col2 INT);
CREATE TABLE Tab12 (col1 INT, col2 INT);
CREATE TABLE Tab13 (col1 INT, col2 INT);
CREATE TABLE Tab14 (col1 INT, col2 INT);
CREATE TABLE Tab15 (col1 INT, col2 INT);
CREATE TABLE Tab16 (col1 INT, col2 INT);
";
        let schemas = sqls
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect::<Vec<_>>();
        let entities = into_entities(schemas);

        let actual = render(entities);
        let expected = "
┌───────┐       ┌──────┐
│ Table1│       │Table2│
├───────┤       ├──────┤
│ Col1  │       │Col1  │
│ Col2  │       │Col2  │
│       │       │      │
└───────┘       └──────┘
        ";

        assert_text(actual, expected.to_owned());
    }
}
