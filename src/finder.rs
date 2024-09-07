use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "Up",
                Direction::Down => "Down",
                Direction::Left => "Left",
                Direction::Right => "Right",
                Direction::None => "None",
            }
        )
    }
}

enum Sign {
    Pos,
    Neg,
}

enum Way {
    Horizontal { vertical: Sign },
    Vertical { horizontal: Sign },
}

// way to go should decide the direction from x, y to target x, y
// should we decide the priorty whther to go horizontal or vertical first?
// answer: no, we should decide the direction based on the current position
// and the target position
//
struct WayToGo {
    way: Way,
    sign: Sign,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn next(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
            Direction::None => Point::new(self.x, self.y),
        }
    }

    fn back(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
            Direction::Left => Point::new(self.x + 1, self.y),
            Direction::Right => Point::new(self.x - 1, self.y),
            Direction::None => Point::new(self.x, self.y),
        }
    }

    fn get_char(&self, canvas: &Vec<Vec<char>>) -> Char {
        Char::from_char(canvas[self.y][self.x])
    }
}

#[derive(Debug, PartialEq)]
enum Char {
    Horizontal,
    Vertical,
    UpRight,
    DownRight,
    RightUp,
    RightDown,
    FkFrom,
    FkTo,
    None,
}
impl Display for Char {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Char {
    fn as_char(&self) -> char {
        match self {
            Char::Horizontal => '─',
            Char::Vertical => '│',
            Char::UpRight => '┌',
            Char::DownRight => '└',
            Char::RightUp => '┘',
            Char::RightDown => '┐',
            Char::FkFrom => '├',
            Char::FkTo => '→',
            Char::None => ' ',
        }
    }

    fn from_char(ch: char) -> Self {
        match ch {
            '─' => Char::Horizontal,
            '│' => Char::Vertical,
            '┌' => Char::UpRight,
            '└' => Char::DownRight,
            '┘' => Char::RightUp,
            '┐' => Char::RightDown,
            '├' => Char::FkFrom,
            '→' => Char::FkTo,
            ' ' => Char::None,
            _ => panic!("Invalid char"),
        }
    }
}

struct Diagram {
    canvas: Vec<Vec<char>>,
    point: Point,
    target: Point,
}
impl Diagram {
    fn new(content: Vec<Vec<char>>, target: Point) -> Self {
        Diagram {
            canvas: content,
            point: Point::new(0, 0),
            target,
        }
    }

    fn next(&mut self, direction: &Direction) {
        self.point = self.point.next(direction);
    }

    fn get_char(&self) -> char {
        let Point { x, y } = self.point;

        self.canvas[y][x]
    }

    fn set_char(&mut self, ch: char) {
        let Point { x, y } = self.point;

        self.canvas[y][x] = ch;
    }

