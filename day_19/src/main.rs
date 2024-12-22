mod parser;

use parser::read_input;
use regex::Regex;
use std::fs;

fn get_result_part1(regex_string: String, designs: Vec<&str>) -> usize {
    let regex = Regex::new(&regex_string).unwrap();
    designs
        .into_iter()
        .filter(|pattern| regex.is_match(&pattern))
        .count()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let towel_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, (regex_string, designs)) = read_input(&towel_string).unwrap();
    let result_part1 = get_result_part1(regex_string, designs);
    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let towel_string =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");
        let (_, (regex_string, designs)) = read_input(&towel_string).unwrap();
        let result = get_result_part1(regex_string, designs);
        assert_eq!(result, 6);
    }
}
