fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let result = std::fs::read_to_string(input_filename)
        .expect("Error reading input file")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|str| str.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            (0..report.len()).any(|i| {
                let (x, y) = report.split_at(i);
                let mut fixed_report = x.to_vec();
                fixed_report.extend(&y[1..]);
                (fixed_report.is_sorted() || fixed_report.iter().rev().is_sorted())
                    && fixed_report
                        .windows(2)
                        .all(|windows| (1..=3).contains(&windows[0].abs_diff(windows[1])))
            })
        })
        .count();

    println!("\n{result:?}");
}
