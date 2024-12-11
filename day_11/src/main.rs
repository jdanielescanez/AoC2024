use std::fs;

fn line_to_stones(line: String) -> Vec<u64> {
    line.split_whitespace()
        .map(|stone_string| stone_string.parse::<u64>().unwrap())
        .collect()
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|stone| match stone {
            0 => vec![1],
            stone if stone.to_string().len() % 2 == 0 => {
                let stone_str = stone.to_string();
                let middle = stone_str.len() / 2;
                vec![
                    stone_str[..middle].parse::<u64>().unwrap(),
                    stone_str[middle..].parse::<u64>().unwrap(),
                ]
            }
            _ => vec![2024 * stone],
        })
        .collect()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let stones_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let mut stones: Vec<u64> = line_to_stones(stones_string);
    for _ in 0..25 {
        stones = blink(&stones);
    }
    let result_part1 = stones.len();
    println!("Result part 1: {}", result_part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_blink() {
        let input = "125 17";
        let mut stones: Vec<u64> = line_to_stones(input.to_string());
        stones = blink(&stones);
        assert_eq!(stones, line_to_stones("253000 1 7".to_string()));
    }

    #[test]
    fn two_blinks() {
        let input = "125 17";
        let mut stones: Vec<u64> = line_to_stones(input.to_string());
        for _ in 0..2 {
            stones = blink(&stones);
        }
        assert_eq!(stones, line_to_stones("253 0 2024 14168".to_string()));
    }

    #[test]
    fn six_blinks() {
        let input = "125 17";
        let mut stones: Vec<u64> = line_to_stones(input.to_string());
        for _ in 0..6 {
            stones = blink(&stones);
        }
        let output = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
        assert_eq!(stones, line_to_stones(output.to_string()));
    }
}
