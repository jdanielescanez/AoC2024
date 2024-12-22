mod parser;
mod ram;

use parser::read_input;
use ram::Ram;
use std::fs;

fn get_result_part1(ram: &Ram) -> u32 {
    ram.clone().run()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let ram_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    // TODO: Decouple functionality from the number of limiting bytes of the parser. It should be the responsibility of the Ram class.
    let (_, ram) = read_input(&ram_string, 70, 1024).unwrap();
    let result_part1 = get_result_part1(&ram);
    println!("Result part 1: {}", result_part1);

    let mut result_part2 = 0;
    // TODO: Add binary search
    for i in 0..=3450 {
        let (_, ram) = read_input(&ram_string, 70, i).unwrap();
        let result = get_result_part1(&ram);
        if result == 0 {
            result_part2 = i;
            break;
        }
    }
    println!("Result part 2: {}", result_part2);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let ram_string =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");
        let (_, ram) = read_input(&ram_string, 6, 12).unwrap();
        let result = get_result_part1(&ram);
        assert_eq!(result, 22);
    }
}
