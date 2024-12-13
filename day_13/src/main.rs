mod behavior;
mod parser;
use std::fs;

use parser::read_input;

fn main() {
    let behaviors_string =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let (_, behaviors) = read_input(&behaviors_string).unwrap();
    let result: u32 = behaviors
        .into_iter()
        .filter_map(|behavior| behavior.get_min_solution())
        .filter(|solution| solution.a <= 100 && solution.b <= 100)
        .map(|solution| {
            dbg!(&solution.a, &solution.b, 3 * &solution.a + &solution.b);
            solution.a * 3 + solution.b
        })
        .sum();
    println!("{}", result);
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_read_behavior() {
        let behaviors_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");
        let (_, behaviors) = read_input(&behaviors_string).unwrap();
        let result: u32 = behaviors
            .into_iter()
            .filter_map(|behavior| behavior.get_min_solution())
            .filter(|solution| solution.a < 100 && solution.b < 100)
            .map(|solution| {
                dbg!(&solution.a, &solution.b);
                solution.a * 3 + solution.b
            })
            .sum();
        assert_eq!(result, 480)
    }
}
