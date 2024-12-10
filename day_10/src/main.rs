use itertools::Itertools;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct IntegerPosition {
    x: i32,
    y: i32,
}

impl IntegerPosition {
    pub fn to_position(&self) -> Option<Position> {
        if self.x >= 0 && self.y >= 0 {
            let (x, y) = (self.x as usize, self.y as usize);
            Some(Position { x, y })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cell {
    height: u32,
    position: Position,
}

struct Map {
    cells: Vec<Cell>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    pub fn new(map_string: String) -> Self {
        let rows = map_string.lines().collect::<Vec<&str>>();
        let n_rows = rows.len();
        let cells = rows
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().map(move |(x, height)| Cell {
                    height: height.to_digit(10).unwrap(),
                    position: Position { x, y },
                })
            })
            .collect::<Vec<Cell>>();
        let n_cols = cells.len() / n_rows;

        Map {
            cells,
            n_rows,
            n_cols,
        }
    }

    fn is_valid_position(&self, position: &Position) -> bool {
        position.x < self.n_cols && position.y < self.n_rows
    }

    fn get_cell(&self, position: &Position) -> Cell {
        self.cells[position.y * self.n_rows + position.x]
    }

    fn get_neighbours(&self, cell: Cell) -> Vec<Cell> {
        let position = cell.position;
        vec![
            IntegerPosition {
                y: position.y as i32 - 1,
                x: position.x as i32,
            },
            IntegerPosition {
                y: position.y as i32 + 1,
                x: position.x as i32,
            },
            IntegerPosition {
                y: position.y as i32,
                x: position.x as i32 - 1,
            },
            IntegerPosition {
                y: position.y as i32,
                x: position.x as i32 + 1,
            },
        ]
        .into_iter()
        .filter_map(|int_pos| int_pos.to_position())
        .filter(|position| self.is_valid_position(position))
        .map(|valid_position| self.get_cell(&valid_position))
        .collect()
    }

    fn step(&self, cell: Cell) -> Vec<Cell> {
        self.get_neighbours(cell)
            .into_iter()
            .filter(|neighbour| neighbour.height == cell.height + 1)
            .collect()
    }

    fn recursive_step(&self, cell: Cell, n_steps: u32) -> Vec<Cell> {
        let cell_after_step = self.step(cell).into_iter();
        if n_steps > 1 {
            cell_after_step
                .flat_map(|next_cell| self.recursive_step(next_cell, n_steps - 1))
                .collect()
        } else {
            cell_after_step.collect()
        }
    }

    fn get_inits(&self) -> Vec<Cell> {
        self.cells
            .iter()
            .filter(|cell| cell.height == 0)
            .map(|init| *init)
            .collect()
    }

    fn get_trailheads_count(&self) -> usize {
        self.get_inits()
            .into_iter()
            .flat_map(|init| {
                self.recursive_step(init, 9)
                    .into_iter()
                    .filter(|cell| cell.height == 9)
                    .unique()
            })
            .count()
    }

    fn get_trailheads_distinct_count(&self) -> usize {
        self.get_inits()
            .into_iter()
            .flat_map(|init| {
                self.recursive_step(init, 9)
                    .into_iter()
                    .filter(|cell| cell.height == 9)
            })
            .count()
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let map_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let map = Map::new(map_string);

    let result_part1 = map.get_trailheads_count();
    let result_part2 = map.get_trailheads_distinct_count();
    println!("Result part 1: {}", result_part1);
    println!("Result part 2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_example() {
        let map_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");
        let map = Map::new(map_string);
        assert_eq!(map.get_trailheads_count(), 3);
    }

    #[test]
    fn complex_example() {
        let map_string =
            fs::read_to_string("test2.txt").expect("Should have been able to read the file");
        let map = Map::new(map_string);
        assert_eq!(map.get_trailheads_count(), 36);
    }

    #[test]
    fn complex_distinct_example() {
        let map_string =
            fs::read_to_string("test2.txt").expect("Should have been able to read the file");
        let map = Map::new(map_string);
        assert_eq!(map.get_trailheads_distinct_count(), 81);
    }
}