    fn to_string(&self) -> String {
        self.canvas
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn anlayze_direction(&mut self, direction: Option<Direction>) -> Direction {
        let Point { x, y } = self.point;
        let Point {
            x: target_x,
            y: target_y,
        } = self.target;

        let horizontal_distance = (target_x as i32 - x as i32).abs();
        let vertical_distance = (target_y as i32 - y as i32).abs();

        let try_direction = direction.unwrap_or(
            match (
                vertical_distance < horizontal_distance,
                target_x < x,
                target_y < y,
            ) {
                (true, true, _) => Direction::Left,
                (true, false, _) => Direction::Right,
                (false, _, true) => Direction::Up,
                (false, false, false) if horizontal_distance == 0 && vertical_distance == 0 => {
                    return Direction::None
                }
                (false, _, false) => Direction::Down,
            },
        );

        // let try_direction = if prev_direction == try_direction {
        //     match try_direction {
        //         Direction::Up | Direction::Down => {
        //             // try left or right depending on the distance to target
        //             if target_x < x {
        //                 Direction::Left
        //             } else {
        //                 Direction::Right
        //             }
        //         }
        //         Direction::Left | Direction::Right => {
        //             // try up or down depending on the distance to target
        //             if target_y < y {
        //                 Direction::Up
        //             } else {
        //                 Direction::Down
        //             }
        //         }
        //         Direction::None => unreachable!(),
        //     }
        // } else {
        //     try_direction
        // };

        let cur_char = self.point.get_char(&self.canvas);
        let try_char = self.point.next(&try_direction).get_char(&self.canvas);

        // do i need cur_char to analyze the direction? yes if it has to make the corner
        let next_direction = match (cur_char, try_direction, try_char) {
            (cur_char, direction, Char::None) => {
                match (cur_char, &direction) {
                    (Char::Horizontal, Direction::Up) => {
                        match x < target_x {
                            true => self.set_char(Char::RightUp.as_char()),
                            false => self.set_char(Char::DownRight.as_char()),
                        }
                        self.next(&direction);
                        self.set_char(Char::Vertical.as_char())
                    }
                    (Char::Horizontal, Direction::Down) => {
                        match x < target_x {
                            true => self.set_char(Char::RightDown.as_char()),
                            false => self.set_char(Char::UpRight.as_char()),
                        }
                        self.next(&direction);
                        self.set_char(Char::Vertical.as_char())
                    }
                    (Char::Vertical, Direction::Left) => {
                        match y < target_y {
                            true => self.set_char(Char::RightUp.as_char()),
                            false => self.set_char(Char::RightDown.as_char()),
                        }
                        self.next(&direction);
                        self.set_char(Char::Horizontal.as_char())
                    }

                    (Char::Vertical, Direction::Right) => {
                        match y < target_y {
                            true => self.set_char(Char::DownRight.as_char()),
                            false => self.set_char(Char::RightUp.as_char()),
                        }
                        self.next(&direction);
                        self.set_char(Char::Horizontal.as_char())
                    }
                    (cur_char, direction) => {
                        println!("cur_char: {:?}, direction: {:?}", cur_char, direction);
                        self.next(direction);
                        match direction {
                            Direction::Right | Direction::Left => {
                                self.set_char(Char::Horizontal.as_char())
                            }
                            Direction::Up | Direction::Down => {
                                self.set_char(Char::Vertical.as_char())
                            }
                            Direction::None => {}
                        }
                    }
                }

                return Direction::None;
            }
            (Char::Horizontal, Direction::Right, Char::Vertical) => {
                if y < target_y {
                    self.set_char(Char::RightDown.as_char());
                    Direction::Down
                } else {
                    self.set_char(Char::RightUp.as_char());
                    Direction::Up
                }
            }
            (Char::Horizontal, Direction::Left, Char::Vertical) => {
                if y < target_y {
                    self.set_char(Char::UpRight.as_char());
                    Direction::Down
                } else {
                    self.set_char(Char::DownRight.as_char());
                    Direction::Up
                }
            }
            (Char::Vertical, Direction::Up, Char::Horizontal) => {
                if x < target_x {
                    self.set_char(Char::UpRight.as_char());
                    Direction::Right
                } else {
                    self.set_char(Char::RightDown.as_char());
                    Direction::Left
                }
            }
            (Char::Vertical, Direction::Down, Char::Horizontal) => {
                if x < target_x {
                    self.set_char(Char::DownRight.as_char());
                    Direction::Right
                } else {
                    self.set_char(Char::RightUp.as_char());
                    Direction::Left
                }
            }
            (Char::Horizontal, Direction::Right, Char::FkTo)
            | (Char::DownRight, Direction::Right, Char::FkTo)
            | (Char::UpRight, Direction::Right, Char::FkTo) => {
                return Direction::None;
            }
            (Char::Vertical, Direction::Right, _) => {
                if y < target_y {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            // (Char::Vertical, Direction::Right, Char::DownRight) => {}
            // vertical, up , rightdown
            x => unreachable!("{:?}", x),
            // (Char::Vertical, Direction::Right, Char::Horizontal)
            // | (Char::Horizontal, Direction::Up, Char::Vertical)
            // | (Char::Horizontal, Direction::Down, Char::Vertical)
            // | (Char::Horizontal, _, Char::Horizontal)
            // | (Char::Vertical, _, Char::Vertical)
            // | (Char::Vertical, Direction::Left, Char::Horizontal) => unreachable!(),
        };

        // if try_char == Char::None {
        //     // draw the line
        //     // let (cur_char_new, next_char) = match (cur_char, try_direction) {
        //     //     (Char::Horizontal, Direction::Up) => (Char::LeftUp, Char::Vertical),
        //     //     _ => todo!(),
        //     // };
        // } else {
        //     // try another
        // }

        self.anlayze_direction(Some(next_direction))
    }

    // fn next(self) -> Self {
    //     let direction = self.anlayze_direction();
    //     self.p

    //     return self;
    // }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use cursive::vec;

    use crate::finder::{Diagram, Direction, Point};

    #[test]
    fn test_point() {
        let actual_source = "
 ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐            
 │Tab1      │   │Tab2      │   │Tab3      │   │Tab4      │   │Tab5      │   │Tab6      │            
 ├──────────┤   ├──────────┤   ├──────────┤   ├──────────┤   ├──────────┤   ├──────────┤            
 │col1      ├   │col1      │   │col1      │   │col1      │   │col1      │   │col1      │            
 │col2      │   │col2      │   │col2      │   │col2      │   │col2      │   │col2      │            
 │col3      │   └──────────┘   │col3      │   └──────────┘   └──────────┘   └──────────┘            
 │col4      │                  │col4      │                                                         
 └──────────┘                  │col5      │                                                         
                               │col6      │                                                         
                ┌──────────┐   └──────────┘   ┌──────────┐   ┌──────────┐   ┌──────────┐            
                │Tab8      │                  │Tab10     │   │Tab11     │   │Tab12     │            
 ┌──────────┐   ├──────────┤                  ├──────────┤   ├──────────┤   ├──────────┤            
 │Tab7      │   │col1      │                  │col1      │   │col1      │   │col1      │            
 ├──────────┤   │col2      │   ┌──────────┐   │col2      │   │col2      │   │col2      │            
 │col1      │   └──────────┘   │Tab9      │   └──────────┘   └──────────┘   └──────────┘            
 │col2      │                  ├──────────┤                                                         
 └──────────┘                  │col1      │                                                         
                               │col2      │                                                         
                ┌──────────┐   └──────────┘   ┌──────────┐                                          
                │Tab14     │                  │Tab16     │                                          
 ┌──────────┐   ├──────────┤                  ├──────────┤                                          
 │Tab13     │   │col1      │                  │col1      │                                          
 ├──────────┤   │col2      │   ┌──────────┐   │col2      │                                          
 │col1      │   └──────────┘   │Tab15     │   └──────────┘                                          
 │col2      │                  ├──────────┤                                                         
 └──────────┘                  │col1      │                                                         
                               │col2      │                                                         
                               └──────────┘
";
        let expected = "
 ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐            
 │Tab1      │   │Tab2      │   │Tab3      │   │Tab4      │   │Tab5      │   │Tab6      │            
 ├──────────┤   ├──────────┤   ├──────────┤   ├──────────┤   ├──────────┤   ├──────────┤            
 │col1      ├──┐│col1      │   │col1      │   │col1      │   │col1      │   │col1      │            
 │col2      │  ││col2      │   │col2      │   │col2      │   │col2      │   │col2      │            
 │col3      │  │└──────────┘   │col3      │   └──────────┘   └──────────┘   └──────────┘            
 │col4      │  └──────────────┐│col4      │                                                         
 └──────────┘                 ││col5      │                                                         
                              ││col6      │                                                         
                ┌──────────┐  │└──────────┘   ┌──────────┐   ┌──────────┐   ┌──────────┐            
                │Tab8      │  └─────────────┐ │Tab10     │   │Tab11     │   │Tab12     │            
 ┌──────────┐   ├──────────┤                │ ├──────────┤   ├──────────┤   ├──────────┤            
 │Tab7      │   │col1      │                └─│col1      │   │col1      │   │col1      │            
 ├──────────┤   │col2      │   ┌──────────┐   │col2      │   │col2      │   │col2      │            
 │col1      │   └──────────┘   │Tab9      │   └──────────┘   └──────────┘   └──────────┘            
 │col2      │                  ├──────────┤                                                         
 └──────────┘                  │col1      │                                                         
                               │col2      │                                                         
                ┌──────────┐   └──────────┘   ┌──────────┐                                          
                │Tab14     │                  │Tab16     │                                          
 ┌──────────┐   ├──────────┤                  ├──────────┤                                          
 │Tab13     │   │col1      │                  │col1      │                                          
 ├──────────┤   │col2      │   ┌──────────┐   │col2      │                                          
 │col1      │   └──────────┘   │Tab15     │   └──────────┘                                          
 │col2      │                  ├──────────┤                                                         
 └──────────┘                  │col1      │                                                         
                               │col2      │                                                         
                               └──────────┘
";

        let mut canvas = Diagram {
            canvas: actual_source
                .split("\n")
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            point: Point::new(12, 4),
            target: Point::new(46, 13),
        };
        assert_eq!(canvas.get_char(), '├');

        // anaylze the direction
        // mutate cur_char and next_char if needed
        //
        for _ in 0..43 {
            canvas.anlayze_direction(None);
        }
        // canvas.next();
        println!(
            "{}",
            canvas.to_string().chars().take(2000).collect::<String>()
        );

        // let canvas = canvas.next();
        // assert_eq!(canvas.point, Point::new(13, 4));
        // assert_eq!(canvas.get_char(), '─');

        // assert_eq!(direction, Direction::Up);
    }
}
