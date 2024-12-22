mod parser;
mod racetrack;

use parser::read_input;
use racetrack::{Cell, Racetrack};
use std::fs;

fn get_result_part1(racetrack: &Racetrack) -> usize {
    let score = racetrack.clone().get_picoseconds();
    racetrack
        .get_valid_wall_positions()
        .into_iter()
        .map(|wall_position| {
            let mut alternative_racetrack = racetrack.clone();
            alternative_racetrack.map[wall_position.y][wall_position.x] = Cell::Empty;
            score - alternative_racetrack.get_picoseconds()
        })
        .filter(|saved| saved >= &100)
        .count()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let racetrack_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, racetrack) = read_input(&racetrack_string).unwrap();
    let result_part1 = get_result_part1(&racetrack);
    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let racetrack_string =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");
        let (_, racetrack) = read_input(&racetrack_string).unwrap();
        let score = racetrack.clone().get_picoseconds();
        let result: u32 = racetrack
            .get_valid_wall_positions()
            .into_iter()
            .map(|wall_position| {
                let mut alternative_racetrack = racetrack.clone();
                alternative_racetrack.map[wall_position.y][wall_position.x] = Cell::Empty;
                score - alternative_racetrack.get_picoseconds()
            })
            .sum();
        assert_eq!(result, 382);
    }
}
