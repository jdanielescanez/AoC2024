use itertools::{repeat_n, Itertools};
use std::fs;

struct CalibrationEquiation {
    result: u64,
    operands: Vec<u64>,
    operators: Vec<fn(u64, u64) -> u64>,
}

impl CalibrationEquiation {
    pub fn new(line: &str) -> Self {
        let mut iter_line = line.split(": ");
        let result = iter_line.next().unwrap().parse::<u64>().unwrap();
        let operands = iter_line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|operand| operand.parse::<u64>().unwrap())
            .collect();
        let operators = vec![
            |a: u64, b: u64| a + b,
            |a: u64, b: u64| a * b,
            |a: u64, b: u64| (a.to_string() + &b.to_string()).parse::<u64>().unwrap(),
        ];

        CalibrationEquiation {
            result,
            operands,
            operators,
        }
    }

    fn eval_combination(&self, operator_combination: Vec<fn(u64, u64) -> u64>) -> Option<u64> {
        self.operands
            .clone()
            .into_iter()
            .skip(1)
            .enumerate()
            .try_fold(self.operands[0], |acc, (j, operand)| {
                let current_result = operator_combination[j](acc, operand);

                if current_result <= self.result {
                    Some(current_result)
                } else {
                    None
                }
            })
            .filter(|x| *x == self.result)
    }

    pub fn get_partial_result(&self) -> u64 {
        repeat_n(self.operators.clone().into_iter(), self.operands.len())
            .multi_cartesian_product()
            .find_map(|operator_combination| self.eval_combination(operator_combination))
            .unwrap_or(0)
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");

        std::process::exit(1)
    });

    let calibration_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let result: u64 = calibration_string
        .split('\n')
        .map(|line| {
            let calibration_eq = CalibrationEquiation::new(&line);
            calibration_eq.get_partial_result()
        })
        .sum();
    println!("\n{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let calibration_eq = CalibrationEquiation::new("29: 10 19");
        assert_eq!(calibration_eq.get_partial_result(), 29);
    }

    #[test]
    fn mult() {
        let calibration_eq = CalibrationEquiation::new("190: 10 19");
        assert_eq!(calibration_eq.get_partial_result(), 190);
    }

    #[test]
    fn sum_and_mult() {
        let calibration_eq = CalibrationEquiation::new("3267: 81 40 27");
        assert_eq!(calibration_eq.get_partial_result(), 3267);
    }

    #[test]
    fn complex_sum_and_mult() {
        let calibration_eq = CalibrationEquiation::new("292: 11 6 16 20");
        assert_eq!(calibration_eq.get_partial_result(), 292);
    }

    #[test]
    fn no_operations_for_this() {
        let calibration_eq = CalibrationEquiation::new("21037: 9 7 18 13");
        assert_eq!(calibration_eq.get_partial_result(), 0);
    }

    #[test]
    fn concatenation() {
        let calibration_eq = CalibrationEquiation::new("156: 15 6");
        assert_eq!(calibration_eq.get_partial_result(), 156);
    }

    #[test]
    fn concatenation_and_sum() {
        let calibration_eq = CalibrationEquiation::new("192: 17 8 14");
        assert_eq!(calibration_eq.get_partial_result(), 192);
    }

    #[test]
    fn all_operations() {
        let calibration_eq = CalibrationEquiation::new("7290: 6 8 6 15");
        assert_eq!(calibration_eq.get_partial_result(), 7290);
    }
}
