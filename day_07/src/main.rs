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
        let operators = vec![|a: u64, b: u64| a + b, |a: u64, b: u64| a * b];
        CalibrationEquiation {
            result,
            operands,
            operators,
        }
    }

    fn get_operands_value(&self, i: usize) -> Option<u64> {
        self.operands
            .clone()
            .into_iter()
            .skip(1)
            .enumerate()
            .try_fold(self.operands[0], |acc, (j, operand)| {
                let current_bit = (i >> j) & 1;
                let current_result = self.operators[current_bit](acc, operand);

                if current_result <= self.result {
                    Some(current_result)
                } else {
                    None
                }
            })
            .filter(|x| *x == self.result)
    }

    pub fn get_partial_result(&self) -> u64 {
        (0..2_usize.pow(self.operands.len() as u32 - 1))
            .find_map(|i| self.get_operands_value(i))
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
}
