use std::collections::HashMap;
use std::fs;

fn line_to_stones(line: String) -> Vec<u64> {
    line.split_whitespace()
        .map(|stone_string| stone_string.parse::<u64>().unwrap())
        .collect()
}

fn single_blink(stone: u64) -> Vec<u64> {
    match stone {
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
    }
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|&stone| single_blink(stone))
        .collect()
}

fn part_1(mut stones: Vec<u64>) -> usize {
    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.len()
}

fn part_2(mut stones: Vec<u64>) -> usize {
    let mut result = 0;
    let mut table = HashMap::new();

    // 15 blinks
    for _ in 0..15 {
        stones = blink(&stones);
    }

    for stone in stones {
        let mut mid_frontier = vec![stone];
        // 30 + 15 = 45 blinks
        for _ in 0..30 {
            mid_frontier = blink(&mid_frontier);
            table.insert(stone, mid_frontier.len());
        }
        for frontier_stone in mid_frontier {
            // 30 + 30 + 15 = 75 blinks
            if table.contains_key(&frontier_stone) {
                result += table[&frontier_stone];
            } else {
                let mut last_frontier = vec![frontier_stone];
                for _ in 0..30 {
                    last_frontier = blink(&last_frontier);
                }
                table.insert(frontier_stone, last_frontier.len());
                result += last_frontier.len();
            }
        }
    }
    result
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let stones_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let stones: Vec<u64> = line_to_stones(stones_string);

    let result_part1 = part_1(stones.clone());
    println!("Result part 1: {}", result_part1);

    let result_part2 = part_2(stones);
    println!("Result part 2: {}", result_part2);
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
