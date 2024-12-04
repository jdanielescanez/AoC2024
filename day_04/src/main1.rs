use std::fs;

pub struct WordSearch<'a> {
    data: Vec<Vec<char>>,
    word: &'a str,
}

impl<'a> WordSearch<'a> {
    pub fn new(data: &'a str, word: &'a str) -> Self {
        Self {
            data: data
                .split('\n')
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
            word,
        }
    }

    pub fn get_transpose_data(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
        (0..data[0].len())
            .map(|i| {
                data.iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>()
    }

    pub fn horizontal_search(&self, data: Vec<Vec<char>>) -> usize {
        data.iter()
            .map(|line| {
                line.windows(self.word.len())
                    .filter(|&window| {
                        window == self.word.chars().collect::<Vec<char>>()
                            || window == self.word.chars().rev().collect::<Vec<char>>()
                    })
                    .count()
            })
            .sum()
    }

    pub fn vertical_search(&self, data: Vec<Vec<char>>) -> usize {
        let transposed_data = WordSearch::<'a>::get_transpose_data(data);
        self.horizontal_search(transposed_data)
    }

    pub fn diagonal_search(&self, data: Vec<Vec<char>>) -> usize {
        let n = data.len();
        let mut padded_data_principal = data.clone();
        (0..n).for_each(|i| {
            (0..n - i - 1).rev().for_each(|_| {
                padded_data_principal[i].insert(0, '@');
                padded_data_principal[n - i - 1].push('@');
            })
        });

        let mut padded_data_secondary = data.clone();
        (0..n).for_each(|i| {
            (0..n - i - 1).for_each(|_| {
                padded_data_secondary[i].push('@');
                padded_data_secondary[n - i - 1].insert(0, '@');
            })
        });

        self.vertical_search(padded_data_principal) + self.vertical_search(padded_data_secondary)
    }

    pub fn count_all(&self) -> usize {
        self.horizontal_search(self.data.clone())
            + self.vertical_search(self.data.clone())
            + self.diagonal_search(self.data.clone())
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let word_search_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let result = WordSearch::new(&word_search_string, "XMAS").count_all();
    println!("\n{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_square() {
        let result = WordSearch::get_transpose_data(vec![vec!['1', '2'], vec!['3', '4']]);
        assert_eq!(result, vec![vec!['1', '3'], vec!['2', '4']]);
    }

    #[test]
    fn transpose_rectangular() {
        let result = WordSearch::get_transpose_data(vec![vec!['1', '2', '3'], vec!['4', '5', '6']]);
        assert_eq!(result, vec![vec!['1', '4'], vec!['2', '5'], vec!['3', '6']]);
    }

    #[test]
    fn horizontal_search_simple() {
        let word_search = WordSearch::new("_", "XMAS");
        assert_eq!(
            word_search.horizontal_search(vec!["XMAS".chars().collect::<Vec<char>>()]),
            1
        );
    }

    #[test]
    fn horizontal_search_complex() {
        let word_search = WordSearch::new("_", "XMAS");
        assert_eq!(
            word_search.horizontal_search(vec!["XMASSMAXASMSMXMASMSMXXMAS"
                .chars()
                .collect::<Vec<char>>()]),
            3
        );
    }

    #[test]
    fn horizontal_search_reverse() {
        let word_search = WordSearch::new("_", "XMAS");
        assert_eq!(
            word_search.horizontal_search(vec!["SAMX".chars().collect::<Vec<char>>()]),
            1
        );
    }

    #[test]
    fn horizontal_search_overlap_with_reversed() {
        let word_search = WordSearch::new("_", "XMAS");
        assert_eq!(
            word_search.horizontal_search(vec!["XMASAMX".chars().collect::<Vec<char>>()]),
            2
        );
    }

    #[test]
    fn count_all() {
        let word_search = WordSearch::new("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", "XMAS");
        assert_eq!(word_search.count_all(), 18);
    }
}
