use regex::Regex;
use std::fs;

fn mull_it_over(memory: &str) -> u32 {
    let enabled_memory = memory
        .split("do()")
        .map(|expr|
            expr.split("don't()")
                .collect::<Vec<&str>>()[0]
        )
        .collect::<Vec<&str>>()
        .join("");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&enabled_memory).fold(0, |acc, capture| {
        let x = capture[1].parse::<u32>().unwrap();
        let y = capture[2].parse::<u32>().unwrap();
        acc + x * y
    })
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let memory = fs::read_to_string(input_filename)
        .expect("Should have been able to read the file");

    let result = mull_it_over(&memory);
    println!("\n{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_mul() {
        assert_eq!(mull_it_over("mul(2,4)"), 8);
    }

    #[test]
    fn two_muls() {
        assert_eq!(mull_it_over("mul(2,4)mul(5,5)"), 33);
    }

    #[test]
    fn two_muls_with_corruption() {
        assert_eq!(mull_it_over("mul(2,4)fkpdf@#6mul(5,5)"), 33);
    }

    #[test]
    fn invalid_mul() {
        assert_eq!(mull_it_over("mul(4*"), 0);
        assert_eq!(mull_it_over("mul(6,9!"), 0);
        assert_eq!(mull_it_over("?(12,34)"), 0);
        assert_eq!(mull_it_over("mul ( 2 , 4 )"), 0);
        assert_eq!(mull_it_over("mul(1234,34)"), 0);
    }

    #[test]
    fn long_example() {
        assert_eq!(mull_it_over("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }
}
