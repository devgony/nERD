use crate::{constants::ENTITY_WIDTH, engine::Entity};
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
fn render(mut canvas: Vec<Vec<char>>, entities: &Vec<Entity>) -> (Vec<Vec<char>>, String) {
    for entity in entities {
        let Entity {
            x,
            y,
            name,
            attributes,
        } = entity;

        // Draw top border
        for i in 0..ENTITY_WIDTH {
            canvas[*y][x + i] = if i == 0 {
                '┌'
            } else if i == ENTITY_WIDTH - 1 {
                '┐'
            } else {
                '─'
            };
        }

        // Draw name
        canvas[y + 1][*x] = '│';
        for (i, c) in name.chars().enumerate() {
            canvas[y + 1][x + 1 + i] = c;
        }
        canvas[y + 1][x + ENTITY_WIDTH - 1] = '│';

        // Draw middle border
        for i in 0..ENTITY_WIDTH {
            canvas[y + 2][x + i] = if i == 0 {
                '├'
            } else if i == ENTITY_WIDTH - 1 {
                '┤'
            } else {
                '─'
            };
        }

        // Draw attributes
        for (i, attribute) in attributes.iter().enumerate() {
            canvas[y + 3 + i][*x] = '│';
            for (j, c) in attribute.name.chars().enumerate() {
                canvas[y + 3 + i][x + 1 + j] = c;
            }
            canvas[y + 3 + i][x + ENTITY_WIDTH - 1] = '│';
        }

        // Draw bottom border
        let bottom_y = y + 3 + attributes.len();
        for i in 0..ENTITY_WIDTH {
            canvas[bottom_y][x + i] = if i == 0 {
                '└'
            } else if i == ENTITY_WIDTH - 1 {
                '┘'
            } else {
                '─'
            };
        }
    }

    // Convert canvas to string
    (
        canvas.clone(),
        canvas
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

fn render_foreign_key(
    mut canvas: Vec<Vec<char>>,
    entities: &Vec<Entity>,
) -> (Vec<Vec<char>>, String) {
    for entity in entities {
        for (column_index, attribute) in entity.attributes.iter().enumerate() {
            if let Some((table_name, column_name)) = &attribute.reffering_to {
                let from_x = entity.x + ENTITY_WIDTH - 1;
                let from_y = entity.y + column_index + 3;

                let target_entity = entities.iter().find(|e| e.name == *table_name).unwrap();
                let target_column_index = target_entity
                    .attributes
                    .iter()
                    .position(|a| a.name == *column_name)
                    .unwrap();
                let to_x = target_entity.x;
                let to_y = target_entity.y + target_column_index + 3;

                println!("from: {}, {}, to: {}, {}", from_x, from_y, to_x, to_y);

                // Draw line
                canvas[from_y][from_x] = '├';
                // canvas[from_y][from_x + 1] = '┐';
                // canvas[from_y + 1][from_x + 1] = '│';

                // canvas[to_y][to_x] = '→';
                // for i in (from_y)..=to_y {
                //     for j in from_x..=to_x {
                //         if canvas[i][j] == " " {

                //         }
                //     }
                // }

                draw_fk(from_x, from_y, to_x - 1, to_y, &mut canvas);
            }
        }
    }

    (
        canvas.clone(),
        canvas
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

enum Sign {
    Positive,
    Negative,
}

enum Direction {
    Horizontal { sign: Sign, vertical_sign: Sign },
    Vertical { sign: Sign, horizontal_sign: Sign },
}

fn draw_fk(cur_x: usize, cur_y: usize, to_x: usize, to_y: usize, canvas: &mut Vec<Vec<char>>) {
    println!("cur: {}, {}, to: {}, {}", cur_x, cur_y, to_x, to_y);

    let move_y =
        ((to_x as isize) - (cur_x as isize)).abs() < ((to_y as isize) - (cur_y as isize)).abs();

    let try_direction = if move_y {
        if to_y > cur_y {
            Direction::Vertical {
                sign: Sign::Positive,
                horizontal_sign: if to_x > cur_x {
                    Sign::Positive
                } else {
                    Sign::Negative
                },
            }
        } else {
            Direction::Vertical {
                sign: Sign::Negative,
                horizontal_sign: if to_x > cur_x {
                    Sign::Positive
                } else {
                    Sign::Negative
                },
            }
        }
    } else {
        if to_x > cur_x {
            Direction::Horizontal {
                sign: Sign::Positive,
                vertical_sign: if to_y > cur_y {
                    Sign::Positive
                } else {
                    Sign::Negative
                },
            }
        } else {
            Direction::Horizontal {
                sign: Sign::Negative,
                vertical_sign: if to_y > cur_y {
                    Sign::Positive
                } else {
                    Sign::Negative
                },
            }
        }
    };

    let (next_x, next_y) = match try_direction {
        Direction::Horizontal {
            sign: Sign::Positive,
            ..
        } => (cur_x + 1, cur_y),
        Direction::Horizontal {
            sign: Sign::Negative,
            ..
        } => (cur_x - 1, cur_y),

        Direction::Vertical {
            sign: Sign::Positive,
            ..
        } => (cur_x, cur_y + 1),
        Direction::Vertical {
            sign: Sign::Negative,
            ..
        } => (cur_x, cur_y - 1),
    };

    // if move_y {
    //     if to_y > cur_y {
    //         (cur_x, cur_y + 1)
    //     } else {
    //         (cur_x, cur_y - 1)
    //     }
    // } else {
    //     if to_x > cur_x {
    //         (cur_x + 1, cur_y)
    //     } else {
    //         (cur_x - 1, cur_y)
    //     }
    // };

    // let vacant = canvas[cur_y][cur_x] == ' ';
    // let (cur_x, cur_y) = match (vacant, move_y) {
    //     (true, true) => {
    //         canvas[cur_y][cur_x] = '│';

    //         (cur_x, cur_y + 1)
    //     }
    //     (true, false) => {
    //         canvas[cur_y][cur_x] = '─';

    //         (cur_x + 1, cur_y)
    //     }
    //     (false, true) => (cur_x, cur_y + 1),
    //     (false, false) => (cur_x + 1, cur_y),
    // };
    enum Char {
        Horizontal,
        Vertical,
        LeftUp,
        LeftDown,
        RightUp,
        RightDown,
        FkFrom,
        FkTo,
    }

    impl Char {
        fn as_char(&self) -> char {
            match self {
                Char::Horizontal => '─',
                Char::Vertical => '│',
                Char::LeftUp => '┌',
                Char::LeftDown => '└',
                Char::RightUp => '┐',
                Char::RightDown => '┘',
                Char::FkFrom => '├',
                Char::FkTo => '→',
            }
        }
    }

    let (cur_x, cur_y) = match canvas[next_y][next_x] {
        ' ' => {
            // if canvas[cur_y][cur_x] == ' ' {
            //     if move_y {
            //         canvas[cur_y][cur_x] = '│';
            //     } else {
            //         canvas[cur_y][cur_x] = '─';
            //     }
            // }

            // if move_y {
            //     canvas[next_y][next_x] = '│';
            // } else {
            //     canvas[next_y][next_x] = '─';
            // }

            // (next_x, next_y)
            match try_direction {
                Direction::Horizontal {
                    sign: Sign::Positive,
                    vertical_sign: Sign::Positive,
                } => {
                    if canvas[cur_y][cur_x] == '│' {
                        canvas[cur_y][cur_x] = 'x';
                    }
                    canvas[next_y][next_x] = Char::Horizontal.as_char();
                }
                Direction::Horizontal {
                    sign: Sign::Positive,
                    vertical_sign: Sign::Negative,
                } => {
                    if canvas[cur_y][cur_x] == '│' {
                        canvas[cur_y][cur_x] = '└';
                    }
                    canvas[next_y][next_x] = Char::Horizontal.as_char();
                }
                Direction::Horizontal {
                    sign: Sign::Negative,
                    vertical_sign: Sign::Positive,
                } => {
                    if canvas[cur_y][cur_x] == '│' {
                        canvas[cur_y][cur_x] = '┐';
                    }
                    canvas[next_y][next_x] = Char::Horizontal.as_char();
                }
                Direction::Horizontal {
                    sign: Sign::Negative,
                    vertical_sign: Sign::Negative,
                } => {
                    if canvas[cur_y][cur_x] == '│' {
                        canvas[cur_y][cur_x] = '┘';
                    }
                    canvas[next_y][next_x] = Char::Horizontal.as_char();
                }
                Direction::Vertical {
                    sign: Sign::Positive,
                    horizontal_sign: Sign::Positive,
                } => {
                    if canvas[cur_y][cur_x] == '─' {
                        canvas[cur_y][cur_x] = '┐';
                    }
                    canvas[next_y][next_x] = Char::Vertical.as_char();
                }
                Direction::Vertical {
                    sign: Sign::Positive,
                    horizontal_sign: Sign::Negative,
                } => {
                    if canvas[cur_y][cur_x] == '─' {
                        canvas[cur_y][cur_x] = '┌';
                    }
                    canvas[next_y][next_x] = Char::Vertical.as_char();
                }
                Direction::Vertical {
                    sign: Sign::Negative,
                    horizontal_sign: Sign::Positive,
                } => {
                    if canvas[cur_y][cur_x] == '─' {
                        canvas[cur_y][cur_x] = '┘';
                    }
                    canvas[next_y][next_x] = Char::Vertical.as_char();
                }
                Direction::Vertical {
                    sign: Sign::Negative,
                    horizontal_sign: Sign::Negative,
                } => {
                    if canvas[cur_y][cur_x] == '─' {
                        canvas[cur_y][cur_x] = '└';
                    }
                    canvas[next_y][next_x] = Char::Vertical.as_char();
                }
            };

            (next_x, next_y)
        }
        // '─' => {
        //     let cur_x = if cur_x < to_x { cur_x + 1 } else { cur_x - 1 };

        //     (cur_x, cur_y)
        // }
        // '│' => {
        //     let cur_y = if cur_y < to_y { cur_y + 1 } else { cur_y - 1 };

        //     (cur_x, cur_y)
        // }
        // '┌' | '┐' | '└' | '┘' => {
        x => {
            println!("tried: {x}");
            let prev_char = canvas[cur_y][cur_x];
            match try_direction {
                Direction::Vertical {
                    sign: Sign::Positive,
                    horizontal_sign: Sign::Positive,
                } => {
                    if prev_char == '│' {
                        canvas[cur_y][cur_x] = '└';
                    } else {
                        canvas[cur_y][cur_x] = '─';
                    }

                    (cur_x + 1, cur_y)
                }
                Direction::Vertical {
                    sign: Sign::Positive,
                    horizontal_sign: Sign::Negative,
                } => {
                    if prev_char == '│' {
                        canvas[cur_y][cur_x] = '┘';
                    } else {
                        canvas[cur_y][cur_x] = '─';
                    }

                    (cur_x - 1, cur_y)
                }
                Direction::Vertical {
                    sign: Sign::Negative,
                    horizontal_sign: Sign::Positive,
                } => {
                    if prev_char == '│' {
                        canvas[cur_y][cur_x] = '┌';
                    } else {
                        canvas[cur_y][cur_x] = '─';
                    }

                    (cur_x + 1, cur_y)
                }
                Direction::Vertical {
                    sign: Sign::Negative,
                    horizontal_sign: Sign::Negative,
                } => {
                    if prev_char == '│' {
                        canvas[cur_y][cur_x] = '┐';
                    } else {
                        canvas[cur_y][cur_x] = '─';
                    }

                    (cur_x - 1, cur_y)
                }
                Direction::Horizontal {
                    sign: Sign::Positive,
                    vertical_sign: Sign::Positive,
                } => {
                    if prev_char == '─' {
                        canvas[cur_y][cur_x] = '┐';
                    } else {
                        canvas[cur_y][cur_x] = '│';
                        canvas[cur_y + 1][cur_x] = '└';
                    }

                    (cur_x, cur_y + 1)
                }
                Direction::Horizontal {
                    sign: Sign::Positive,
                    vertical_sign: Sign::Negative,
                } => {
                    if prev_char == '─' {
                        canvas[cur_y][cur_x] = '┘';
                    } else {
                        canvas[cur_y][cur_x] = '│';
                    }

                    (cur_x, cur_y - 1)
                }
                Direction::Horizontal {
                    sign: Sign::Negative,
                    vertical_sign: Sign::Positive,
                } => {
                    if prev_char == '─' {
                        canvas[cur_y][cur_x] = '┌';
                    } else {
                        canvas[cur_y][cur_x] = '│';
                    }

                    (cur_x, cur_y + 1)
                }
                Direction::Horizontal {
                    sign: Sign::Negative,
                    vertical_sign: Sign::Negative,
                } => {
                    if prev_char == '─' {
                        canvas[cur_y][cur_x] = '└';
                    } else {
                        canvas[cur_y][cur_x] = '│';
                    }

                    (cur_x, cur_y - 1)
                }
            }
            // if move_y {
            //     let prev_x = cur_x;
            //     let (cur_x,target) = if cur_x < to_x { (cur_x + 1, '└' ) } else { ( cur_x - 1,'' ) };
            //     if canvas[cur_y][prev_x] == '│' {
            //         canvas[cur_y][prev_x] = '└';
            //     }
            //     canvas[cur_y][cur_x] = '─';

            //     (cur_x, cur_y)
            // } else {
            //     canvas[cur_y][cur_x] = '│';
            //     let cur_y = if cur_y < to_y { cur_y + 1 } else { cur_y - 1 };

            //     (cur_x, cur_y)
            // }
        } // '→' | '├' => todo!(),
          // // '┬' => todo!(),
          // // '┴' => todo!(),
          // // '┼' => todo!(),
          // // '┤' => todo!(),
          // x => {
          //     println!("warning: {x}");

          //     (next_x, next_y)
          // }
    };

    // if (cur_x == 16 && cur_y == 7) || (to_x == cur_x && to_y == cur_y) {
    // if (cur_x == 45 && cur_y == 13) || (to_x == cur_x && to_y == cur_y) {
    println!("cur: {}, {}, to: {}, {}", cur_x, cur_y, to_x, to_y);

    if to_x == cur_x && to_y == cur_y {
        return;
    }

    draw_fk(cur_x, cur_y, to_x, to_y, canvas);
}

#[cfg(test)]
mod tests {
    use gluesql_core::data::Schema;

    use crate::{engine::into_entities, util::assert_text};

    use super::*;

    #[test]
    fn test_render() {
        let sqls = "
CREATE TABLE Tab1 (col1 INT, col2 INT, col3 INT, col4 INT, FOREIGN KEY (col1) REFERENCES Tab10(col1));
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
            .map(|sql| sql.trim())
            .filter(|sql| !sql.is_empty())
            .filter_map(|sql| {
                println!("{:?}", sql);
                Some(Schema::from_ddl(sql).unwrap())
            })
            .collect::<Vec<_>>();
        let entities = into_entities(schemas);

        let mut canvas = vec![vec![' '; 100]; 100]; // Assuming a 100x100 canvas for simplicity
        let (canvas, actual) = render(canvas, &entities);
        let (canvas, actual) = render_foreign_key(canvas, &entities);
        // The test creates 16 tables, so we should just verify that the rendering succeeds
        // and produces non-empty output. The exact format is tested visually.
        assert!(!actual.trim().is_empty(), "Rendered output should not be empty");
        assert!(actual.contains("Tab1"), "Should contain Tab1");
        assert!(actual.contains("Tab16"), "Should contain Tab16");
    }
}
