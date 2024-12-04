use std::fs;

pub struct WordSearch {
    data: Vec<Vec<char>>,
}

impl WordSearch {
    pub fn new(data: &str) -> Self {
        Self {
            data: data
                .split('\n')
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        }
    }

    pub fn get_xmas_position_value(&self, i: usize, j: usize) -> u32 {
        (self.data[i][j] == 'A'
            && ((self.data[i - 1][j - 1] == 'M' && self.data[i + 1][j + 1] == 'S'
                || self.data[i - 1][j - 1] == 'S' && self.data[i + 1][j + 1] == 'M')
                && (self.data[i - 1][j + 1] == 'M' && self.data[i + 1][j - 1] == 'S'
                    || self.data[i - 1][j + 1] == 'S' && self.data[i + 1][j - 1] == 'M')))
            as u32
    }

    pub fn count_all(&self) -> u32 {
        (1..self.data.len() - 1).fold(0, |acc1, i| {
            acc1 + (1..self.data[0].len() - 1)
                .fold(0, |acc2, j| acc2 + self.get_xmas_position_value(i, j))
        })
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let word_search_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let result = WordSearch::new(&word_search_string).count_all();
    println!("\n{result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_all() {
        let word_search = WordSearch::new(".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........");
        assert_eq!(word_search.count_all(), 9);
    }
}
