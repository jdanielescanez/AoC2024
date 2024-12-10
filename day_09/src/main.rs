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

        let mut aux_files: Vec<File> = enumerated_chars
            .filter(|(i, _)| i % 2 == 0)
            .enumerate()
            .map(|(id, (_, x))| File {
                id,
                repetitions: x.to_digit(10).unwrap() as usize,
                spaces: spaces[id],
            })
            .collect();

        let mut files: Vec<File> = vec![];
        let mut i = 0;
        while i < aux_files.len() - 1 {
            let mut flag = false;
            if aux_files[i + 1].repetitions == 0 {
                aux_files[i].spaces += aux_files[i + 1].spaces;
                flag = true;
            }
            files.push(aux_files[i]);
            i += if flag { 2 } else { 1 };
        }
        files.push(*aux_files.last().unwrap());

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
        let mut result: Vec<File> = self.files.clone();

        let mut j = result.len() - 1;
        loop {
            let candidate = result[j];
            for i in 0..j {
                let start_file = result[i];
                if start_file.spaces >= candidate.repetitions {
                    result[i].spaces = 0;
                    result.remove(j);
                    result.insert(i + 1, candidate);
                    result[i + 1].spaces = start_file
                        .spaces
                        .checked_sub(candidate.repetitions)
                        .unwrap();
                    result[j].spaces += candidate.repetitions + candidate.spaces;
                    j += 1;
                    break;
                }
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }
        result
            .into_iter()
            .flat_map(|file| [&vec![file.id; file.repetitions][..], &vec![0; file.spaces]].concat())
            .enumerate()
            .map(|(i, file_id)| i * file_id as usize)
            .sum()
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
    let result_part1 = disk_map.get_fragmented_disk_checksum();
    println!("Result part 1: {}", result_part1);
    let result_part2 = disk_map.get_fragmented_disk_checksum_with_whole_file();
    println!("Result part 2: {}", result_part2);
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
