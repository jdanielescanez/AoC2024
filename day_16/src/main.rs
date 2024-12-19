mod parser;
mod reindeer;

use parser::read_input;
use reindeer::Reindeer;
use std::fs;

fn get_result_part1(reindeer: &Reindeer) -> u32 {
    reindeer.clone().run()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let reindeer_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, reindeer) = read_input(&reindeer_string).unwrap();
    let result_part1 = get_result_part1(&reindeer);
    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_small_part1() {
        let reindeer_string =
            fs::read_to_string("test_small.txt").expect("Should have been able to read the file");
        let (_, reindeer) = read_input(&reindeer_string).unwrap();
        let result = get_result_part1(&reindeer);
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_large_part1() {
        let reindeer_string =
            fs::read_to_string("test_large.txt").expect("Should have been able to read the file");
        let (_, reindeer) = read_input(&reindeer_string).unwrap();
        let result = get_result_part1(&reindeer);
        assert_eq!(result, 11048);
    }
}
