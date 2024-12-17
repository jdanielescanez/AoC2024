mod computer;
mod parser;

use computer::Computer;
use parser::read_input;
use std::fs;

fn get_result_part1(computer: &mut Computer) -> String {
    computer
        .run()
        .into_iter()
        .map(|output| output.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let computer_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, mut computer) = read_input(&computer_string).unwrap();
    let result_part1 = get_result_part1(&mut computer);
    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let computer_string = &format!(
            "{}\n{}\n{}\n\n{}",
            "Register A: 729", "Register B: 0", "Register C: 0", "Program: 0,1,5,4,3,0"
        );
        let (_, mut computer) = read_input(&computer_string).unwrap();
        let result = get_result_part1(&mut computer);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1_bdv() {
        let computer_string = &format!(
            "{}\n{}\n{}\n\n{}",
            "Register A: 0", "Register B: 1", "Register C: 9", "Program: 2,6"
        );
        let (_, mut computer) = read_input(&computer_string).unwrap();
        let _ = get_result_part1(&mut computer);
        assert_eq!(computer.b_register, 1);
    }

    #[test]
    fn test_part1_out() {
        let computer_string = &format!(
            "{}\n{}\n{}\n\n{}",
            "Register A: 10", "Register B: 0", "Register C: 0", "Program: 5,0,5,1,5,4"
        );
        let (_, mut computer) = read_input(&computer_string).unwrap();
        let result = get_result_part1(&mut computer);
        assert_eq!(result, "0,1,2");
    }

    #[test]
    fn test_part1_adv_jnz() {
        let computer_string = &format!(
            "{}\n{}\n{}\n\n{}",
            "Register A: 2024", "Register B: 0", "Register C: 0", "Program: 0,1,5,4,3,0"
        );
        let (_, mut computer) = read_input(&computer_string).unwrap();
        let result = get_result_part1(&mut computer);
        assert_eq!(result, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.a_register, 0);
    }

    #[test]
    fn test_part1_bxl() {
        let computer_string = &format!(
            "{}\n{}\n{}\n\n{}",
            "Register A: 0", "Register B: 29", "Register C: 0", "Program: 1,7"
        );
        let (_, mut computer) = read_input(&computer_string).unwrap();
        let _ = get_result_part1(&mut computer);
        assert_eq!(computer.b_register, 26);
    }

    #[test]
    fn test_part1_bxc() {
        let computer_string = &format!(
            "{}\n{}\n{}\n\n{}",
            "Register A: 0", "Register B: 2024", "Register C: 43690", "Program: 4,0"
        );
        let (_, mut computer) = read_input(&computer_string).unwrap();
        let _ = get_result_part1(&mut computer);
        assert_eq!(computer.c_register, 43690);
    }
}
