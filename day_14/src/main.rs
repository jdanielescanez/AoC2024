mod parser;
mod robot;
use std::{collections::HashMap, fs};

use parser::read_input;
use robot::{Position, Quandrant, Robot};

fn get_result_part1(robots: &Vec<Robot>, limits: Position, steps: i32) -> u32 {
    let mut robots = robots.clone();
    let mut counter: HashMap<Quandrant, u32> = [
        (Quandrant::First, 0),
        (Quandrant::Second, 0),
        (Quandrant::Third, 0),
        (Quandrant::Fourth, 0),
    ]
    .into_iter()
    .collect();

    for robot in robots.iter_mut() {
        robot.move_steps(steps, limits);
        match robot.get_quadrant(limits) {
            Some(Quandrant::First) => *counter.get_mut(&Quandrant::First).unwrap() += 1,
            Some(Quandrant::Second) => *counter.get_mut(&Quandrant::Second).unwrap() += 1,
            Some(Quandrant::Third) => *counter.get_mut(&Quandrant::Third).unwrap() += 1,
            Some(Quandrant::Fourth) => *counter.get_mut(&Quandrant::Fourth).unwrap() += 1,
            None => (),
        }
    }
    counter.values().into_iter().product()
}

fn get_result_part2(robots: &Vec<Robot>, limits: Position) -> usize {
    let mut robots = robots.clone();
    for k in 1..100000 {
        robots
            .iter_mut()
            .for_each(|robot| robot.move_steps(1, limits));

        if robots
            .iter()
            .filter(|robot| {
                robot
                    .get_neighbour_positions(limits)
                    .into_iter()
                    .any(|neighbour_pos| robots.iter().any(|robot| robot.position == neighbour_pos))
            })
            .count() as f64
            > robots.len() as f64 * 0.5
        {
            print_robots(&robots, limits);
            return k;
        }
    }
    0
}

fn print_robots(robots: &Vec<Robot>, limits: Position) {
    for i in 0..limits.x {
        for j in 0..limits.y {
            if robots
                .iter()
                .any(|robot| robot.position == Position { x: i, y: j })
            {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let robots_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let (_, robots) = read_input(&robots_string).unwrap();
    let result_part1 = get_result_part1(&robots, Position { x: 101, y: 103 }, 100);
    let result_part2 = get_result_part2(&robots, Position { x: 101, y: 103 });

    println!("Result part 1: {}", result_part1);
    println!("Result part 2: {}", result_part2);
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let robots_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");
        let (_, robots) = read_input(&robots_string).unwrap();
        let result = get_result_part1(&robots, Position { x: 11, y: 7 }, 100);
        assert_eq!(result, 12);
    }
}
