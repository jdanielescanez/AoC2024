use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn multiply(&self, constant: usize) -> Position {
        Position {
            x: constant * self.x,
            y: constant * self.y,
        }
    }

    pub fn checked_sub(&self, position: Position) -> Option<Position> {
        let result_x = self.x.checked_sub(position.x);
        let result_y = self.y.checked_sub(position.y);
        if result_x.is_some() && result_y.is_some() {
            Some(Position {
                x: result_x?,
                y: result_y?,
            })
        } else {
            None
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
                    position: Position { x, y },
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
        position.x < self.n_cols && position.y < self.n_rows
    }

    fn place_antinode(&mut self, position: Position) {
        self.cells[position.y * self.n_rows + position.x].is_antinode = true;
    }

    fn place_resulting_antinode_pair(&mut self, first_pos: Position, second_pos: Position) {
        vec![
            first_pos.multiply(2).checked_sub(second_pos),
            second_pos.multiply(2).checked_sub(first_pos),
        ]
        .into_iter()
        .filter(|x| x.is_some())
        .for_each(|position| {
            if self.valid_position(&position.unwrap()) {
                self.place_antinode(position.unwrap())
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
        assert_eq!(map.compute_antinodes(), 14);
    }
}
