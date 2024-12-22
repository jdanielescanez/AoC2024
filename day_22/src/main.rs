use std::fs;

use nom::{
    character::complete::{newline, u64},
    multi::many1,
    sequence::pair,
    IResult,
};

fn read_input(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, result) = (many1(pair(u64, newline)))(input)?;
    let result = result.into_iter().map(|(number, _)| number).collect();

    Ok((input, result))
}

fn mix_and_prune(value: u64, secret_number: u64) -> u64 {
    prune(mix(value, secret_number))
}

fn mix(value: u64, secret_number: u64) -> u64 {
    value ^ secret_number
}

fn prune(secret_number: u64) -> u64 {
    secret_number % 2_u64.pow(24)
}

fn iteration(secret_number: u64) -> u64 {
    let first_step = secret_number * 2_u64.pow(6);
    let secret_number = mix_and_prune(first_step, secret_number);

    let second_step = secret_number / 2_u64.pow(5);
    let secret_number = mix_and_prune(second_step, secret_number);

    let final_step = secret_number * 2_u64.pow(11);
    let secret_number = mix_and_prune(final_step, secret_number);

    secret_number
}

fn get_result_part1(market: Vec<u64>) -> u64 {
    market
        .into_iter()
        .map(|secret_number| (0..2000).fold(secret_number, |acc, _| iteration(acc)))
        .sum()
}

pub fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let market_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let (_, market) = read_input(&market_string).unwrap();
    let result_part1 = get_result_part1(market);

    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let vector = vec![22, 345, 8, 1, 156804, 1064864, 1, 3489];
        assert_eq!(
            read_input(&format!(
                "{}\n",
                vector
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )),
            Ok(("", vector))
        );
    }

    #[test]
    fn test_easy_part1() {
        let mut secret_number = 123;
        secret_number = iteration(secret_number);
        assert_eq!(secret_number, 15887950);
        secret_number = iteration(secret_number);
        assert_eq!(secret_number, 16495136);
        for _ in 0..8 {
            secret_number = iteration(secret_number);
        }
        assert_eq!(secret_number, 5908254);
    }

    #[test]
    fn test_hard_part1() {
        let market = vec![1, 10, 100, 2024];
        let result = get_result_part1(market);
        assert_eq!(result, 37327623);
    }
}
