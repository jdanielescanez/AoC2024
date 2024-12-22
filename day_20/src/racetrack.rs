use std::collections::{HashMap, HashSet};

use nom::lib::std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Racetrack {
    pub map: Vec<Vec<Cell>>,
    pub start: Position,
    pub end: Position,
}

impl Racetrack {
    pub fn get_valid_wall_positions(&self) -> Vec<Position> {
        let mut valid_wall_positions = vec![];
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                let current_position = Position::new((x, y));
                let is_wall = cell == &Cell::Wall;
                let is_vertical_aisle = self
                    .get_vertical_neighbours(current_position)
                    .into_iter()
                    .all(|position| self.map[position.y][position.x] == Cell::Empty);
                let is_horizontal_aisle = self
                    .get_horizontal_neighbours(current_position)
                    .into_iter()
                    .all(|position| self.map[position.y][position.x] == Cell::Empty);
                if is_wall && (is_vertical_aisle || is_horizontal_aisle) {
                    valid_wall_positions.push(current_position);
                }
            }
        }
        valid_wall_positions
    }

    pub fn get_picoseconds(&self) -> u32 {
        let mut frontier: HashSet<Position> = HashSet::new();
        frontier.insert(self.start);
        let mut g_score: HashMap<Position, u32> = HashMap::new();
        g_score.insert(self.start, 0);
        let mut f_score: HashMap<Position, u32> = HashMap::new();
        f_score.insert(self.start, self.heuristic(self.start));

        while frontier.len() > 0 {
            let current_position = frontier
                .clone()
                .into_iter()
                .min_by_key(|candidate| f_score[candidate])
                .unwrap();
            if current_position == self.end {
                return f_score[&current_position];
            }
            frontier.remove(&current_position);
            for neighbour in self.get_neighbours(current_position) {
                let tentative_g_score = g_score[&current_position] + 1;
                if !g_score.contains_key(&neighbour) || tentative_g_score < g_score[&neighbour] {
                    g_score.insert(neighbour, tentative_g_score);
                    f_score.insert(neighbour, tentative_g_score + self.heuristic(neighbour));
                    frontier.insert(neighbour);
                }
            }
        }

        0
    }

    fn heuristic(&self, position: Position) -> u32 {
        (position.x.abs_diff(self.end.x) + position.y.abs_diff(self.end.y)) as u32
    }

    fn get_neighbours(&self, current_position: Position) -> Vec<Position> {
        [
            (current_position.x as i32, current_position.y as i32 - 1),
            (current_position.x as i32, current_position.y as i32 + 1),
            (current_position.x as i32 + 1, current_position.y as i32),
            (current_position.x as i32 - 1, current_position.y as i32),
        ]
        .into_iter()
        .filter(|&tuple_positon| {
            self.get_cell(tuple_positon)
                .is_some_and(|cell| cell == Cell::Empty)
        })
        .map(|tuple_positon| Position::new((tuple_positon.0 as usize, tuple_positon.1 as usize)))
        .collect::<Vec<Position>>()
    }

    fn get_vertical_neighbours(&self, current_position: Position) -> Vec<Position> {
        [
            (current_position.x as i32, current_position.y as i32 - 1),
            (current_position.x as i32, current_position.y as i32 + 1),
        ]
        .into_iter()
        .filter(|&tuple_positon| self.get_cell(tuple_positon).is_some())
        .map(|tuple_positon| Position::new((tuple_positon.0 as usize, tuple_positon.1 as usize)))
        .collect::<Vec<Position>>()
    }

    fn get_horizontal_neighbours(&self, current_position: Position) -> Vec<Position> {
        [
            (current_position.x as i32 + 1, current_position.y as i32),
            (current_position.x as i32 - 1, current_position.y as i32),
        ]
        .into_iter()
        .filter(|&tuple_positon| self.get_cell(tuple_positon).is_some())
        .map(|tuple_positon| Position::new((tuple_positon.0 as usize, tuple_positon.1 as usize)))
        .collect::<Vec<Position>>()
    }

    fn get_cell(&self, (x, y): (i32, i32)) -> Option<Cell> {
        if 0 <= x && x < self.map[0].len() as i32 && 0 <= y && y < self.map.len() as i32 {
            Some(self.map[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn _write_map(&self, marked_position: &Position) {
        self.map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if marked_position == (&Position { x, y }) {
                    print!("X");
                } else if (self.start == Position { x, y }) {
                    print!("S");
                } else if (self.end == Position { x, y }) {
                    print!("E");
                } else {
                    print!("{}", cell);
                }
            });
            print!("\n");
        });
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Cell {
    Wall,
    Empty,
}

impl Cell {
    pub fn new(cell: char) -> Option<Self> {
        match cell {
            '#' => Some(Cell::Wall),
            'E' | 'S' | '.' => Some(Cell::Empty),
            _ => None,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_char = match self {
            Cell::Wall => '#',
            Cell::Empty => '.',
        };
        write!(f, "{}", cell_char)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new((x, y): (usize, usize)) -> Self {
        Position { x, y }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
