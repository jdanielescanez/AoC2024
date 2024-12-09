use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct File {
    id: usize,
    repetitions: usize,
    spaces: usize,
}

#[derive(PartialEq, Eq)]
struct DiskMap {
    files: Vec<File>,
}

impl DiskMap {
    pub fn new(disk_map_string: &str) -> Self {
        let fixed_input = disk_map_string.to_owned() + "0";
        let enumerated_chars = fixed_input.as_str().chars().enumerate();

        let spaces: Vec<usize> = enumerated_chars
            .clone()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, x)| x.to_digit(10).unwrap() as usize)
            .collect();

        let files = enumerated_chars
            .filter(|(i, _)| i % 2 == 0)
            .enumerate()
            .map(|(id, (_, x))| File {
                id,
                repetitions: x.to_digit(10).unwrap() as usize,
                spaces: spaces[id],
            })
            .collect();

        DiskMap { files }
    }

    pub fn get_fragmented_disk_checksum(&self) -> usize {
        let mut result: Vec<usize> = Vec::new();
        let repeated_files = self
            .files
            .iter()
            .flat_map(|file| vec![file.id; file.repetitions].into_iter())
            .collect::<Vec<usize>>();

        let size = repeated_files.len();
        let mut aux_files = repeated_files.clone();
        let mut rev_free_spaces: Vec<usize> = self.files.iter().map(|x| x.spaces).rev().collect();

        while !aux_files.is_empty() && result.len() < size {
            repeated_files.windows(2).for_each(|file_id_window| {
                let (a, b) = (file_id_window[0], file_id_window[1]);
                result.push(a);
                if a != b && !aux_files.is_empty() {
                    let space_repetition = rev_free_spaces.pop().unwrap();
                    for _ in 0..space_repetition {
                        result.push(aux_files.pop().unwrap());
                    }
                }
            });
        }
        result[..size]
            .into_iter()
            .enumerate()
            .map(|(i, file_id)| file_id * i as usize)
            .sum()
    }

    pub fn get_fragmented_disk_checksum_with_whole_file(&self) -> usize {
        let mut result: Vec<File> = Vec::new();
        let mut unplaced_files: Vec<File> = self.files.clone();

        while unplaced_files.len() > 0 {
            let mut start_file = unplaced_files[0];
            result.push(start_file);
            for j in (result.len()..unplaced_files.len()).rev() {
                let mut candidate = unplaced_files[j];
                dbg!(
                    start_file,
                    candidate,
                    start_file.spaces,
                    candidate.repetitions
                );
                if start_file.spaces >= candidate.repetitions {
                    candidate.spaces = start_file
                        .spaces
                        .checked_sub(candidate.repetitions)
                        .unwrap();
                    let last_index = result.len() - 1;
                    result[last_index].spaces = 0;
                    result.push(candidate);
                    unplaced_files[j - 1].spaces += candidate.repetitions + candidate.spaces;
                    start_file = candidate;
                    unplaced_files.remove(j);
                }
            }
            unplaced_files.remove(0);
            dbg!(&result, &unplaced_files);
        }
        dbg!(result);
        0
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let disk_map_string_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let disk_map = DiskMap::new(&disk_map_string_string);
    let result = disk_map.get_fragmented_disk_checksum();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_example() {
        let disk_map = DiskMap::new("12345");
        assert_eq!(disk_map.get_fragmented_disk_checksum(), 60);
    }

    #[test]
    fn complex_example() {
        let disk_map = DiskMap::new("2333133121414131402");
        assert_eq!(disk_map.get_fragmented_disk_checksum(), 1928);
    }

    #[test]
    fn example_with_whole_file() {
        let disk_map = DiskMap::new("2333133121414131402");
        assert_eq!(
            disk_map.get_fragmented_disk_checksum_with_whole_file(),
            2858
        );
    }
}
