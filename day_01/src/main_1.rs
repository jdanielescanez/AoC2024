
use std::fs;

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let mut lines = file_str.split('\n').collect::<Vec<&str>>();
    lines = lines[..lines.len()-1].to_vec();
    
    let mut left = vec![];
    let mut right = vec![];

    lines.into_iter().for_each(|line| {
        let row = line.split("   ")
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left.push(row[0]);
        right.push(row[1]);
    });

    left.sort();
    right.sort();

    let result = left.into_iter().zip(right.into_iter()).fold(0, |acc, tuple: (i32, i32)| {
        acc + tuple.0.checked_sub(tuple.1).unwrap_or(0).abs()
    });

    println!("\n{result:?}");
}
