use std::collections::{HashMap, HashSet};

use nom::lib::std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Reindeer {
    pub map: Vec<Vec<Cell>>,
    pub start: Position,
    pub end: Position,
}

impl Reindeer {
    pub fn run(&mut self) -> u32 {
        let mut frontier: HashSet<DirectedPosition> = HashSet::new();
        let directed_start = DirectedPosition {
            horizontal: true,
            position: self.start,
        };
        frontier.insert(directed_start);
        let mut came_from: HashMap<DirectedPosition, DirectedPosition> = HashMap::new();
        came_from.insert(
            directed_start,
            DirectedPosition {
                horizontal: true,
                position: Position {
                    x: self.start.x - 1,
                    y: self.start.y,
                },
            },
        );
        let mut g_score: HashMap<DirectedPosition, u32> = HashMap::new();
        g_score.insert(directed_start, 0);
        let mut f_score: HashMap<DirectedPosition, u32> = HashMap::new();
        f_score.insert(directed_start, self.heuristic(self.start));

        while frontier.len() > 0 {
            let current_position = frontier
                .clone()
                .into_iter()
                .min_by_key(|candidate| f_score[candidate])
                .unwrap();
            if current_position.position == self.end {
                return f_score[&current_position];
            }
            frontier.remove(&current_position);
            for neighbour in self.get_neighbours(current_position.position) {
                let predecessor = came_from[&current_position];
                let tentative_g_score = g_score[&current_position]
                    + self.distance(
                        current_position.position,
                        neighbour.position,
                        predecessor.position,
                    );
                if !g_score.contains_key(&neighbour) || tentative_g_score < g_score[&neighbour] {
                    came_from.insert(neighbour, current_position);
                    g_score.insert(neighbour, tentative_g_score);
                    f_score.insert(
                        neighbour,
                        tentative_g_score + self.heuristic(neighbour.position),
                    );
                    frontier.insert(neighbour);
                }
            }
        }

        0
    }

    fn distance(&self, position: Position, neighbour: Position, predecessor: Position) -> u32 {
        if position.x == predecessor.x && position.x == neighbour.x
            || position.y == predecessor.y && position.y == neighbour.y
        {
            1
        } else {
            1001
        }
    }

    fn heuristic(&self, position: Position) -> u32 {
        (position.x.abs_diff(self.end.x) + position.y.abs_diff(self.end.y)) as u32
    }

    fn get_neighbours(&self, current_position: Position) -> Vec<DirectedPosition> {
        [
            DirectedPosition {
                horizontal: true,
                position: Position {
                    x: current_position.x + 1,
                    y: current_position.y,
                },
            },
            DirectedPosition {
                horizontal: true,
                position: Position {
                    x: current_position.x - 1,
                    y: current_position.y,
                },
            },
            DirectedPosition {
                horizontal: false,
                position: Position {
                    x: current_position.x,
                    y: current_position.y - 1,
                },
            },
            DirectedPosition {
                horizontal: false,
                position: Position {
                    x: current_position.x,
                    y: current_position.y + 1,
                },
            },
        ]
        .into_iter()
        .filter(|position| self.get_cell(&position.position) == &Cell::Empty)
        .collect()
    }

    fn get_cell(&self, position: &Position) -> &Cell {
        &self.map[position.y][position.x]
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

#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct DirectedPosition {
    pub position: Position,
    pub horizontal: bool,
}
