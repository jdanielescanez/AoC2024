use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn multiply(&self, constant: i32) -> Position {
        Position {
            x: constant * self.x,
            y: constant * self.y,
        }
    }

    pub fn add(&self, position: Position) -> Position {
        Position {
            x: self.x + position.x,
            y: self.y + position.y,
        }
    }

    pub fn sub(&self, position: Position) -> Position {
        Position {
            x: self.x - position.x,
            y: self.y - position.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cell {
    tag: char,
    position: Position,
    is_antinode: bool,
}

struct Map {
    cells: Vec<Cell>,
    antennas: HashSet<char>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    pub fn new(map_string: String) -> Self {
        let rows = map_string.split('\n').collect::<Vec<&str>>();
        let n_rows = rows.len();
        let cells = rows
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().map(move |(x, tag)| Cell {
                    tag,
                    position: Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    is_antinode: false,
                })
            })
            .collect::<Vec<Cell>>();
        let n_cols = cells.len() / n_rows;
        let antennas = cells
            .iter()
            .map(|cell| cell.tag)
            .filter(|&tag| tag != '.')
            .collect::<HashSet<char>>();

        Map {
            cells,
            antennas,
            n_rows,
            n_cols,
        }
    }

    fn valid_position(&self, position: &Position) -> bool {
        0 <= position.x
            && position.x < self.n_cols as i32
            && 0 <= position.y
            && position.y < self.n_rows as i32
    }

    fn place_antinode(&mut self, position: Position) {
        self.cells[(position.y * self.n_rows as i32 + position.x) as usize].is_antinode = true;
    }

    fn place_resulting_antinode_pair(&mut self, first_pos: Position, second_pos: Position) {
        (0..=self.n_rows)
            .flat_map(|i| {
                vec![
                    first_pos.sub(first_pos.sub(second_pos).multiply(i as i32)),
                    first_pos.add(first_pos.sub(second_pos).multiply(i as i32)),
                ]
            })
            .for_each(|position| {
                if self.valid_position(&position) {
                    self.place_antinode(position)
                }
            })
    }

    pub fn compute_antinodes(&mut self) -> usize {
        let valid_antenna_pairs = self
            .antennas
            .iter()
            .flat_map(|antenna| {
                self.cells
                    .iter()
                    .filter_map(move |cell| (antenna == &cell.tag).then(|| cell.position))
                    .combinations(2)
                    .map(|x| (x[0], x[1]))
            })
            .collect::<Vec<(Position, Position)>>();

        for antenna_pair in valid_antenna_pairs {
            self.place_resulting_antinode_pair(antenna_pair.0, antenna_pair.1)
        }

        self.cells.iter().filter(|cell| cell.is_antinode).count()
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");

        std::process::exit(1)
    });

    let map_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let mut map = Map::new(map_string);
    let result = map.compute_antinodes();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let map_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");
        let mut map = Map::new(map_string);
        assert_eq!(map.compute_antinodes(), 34);
    }
}
