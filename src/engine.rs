use gluesql_core::{ast::DataType, data::Schema};

#[derive(PartialEq, Debug)]
struct Entity {
    name: String,
    attributes: Vec<Attribute>,
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct Attribute {
    name: String,
    data_type: DataType,
    reffered_by: Option<(String, String)>,
    reffering_to: Option<(String, String)>,
}

fn into_entities(schemas: Vec<Schema>) -> Vec<Entity> {
    const LAYOUT_SIZE: usize = 100;
    const MARGIN: usize = 1;
    const ENTITY_WIDTH: usize = 12; // includes line width
    const GAP: usize = 3;
    let mut max_y = MARGIN;

    schemas
        .iter()
        .enumerate()
        .map(|(index, schema)| {
            let attributes: Vec<Attribute> = schema
                .column_defs
                .as_ref()
                .map(|column_defs| {
                    column_defs
                        .into_iter()
                        .map(|column_def| {
                            let reffering_to = schema.foreign_keys.iter().find_map(|fk| {
                                (fk.referencing_column_name == column_def.name).then_some((
                                    fk.referenced_table_name.clone(),
                                    fk.referenced_column_name.clone(),
                                ))
                            });
                            let reffered_by = schemas.iter().find_map(|sch| {
                                sch.foreign_keys
                                    .iter()
                                    .find(|fk| {
                                        fk.referenced_table_name == schema.table_name
                                            && fk.referenced_column_name == column_def.name
                                    })
                                    .map(|fk| {
                                        (sch.table_name.clone(), fk.referencing_column_name.clone())
                                    })
                            });

                            Attribute {
                                name: column_def.name.clone(),
                                data_type: column_def.data_type.clone(),
                                reffered_by,
                                reffering_to,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            // width of layout: 100
            // height of layout: 100
            // init starts with 1,1
            // width of Entity: 12
            // height of Entity: 1 + 1 + 1 + columns.len() + 1
            // gap between Entity: 3
            // if x is over than 400, move next line, which means it should start over the max(y)
            // should memorize max(y) with scan? => temporarily use mut global var

            let x = MARGIN + (ENTITY_WIDTH + GAP) * index;
            let y = max_y;
            let (x, y) = match x > LAYOUT_SIZE {
                true => (
                    MARGIN
                        + (ENTITY_WIDTH + GAP)
                            * (index % ((LAYOUT_SIZE - MARGIN) / (ENTITY_WIDTH + GAP))),
                    y + attributes.len() + 1 + GAP,
                ),
                false => (x, y),
            };
            max_y = y;

            Entity {
                name: schema.table_name.clone(),
                attributes,
                x,
                y,
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use gluesql_core::data::Schema;

    use super::*;

    #[test]
    fn test_engine() {
        let sqls = "
CREATE TABLE Tab1 (col1 INT, col2 INT);
CREATE TABLE Tab2 (col1 INT, col2 INT);
CREATE TABLE Tab3 (col1 INT, col2 INT);
CREATE TABLE Tab4 (col1 INT, col2 INT);
CREATE TABLE Tab5 (col1 INT, col2 INT);
CREATE TABLE Tab6 (col1 INT, col2 INT);
CREATE TABLE Tab7 (col1 INT, col2 INT);
CREATE TABLE Tab8 (col1 INT, col2 INT);
CREATE TABLE Tab9 (col1 INT, col2 INT);
CREATE TABLE Tab10 (col1 INT, col2 INT);
";
        let schemas = sqls
            .split(";")
            .filter_map(|sql| Schema::from_ddl(sql).ok())
            .collect::<Vec<_>>();

        let entities = into_entities(schemas);

        let positions: Vec<(String, usize, usize)> = entities
            .iter()
            .map(|entity| (entity.name.clone(), entity.x, entity.y))
            .collect();

        assert_eq!(
            positions,
            vec![
                ("Tab1".to_owned(), 1, 1),
                ("Tab2".to_owned(), 16, 1),
                ("Tab3".to_owned(), 31, 1),
                ("Tab4".to_owned(), 46, 1),
                ("Tab5".to_owned(), 61, 1),
                ("Tab6".to_owned(), 76, 1),
                ("Tab7".to_owned(), 91, 1),
                ("Tab8".to_owned(), 16, 7),
                ("Tab9".to_owned(), 31, 13),
                ("Tab10".to_owned(), 46, 19),
            ]
        );
    }
}
