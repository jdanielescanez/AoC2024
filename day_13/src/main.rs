mod behavior;
mod parser;
use std::fs;

use behavior::Behavior;
use parser::read_input;

fn get_result(behaviors: Vec<Behavior>) -> u64 {
    behaviors
        .iter()
        .filter_map(|behavior| behavior.get_min_solution())
        .map(|solution| solution.a * 3 + solution.b)
        .sum()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let behaviors_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let (_, mut behaviors) = read_input(&behaviors_string).unwrap();
    let result_part_1: u64 = get_result(behaviors.clone());
    println!("Result part 1: {}", result_part_1);

    behaviors
        .iter_mut()
        .for_each(|behavior| behavior.set_unit_conversion_correction());
    let result_part_2: u64 = get_result(behaviors);
    println!("Result part 2: {}", result_part_2);
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_read_behavior() {
        let behaviors_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");
        let (_, behaviors) = read_input(&behaviors_string).unwrap();
        let result = get_result(behaviors);
        assert_eq!(result, 480)
    }
}
