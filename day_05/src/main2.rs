use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Eq, PartialEq, Default)]
pub struct Manual {
    next_pages: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Manual {
    pub fn set_next_pages_and_updates(&mut self, manual_string: String) {
        let [raw_rules, raw_updates] = manual_string.split("\n\n").collect::<Vec<&str>>()[..=1]
        else {
            panic!("Invalid input")
        };

        let rule_tuples: Vec<(u32, u32)> = raw_rules
            .split('\n')
            .map(|line| {
                line.split('|')
                    .map(|page| page.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|vec| (vec[0], vec[1]))
            .collect();

        for (predecessor, successor) in rule_tuples {
            self.add(predecessor, successor);
        }

        self.updates = raw_updates
            .split('\n')
            .map(|line| {
                line.split(',')
                    .map(|page| page.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
    }

    pub fn add(&mut self, predecessor: u32, successor: u32) {
        if let Some(predecessor_next_pages) = self.next_pages.get_mut(&predecessor) {
            predecessor_next_pages.insert(successor);
        } else {
            self.next_pages
                .insert(predecessor, HashSet::from([successor]));
        }
    }

    pub fn are_ordered(&self, predecessor: &u32, posible_successor: &u32) -> bool {
        self.next_pages.contains_key(predecessor)
            && self.next_pages[&predecessor].contains(&posible_successor)
    }

    pub fn fix_update(&self, update: Vec<u32>) -> Vec<u32> {
        let update_size = update.len();
        let mut new_update = update.clone();
        (0..update_size - 1).for_each(|i| {
            if !self.are_ordered(&new_update[i], &new_update[i + 1]) {
                new_update.swap(i, i + 1);
            }
        });
        if !(0..update_size - 1).all(|i| self.are_ordered(&new_update[i], &new_update[i + 1])) {
            self.fix_update(new_update)
        } else {
            new_update
        }
    }

    pub fn count_ordered_updates(&self) -> u32 {
        self.updates
            .clone()
            .into_iter()
            .map(|update| {
                let update_size = update.len();
                if (0..update_size - 1).all(|i| self.are_ordered(&update[i], &update[i + 1])) {
                    0
                } else {
                    self.fix_update(update)[update_size / 2]
                }
            })
            .sum()
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");

        std::process::exit(1)
    });

    let manual_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let mut manual = Manual::default();
    manual.set_next_pages_and_updates(manual_string);
    let result = manual.count_ordered_updates();
    println!("\n{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let manual_string =
            fs::read_to_string("test.txt").expect("Should have been able to read the file");

        let mut manual = Manual::default();
        manual.set_next_pages_and_updates(manual_string);
        assert_eq!(manual.count_ordered_updates(), 123);
    }
}
