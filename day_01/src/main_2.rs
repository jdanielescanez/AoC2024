use std::collections::HashMap;
use std::fs;

fn main() {
    let file_str = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut lines = file_str.split('\n').collect::<Vec<&str>>();
    lines = lines[..lines.len() - 1].to_vec();

    let mut left = vec![];
    let mut right_cnt: HashMap<u32, u32> = HashMap::new();

    lines
        .into_iter()
        .map(|line| {
            line.split("   ")
                .map(|number| number.parse::<u32>().unwrap())
        })
        .for_each(|mut pair| {
            left.push(pair.next().unwrap());
            *right_cnt.entry(pair.next().unwrap()).or_insert(0) += 1;
        });

    let result = left.into_iter().fold(0, |acc, left_i| {
        acc + left_i * right_cnt.get(&left_i).unwrap_or(&0)
    });

    println!("\n{result:?}");
}
