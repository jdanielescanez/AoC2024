use itertools::Itertools;
use std::{collections::HashSet, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cell {
    id: char,
    position: Position,
}

struct Map {
    cells: Vec<Cell>,
    n_rows: usize,
    n_cols: usize,
    visited: HashSet<Cell>,
}

impl Map {
    pub fn new(map_string: String) -> Self {
        let rows = map_string.lines().collect::<Vec<&str>>();
        let n_rows = rows.len();

        let cells = rows
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().map(move |(x, id)| Cell {
                    id,
                    position: Position { x: x + 1, y: y + 1 },
                })
            })
            .collect::<Vec<Cell>>();

        let n_cols = cells.len() / n_rows;
        let visited = HashSet::new();
        Map {
            cells,
            n_rows,
            n_cols,
            visited,
        }
    }

    fn is_valid_position(&self, position: &Position) -> bool {
        position.x <= self.n_cols && position.y <= self.n_rows
    }

    fn get_cell(&self, position: &Position) -> Cell {
        if position.x == 0 || position.y == 0 || !self.is_valid_position(position) {
            Cell {
                id: '.',
                position: *position,
            }
        } else {
            self.cells[(position.y - 1) * self.n_rows + (position.x - 1)]
        }
    }

    fn get_neighbours(&self, cell: Cell) -> Vec<Cell> {
        let position = cell.position;
        vec![
            Position {
                x: position.x,
                y: position.y + 1,
            },
            Position {
                x: position.x + 1,
                y: position.y,
            },
            Position {
                x: position.x,
                y: position.y - 1,
            },
            Position {
                x: position.x - 1,
                y: position.y,
            },
        ]
        .into_iter()
        .map(|position| self.get_cell(&position))
        .collect()
    }

    fn get_diagonal(&self, first_neighbour: Cell, second_neighbour: Cell, current: Cell) -> Cell {
        let position = if current.position.x == first_neighbour.position.x {
            Position {
                x: second_neighbour.position.x,
                y: first_neighbour.position.y,
            }
        } else {
            Position {
                x: first_neighbour.position.x,
                y: second_neighbour.position.y,
            }
        };
        self.get_cell(&position)
    }

    fn step(&mut self, cell: Cell) -> (Vec<Cell>, usize) {
        self.visited.insert(cell);
        let all_neighbours = self.get_neighbours(cell);
        let (equal_neighbours, unequal_neighbours): (Vec<Cell>, Vec<Cell>) = all_neighbours
            .clone()
            .into_iter()
            .partition(|neighbour| neighbour.id == cell.id);

        let mut all_neighbours = self.get_neighbours(cell);
        all_neighbours.push(all_neighbours[0]);
        let direct_vertex = all_neighbours
            .windows(2)
            .filter(|cells| {
                unequal_neighbours.contains(&cells[0]) && unequal_neighbours.contains(&cells[1])
            })
            .count();
        let undirect_vertex = all_neighbours
            .windows(2)
            .filter(|cells| {
                equal_neighbours.contains(&cells[0])
                    && equal_neighbours.contains(&cells[1])
                    && self.get_diagonal(cells[0], cells[1], cell).id != cell.id
            })
            .count();

        let unvisited_neighbours = equal_neighbours
            .into_iter()
            .filter(|neighbour| !self.visited.contains(neighbour))
            .collect();

        (unvisited_neighbours, direct_vertex + undirect_vertex)
    }

    fn recursive_step(&mut self, cell: Cell) -> (Vec<Cell>, usize) {
        if self.visited.contains(&cell) {
            return (vec![], 0);
        }
        let (mut equal_neighbours, mut partial_perimeter) = self.step(cell);
        let size = equal_neighbours.clone().into_iter().count();
        let (mut next_equal_neighbours, next_partial_perimeter) = if size > 0 {
            equal_neighbours
                .iter()
                .map(|&next_cell| self.recursive_step(next_cell))
                .fold(
                    (vec![], 0),
                    |(mut acc_neighbours, acc_perimeter),
                     (mut next_equal_neighbours, mut next_partial_perimeter)| {
                        next_equal_neighbours.append(&mut acc_neighbours);
                        next_partial_perimeter += acc_perimeter;
                        (next_equal_neighbours, next_partial_perimeter)
                    },
                )
        } else {
            (vec![], 0)
        };

        equal_neighbours.push(cell);
        equal_neighbours.append(&mut next_equal_neighbours);
        partial_perimeter += next_partial_perimeter;
        (equal_neighbours, partial_perimeter)
    }

    fn get_price(&mut self) -> usize {
        let mut result = 0;
        for &cell in self.cells.clone().iter() {
            if !self.visited.contains(&cell) {
                let (path, partial_perimeter) = self.recursive_step(cell);
                result += path.iter().unique().count() * partial_perimeter;
            }
        }
        result
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
    let result_part1 = map.get_price();
    println!("Result part 2: {}", result_part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xo_example() {
        let map_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");
        let mut map = Map::new(map_string);
        assert_eq!(map.get_price(), 436);
    }

    #[test]
    fn e_example() {
        let map_string =
            fs::read_to_string("test2.txt").expect("Should have been able to read the file");
        let mut map = Map::new(map_string);
        assert_eq!(map.get_price(), 236);
    }

    #[test]
    fn ab_example() {
        let map_string =
            fs::read_to_string("test3.txt").expect("Should have been able to read the file");
        let mut map = Map::new(map_string);
        assert_eq!(map.get_price(), 368);
    }
}
