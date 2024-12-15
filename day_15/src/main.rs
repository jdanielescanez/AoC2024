mod parser;
mod warehouse;

use parser::read_input;
use std::fs;
use warehouse::Warehouse;

fn get_result_part1(warehouse: &Warehouse, verbose: bool) -> usize {
    warehouse.clone().run(verbose)
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let warehouse_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, warehouse) = read_input(&warehouse_string).unwrap();
    let result_part1 = get_result_part1(&warehouse, false);
    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_small_part1() {
        let warehouse_string =
            fs::read_to_string("test_small.txt").expect("Should have been able to read the file");
        let (_, warehouse) = read_input(&warehouse_string).unwrap();
        let result = get_result_part1(&warehouse, true);
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_large_part1() {
        let warehouse_string =
            fs::read_to_string("test_large.txt").expect("Should have been able to read the file");
        let (_, warehouse) = read_input(&warehouse_string).unwrap();
        let result = get_result_part1(&warehouse, true);
        assert_eq!(result, 10092);
    }
}
