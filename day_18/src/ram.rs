use std::collections::{HashMap, HashSet};

use nom::lib::std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Ram {
    map: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
    size: usize,
}

impl Ram {
    pub fn new(bytes: Vec<Position>, size: usize, fallen_bytes: usize) -> Self {
        let bytes = &bytes[..fallen_bytes];
        let mut map = vec![];
        for y in 0..=size {
            map.push(vec![]);
            for x in 0..=size {
                let position = Position::new((x, y));
                let cell = if bytes.contains(&position) {
                    Cell::Wall
                } else {
                    Cell::Empty
                };
                map[y].push(cell);
            }
        }
        let start = Position { x: 0, y: 0 };
        let end = Position { x: size, y: size };
        Ram {
            map,
            start,
            end,
            size,
        }
    }

    pub fn run(&mut self) -> u32 {
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
            (current_position.x as i32 + 1, current_position.y as i32),
            (current_position.x as i32 - 1, current_position.y as i32),
            (current_position.x as i32, current_position.y as i32 - 1),
            (current_position.x as i32, current_position.y as i32 + 1),
        ]
        .into_iter()
        .filter(|&tuple_positon| {
            self.get_cell(tuple_positon)
                .is_some_and(|cell| cell == Cell::Empty)
        })
        .map(|tuple_positon| Position::new((tuple_positon.0 as usize, tuple_positon.1 as usize)))
        .collect::<Vec<Position>>()
    }

    fn get_cell(&self, (x, y): (i32, i32)) -> Option<Cell> {
        if 0 <= x && x <= self.size as i32 && 0 <= y && y <= self.size as i32 {
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
    x: usize,
    y: usize,
}

impl Position {
    pub fn new((x, y): (usize, usize)) -> Self {
        Position { x, y }
    }
}
