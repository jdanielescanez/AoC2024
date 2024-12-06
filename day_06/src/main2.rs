use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Obstruction,
    Marked,
}

impl Cell {
    pub fn new(c: char) -> Option<Self> {
        match c {
            '.' => Some(Cell::Empty),
            '#' => Some(Cell::Obstruction),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn new(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Right,
            '>' => Direction::Down,
            _ => Direction::Left,
        }
    }

    pub fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct GuardGallivant {
    map: Vec<Vec<Cell>>,
    guard_position: Position,
    guard_direction: Direction,
}

impl GuardGallivant {
    pub fn new(guard_gallivant_string: String) -> Self {
        let mut guard_direction = Direction::Up;
        let mut guard_position = Position { x: 0, y: 0 };
        let map: Vec<Vec<Cell>> = guard_gallivant_string
            .split('\n')
            .enumerate()
            .map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if Cell::new(c).is_some() {
                            Cell::new(c).unwrap()
                        } else {
                            guard_direction = Direction::new(c);
                            guard_position = Position { x: i, y: j };
                            Cell::Marked
                        }
                    })
                    .collect()
            })
            .collect();

        GuardGallivant {
            map,
            guard_position,
            guard_direction,
        }
    }

    pub fn guard_view(&self) -> Option<Cell> {
        let next_position = self.next_position();
        if next_position.is_some() {
            Some(self.get_cell_in_position(next_position?))
        } else {
            None
        }
    }

    pub fn get_cell_in_position(&self, pos: Position) -> Cell {
        self.map[pos.y][pos.x]
    }

    pub fn set_cell_in_position(&mut self, cell: Cell, pos: Position) {
        self.map[pos.y][pos.x] = cell;
    }

    pub fn run(&mut self) -> usize {
        let initial_dir = self.guard_direction;
        let initial_pos = self.guard_position.clone();
        let mut inside_map = self.step();

        while inside_map
            && !(self.guard_direction == initial_dir && self.guard_position == initial_pos)
        {
            inside_map = self.step();
        }

        let path = self
            .map
            .clone()
            .into_iter()
            .enumerate()
            .map(|(j, row)| {
                row.into_iter().enumerate().filter_map(move |(i, cell)| {
                    let candidate_position = Position { x: i, y: j };
                    if cell == Cell::Marked && candidate_position != initial_pos {
                        Some(candidate_position)
                    } else {
                        None
                    }
                })
            })
            .flatten();

        let initial_map = self.map.clone();
        path.filter(|pos| {
            self.map = initial_map.clone();
            self.guard_position = initial_pos;
            self.guard_direction = initial_dir;
            self.set_cell_in_position(Cell::Obstruction, *pos);

            let mut hash = HashSet::<(Position, Direction)>::new();
            let mut inside_map = true;
            while inside_map {
                let guard = (self.guard_position, self.guard_direction);
                if hash.contains(&guard) {
                    return true;
                } else {
                    hash.insert(guard);
                }
                inside_map = self.step();
            }
            false
        })
        .count()
    }

    pub fn step(&mut self) -> bool {
        match self.guard_view() {
            Some(Cell::Empty) | Some(Cell::Marked) => {
                self.guard_position = self.next_position().unwrap();
                self.set_cell_in_position(Cell::Marked, self.guard_position.clone());
                true
            }
            Some(Cell::Obstruction) => {
                self.guard_direction = self.guard_direction.next();
                self.step()
            }
            _ => false,
        }
    }

    pub fn next_position(&self) -> Option<Position> {
        let (pre_x, pre_y) = (self.guard_position.x, self.guard_position.y);
        self.validate_position(match self.guard_direction {
            Direction::Up => Position {
                x: pre_x,
                y: pre_y.checked_sub(1)?,
            },
            Direction::Right => Position {
                x: pre_x + 1,
                y: pre_y,
            },
            Direction::Down => Position {
                x: pre_x,
                y: pre_y + 1,
            },
            Direction::Left => Position {
                x: pre_x.checked_sub(1)?,
                y: pre_y,
            },
        })
    }

    pub fn validate_position(&self, pos: Position) -> Option<Position> {
        let height: usize = self.map.len();
        let width = self.map[0].len();
        if pos.x < width && pos.y < height {
            Some(pos)
        } else {
            None
        }
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");

        std::process::exit(1)
    });

    let guard_gallivant_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let mut guard_gallivant = GuardGallivant::new(guard_gallivant_string);
    let result = guard_gallivant.run();
    println!("\n{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let guard_gallivant_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");

        let mut guard_gallivant = GuardGallivant::new(guard_gallivant_string);
        let result = guard_gallivant.run();
        assert_eq!(result, 6);
    }
}
