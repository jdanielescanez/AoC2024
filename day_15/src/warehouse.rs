use nom::lib::std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Warehouse {
    pub moves: Vec<Direction>,
    pub map: Vec<Vec<Cell>>,
    pub robot_position: Position,
}

impl Warehouse {
    pub fn run(&mut self, verbose: bool) -> usize {
        self.moves.clone().into_iter().for_each(|direction| {
            self.step(direction);
            if verbose {
                print!("Move {}:\n", direction);
                self.write_map();
                print!("\n");
            }
        });

        let mut result = 0;
        self.map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if cell == &Cell::Box {
                    result += 100 * y + x;
                }
            })
        });
        result
    }

    fn step(&mut self, direction: Direction) {
        let position_to_move = match direction {
            Direction::Right => Position {
                x: self.robot_position.x + 1,
                y: self.robot_position.y,
            },
            Direction::Left => Position {
                x: self.robot_position.x - 1,
                y: self.robot_position.y,
            },
            Direction::Up => Position {
                x: self.robot_position.x,
                y: self.robot_position.y - 1,
            },
            Direction::Down => Position {
                x: self.robot_position.x,
                y: self.robot_position.y + 1,
            },
        };

        if self.get_position(&position_to_move) == &Cell::Empty || self.try_to_push(direction) {
            self.robot_position = position_to_move;
        }
    }

    fn try_to_push(&mut self, direction: Direction) -> bool {
        self.fix_map(&direction, false);
        let result = self.try_to_push_right();
        self.fix_map(&direction, true);
        result
    }

    fn fix_map(&mut self, direction: &Direction, second_time: bool) {
        match direction {
            Direction::Down => {
                self.transpose();
            }
            Direction::Left => {
                self.reverse();
            }
            Direction::Up => {
                if second_time {
                    self.reverse();
                    self.transpose();
                } else {
                    self.transpose();
                    self.reverse();
                }
            }
            Direction::Right => {}
        }
    }

    fn try_to_push_right(&mut self) -> bool {
        let row = self.map[self.robot_position.y].clone();
        let next_wall = row
            .iter()
            .enumerate()
            .find_map(|(i, cell)| {
                if i > self.robot_position.x && cell == &Cell::Wall {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();

        let next_empty = row.iter().enumerate().find_map(|(i, cell)| {
            if i < next_wall && i > self.robot_position.x && cell == &Cell::Empty {
                Some(i)
            } else {
                None
            }
        });

        if next_empty.is_some() {
            self.map[self.robot_position.y].remove(next_empty.unwrap());
            self.map[self.robot_position.y].insert(self.robot_position.x + 1, Cell::Empty);
            return true;
        }
        return false;
    }

    fn get_position(&self, position: &Position) -> &Cell {
        &self.map[position.y][position.x]
    }

    fn transpose(&mut self) {
        self.map = (0..self.map[0].len())
            .map(|i| self.map.iter().map(|inner| inner[i].clone()).collect())
            .collect();
        self.robot_position = Position {
            x: self.robot_position.y,
            y: self.robot_position.x,
        }
    }

    fn reverse(&mut self) {
        self.map.iter_mut().for_each(|row| row.reverse());
        self.robot_position = Position {
            x: self.map[0].len() - 1 - self.robot_position.x,
            y: self.robot_position.y,
        }
    }

    fn write_map(&self) {
        self.map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if (self.robot_position == Position { x, y }) {
                    print!("@");
                } else {
                    print!("{}", cell);
                }
            });
            print!("\n");
        });
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Cell {
    Box,
    Wall,
    Empty,
}

impl Cell {
    pub fn new(cell: char) -> Option<Self> {
        match cell {
            'O' => Some(Cell::Box),
            '#' => Some(Cell::Wall),
            '@' | '.' => Some(Cell::Empty),
            _ => None,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_char = match self {
            Cell::Box => 'O',
            Cell::Wall => '#',
            Cell::Empty => '.',
        };
        write!(f, "{}", cell_char)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn new(direction: char) -> Option<Self> {
        match direction {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction_char = match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        write!(f, "{}", direction_char)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
